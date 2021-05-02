// use crate::anim::*;
use crate::model::*;
use gltf;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ModelRef(usize);
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct RigRef(usize);
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AnimRef(usize);

pub struct Assets {
    asset_root: PathBuf,
    models: HashMap<ModelRef, Model>,
    // rigs: HashMap<RigRef, Rig>,
    // anims: HashMap<AnimRef, Anim>,
}
impl Assets {
    pub fn new(asset_root: impl AsRef<Path>) -> Self {
        // ... register filesystem watchers with crate notify = "4.0.15":
        Self {
            asset_root: asset_root.as_ref().to_owned(),
            models: HashMap::new(),
            // rigs: HashMap::new(),
            // anims: HashMap::new(),
        }
    }
    pub fn load_model(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        model: impl AsRef<Path>,
    ) -> ModelRef {
        let mref = ModelRef(self.models.len());
        let ar = &self.asset_root;
        self.models.insert(
            mref,
            Model::load(device, queue, layout, ar.join(&model)).unwrap(),
        );
        mref
    }
    pub fn get_model(&self, model: ModelRef) -> Option<&Model> {
        self.models.get(&model)
    }
    pub fn load_gltf(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        gltf_file: impl AsRef<Path>,
    ) -> Vec<ModelRef> {
        dbg!(gltf_file.as_ref());
        let gltf_file = gltf_file.as_ref();
        let gltf_file_path = self.asset_root.join(gltf_file);
        let (g, bufs, images) = gltf::import(gltf_file_path).unwrap();
        let mut models = vec![];
        // let mut rigs = vec![];
        // let mut anims = vec![];
        for mesh in g.meshes() {
            let model = Model::from_gltf(device, queue, layout, &g, &bufs, &images, mesh);
            let mref = ModelRef(self.models.len());
            models.push(mref);
            self.models.insert(mref, model);
        }
        models
    }
    //     let mut active_rig = None;
    //     for skin in g.skins() {
    //         // build the rig out of the joints
    //         let rig = Rig::from_gltf(&g, &bufs, skin);
    //         let rref = RigRef(self.rigs.len());
    //         self.rigs.insert(rref, rig);
    //         rigs.push(rref);
    //         active_rig = Some(rref);
    //     }
    //     for ganim in g.animations() {
    //         // TODO make animations retargetable, use target(rig) to assign targets to joints;
    //         // For now, just use the last rig
    //         // build an animation out of this anim's channels and samplers
    //         let anim = Anim::from_gltf(
    //             &g,
    //             &bufs,
    //             ganim,
    //             self.get_rig(active_rig.unwrap()).as_ref().unwrap(),
    //         );
    //         let aref = AnimRef(self.anims.len());
    //         anims.push(aref);
    //         self.anims.insert(aref, anim);
    //     }
    //     (models, rigs, anims)
    // }
    // pub fn get_rig(&self, rig: RigRef) -> Option<&Rig> {
    //     self.rigs.get(&rig)
    // }
    // pub fn get_anim(&self, anim: AnimRef) -> Option<&Anim> {
    //     self.anims.get(&anim)
    }
