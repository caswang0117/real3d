use crate::geom::*;
use crate::model::{DrawModel, Model};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Bone {
    pub translation: [f32; 3],
    pub ignored:f32,
    pub rotation: [f32; 4], // a quaternion
}

pub struct Joint {
    children: [u8; 4],
    // In local coordinate frame, binding pose
    translation: Vec3,
    rotation: Quat,
}

pub struct Rig {
    joints: Vec<Joint>,
    ibms: Vec<Mat4>,
    // erk, hack, only used during loading
    nodes_to_joints: std::collections::BTreeMap<usize, u8>, // names:Vec<Option<String>>
}

impl Rig {
    pub fn from_gltf(_g: &gltf::Document, bufs: &[gltf::buffer::Data], skin: gltf::Skin) -> Self {
        let reader = skin.reader(|buffer| Some(&bufs[buffer.index()]));
        use std::collections::BTreeMap;
        let mut nodes_to_joints: BTreeMap<usize, u8> = skin.joints().enumerate().map(|(ji,n)| {
            assert!(ji < 255);
            (n.index(), ji as u8)
        }).collect();
        let joints: Vec<_> = skin
            .joints()
            .enumerate()
            .map(|(ji, n)| {
                assert!(ji < 255);
                nodes_to_joints.insert(n.index(), ji as u8);
                let (translation, rotation) = match n.transform() {
                    gltf::scene::Transform::Matrix { matrix: _m4 } => {
                        // let m4 = Mat4::from(m4);
                        panic!("Oh well, don't support matrices")
                    }
                    gltf::scene::Transform::Decomposed {
                        translation: tr,
                        rotation: rot,
                        scale: _sc,
                    } => (Vec3::from(tr), Quat::from(rot)),
                };
                let mut children = [255, 255, 255, 255];
                for (ci, c) in n.children().enumerate() {
                    assert!(ci < 4);
                    let cidx = nodes_to_joints[&c.index()];
                    children[ci] = cidx;
                }
                Joint {
                    children,
                    translation,
                    rotation,
                }
            })
            .collect();
        Self {
            ibms: reader
                .read_inverse_bind_matrices()
                .map(|ibms| ibms.map(Mat4::from).collect())
                .unwrap_or_else(|| vec![Mat4::identity(); joints.len()]),
            joints,
            nodes_to_joints,
        }
    }
    pub fn reset(&self, bones:&mut [Bone]) {
        for (j,b) in self.joints.iter().zip(bones.iter_mut()) {
            b.translation = j.translation.into();
            b.rotation = j.rotation.into();
        }
    }
}

pub struct Anim {
    trans_targets: Vec<u8>,
    trans_keys: Vec<Vec3>, // for each frame, one translation for each target
    rot_targets: Vec<u8>,
    rot_keys: Vec<Quat>, // for each frame, one rotation for each target
    timings: Vec<f32>,
}

fn transpose_rowcol<T:Copy+cgmath::Zero>(inp: Vec<T>, num_rows: usize) -> Vec<T> {
    if num_rows == 0 { return inp; }
    let num_cols = inp.len() / num_rows;
    let mut outp = vec![T::zero();inp.len()];
    // From https://docs.rs/transpose/0.2.1/src/transpose/out_of_place.rs.html#17-26
    unsafe {
        for y in 0..num_rows {
            for x in 0..num_cols {
                let inp_idx = x + y * num_cols;
                let outp_idx = y + x * num_rows;
                *outp.get_unchecked_mut(outp_idx) = *inp.get_unchecked(inp_idx);
            }
        }
    }
    outp
}

impl Anim {
    pub fn from_gltf(
        _g: &gltf::Document,
        bufs: &[gltf::buffer::Data],
        anim: gltf::Animation,
        rig: &Rig,
    ) -> Self {
        let timings: Vec<_> = anim
            .channels()
            .next()
            .unwrap()
            .reader(|b| Some(&bufs[b.index()]))
            .read_inputs()
            .unwrap()
            .collect();
        let mut trans_targets = vec![];
        let mut trans_keys_by_tgt = vec![];
        let mut rot_targets = vec![];
        let mut rot_keys_by_tgt = vec![];
        for c in anim.channels() {
            let reader = c.reader(|b| Some(&bufs[b.index()]));
            let c_timings: Vec<_> = reader.read_inputs().unwrap().collect();
            assert_eq!(c_timings.len(), timings.len());
            let tgt = c.target().node().index();
            let prop = c.target().property();
            let tgt = rig.nodes_to_joints[&tgt];
            match prop {
                gltf::animation::Property::Translation => trans_targets.push(tgt as u8),
                gltf::animation::Property::Rotation => rot_targets.push(tgt as u8),
                _ => panic!("Unsupported anim target"),
            };
            match reader.read_outputs().unwrap() {
                gltf::animation::util::ReadOutputs::Translations(trs) => {
                    assert_eq!(prop, gltf::animation::Property::Translation);
                    assert_eq!(trs.len(), timings.len());
                    // dbg!("TKEY",trs.len());
                    trans_keys_by_tgt.extend(trs.map(|tr| Vec3::from(tr)));
                }
                gltf::animation::util::ReadOutputs::Rotations(rots) => {
                    assert_eq!(prop, gltf::animation::Property::Rotation);
                    match rots {
                        gltf::animation::util::Rotations::F32(quadruples) => {
                            assert_eq!(quadruples.len(), timings.len());
                            // dbg!("RKEY",quadruples.len());
                            rot_keys_by_tgt.extend(quadruples.map(|q| Quat::from(q)))
                        }
                        _ => panic!("Unsupported rot format"),
                    }
                }
                _ => panic!("Unsupported anim output"),
            }
        }
        let tklen = trans_keys_by_tgt.len();
        let rklen = rot_keys_by_tgt.len();
        let trans_keys = transpose_rowcol(trans_keys_by_tgt, trans_targets.len());
        let rot_keys = transpose_rowcol(rot_keys_by_tgt, rot_targets.len());
        assert_eq!(trans_keys.len(), tklen);
        assert_eq!(rot_keys.len(), rklen);
        Self {
            timings,
            trans_keys,
            rot_keys,
            trans_targets,
            rot_targets,
        }
    }
    pub fn duration(&self) -> f32 {
        *self.timings.last().unwrap_or(&0.0)
    }
    pub fn sample(&self, mut t: f32, rig: &Rig, bones: &mut [Bone]) {
        assert!(self.duration() > 0.0);
        assert!(t >= 0.0);
        // TODO maybe not the best place for this?
        while t >= self.duration() {
            t -= self.duration();
        }
        let t = self.timings.last().unwrap().min(t);
        let kidx = self
            .timings
            .iter()
            .zip(self.timings[1..].iter())
            .position(|(t0, t1)| t >= *t0 && t <= *t1)
            .unwrap();
        let t0 = self.timings[kidx];
        let t1 = self.timings[kidx + 1];
        let tr = (t - t0) / (t1 - t0);
        //let key_count = self.timings.len();
        let ttgt_count = self.trans_targets.len();
        let rtgt_count = self.rot_targets.len();
        // println!("Anim: {} t, {} r, {} keyframes ({}/{})", ttgt_count, rtgt_count, self.timings.len(), self.trans_keys.len(), self.rot_keys.len());
        if ttgt_count > 0 {
            // there are trans_targets trans targets per keyframe
            // there are trans_targets * timings trans_keys total
            let tfrom = &self.trans_keys[(ttgt_count * kidx)..(ttgt_count * (kidx + 1))];
            let tto = &self.trans_keys[(ttgt_count * (kidx + 1))..(ttgt_count * (kidx + 2))];
            for ((tgt, from), to) in self.trans_targets.iter().zip(tfrom.iter()).zip(tto.iter()) {
                let j = &rig.joints[*tgt as usize];
                bones[*tgt as usize].translation = (from.lerp(*to, tr)).into();
            }
        }
        // there are rot_targets rot targets per keyframe
        // there are rot_targets * timings trans_keys total
        if rtgt_count > 0 {
            let rfrom = &self.rot_keys[(rtgt_count * kidx)..(rtgt_count * (kidx + 1))];
            let rto = &self.rot_keys[(rtgt_count * (kidx + 1))..(rtgt_count * (kidx + 2))];
            for ((tgt, from), to) in self.rot_targets.iter().zip(rfrom.iter()).zip(rto.iter()) {
                let j = &rig.joints[*tgt as usize];
                bones[*tgt as usize].rotation = (from.nlerp(*to, tr)).into();
            }
        }

        // right now all bones are set in joint-local terms.
        // we need to go from top to bottom to fix that...
        for (ji, j) in rig.joints.iter().enumerate() {
            let b = bones[ji];
            // transform all direct child bones by this bone's transformation.
            let br = Quat::from(b.rotation);
            let bt = Vec3::from(b.translation);
            let btrans = cgmath::Decomposed{scale:1.0, rot:br, disp:bt};
            for &ci in j.children.iter() {
                if ci == 255 {
                    break;
                }
                let b2 = &mut bones[ci as usize];
                let b2trans = btrans*cgmath::Decomposed{scale:1.0, rot:b2.rotation.into(), disp:b2.translation.into()};
                // augment b2 translation: include rotate-move from b to b2
                b2.translation = b2trans.disp.into();
                b2.rotation = b2trans.rot.into();
            }
            // but then we need to multiply by the inverse bind matrix to
            // turn this bone into a "change in vertex translations"
            let ibm = rig.ibms[ji];
            let post_ibm:Mat4 = Mat4::from(cgmath::Decomposed{scale:1.0_f32,rot:Quat::from(b.rotation),disp:b.translation.into()})*ibm;
            let transl = post_ibm.w.truncate();
            let rotn = Mat3::from_cols(post_ibm.x.truncate(), post_ibm.y.truncate(), post_ibm.z.truncate());
            // warning that might not be right
            let b = &mut bones[ji];
            b.translation = transl.into();
            b.rotation = Quat::from(rotn).into();
        }
    }
}

pub trait DrawAnimated<'a, 'b>
where
    'b: 'a,
{
    fn draw_model_skinned(
        &mut self,
        model: &'b Model,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
        bones: &'b wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawAnimated<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model_skinned(
        &mut self,
        model: &'b Model,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
        bones: &'b wgpu::BindGroup,
    ) {
        self.set_bind_group(3, &bones, &[]);
        self.draw_model_instanced(model, 0..1, uniforms, light);
    }
}

pub struct State {}
