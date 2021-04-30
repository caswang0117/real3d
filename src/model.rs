use crate::geom::*;
use crate::texture;
use anyhow::*;
use std::ops::Range;
use std::path::Path;
use wgpu::util::DeviceExt;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
    bone_ids: u32, // 32 bits, fits into last slot of previous line
    // Not relevant for static geometry, wasteful!
    // But, this means we just need one layout...
    bone_weights: [f32; 4], // 32*4 bits
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 8]>() + mem::size_of::<u32>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    // pub normal_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

fn convert_mag_filter(f: Option<gltf::texture::MagFilter>) -> wgpu::FilterMode {
    match f {
        None => wgpu::FilterMode::default(),
        Some(gltf::texture::MagFilter::Linear) => wgpu::FilterMode::Linear,
        Some(gltf::texture::MagFilter::Nearest) => wgpu::FilterMode::Nearest,
    }
}

fn convert_min_filter(f: Option<gltf::texture::MinFilter>) -> wgpu::FilterMode {
    match f {
        None => wgpu::FilterMode::default(),
        Some(gltf::texture::MinFilter::Linear) => wgpu::FilterMode::Linear,
        Some(gltf::texture::MinFilter::Nearest) => wgpu::FilterMode::Nearest,
        Some(f) => {
            println!("mipmap config loading not supported ({:?}), falling back to nearest", f);
            wgpu::FilterMode::Nearest
        }
    }
}

fn convert_wrap(f: gltf::texture::WrappingMode) -> wgpu::AddressMode {
    match f {
        gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
        gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
        gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn load_obj(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: &Path,
    ) -> Result<Self> {
        let (obj_models, obj_materials) = tobj::load_obj(path, true)?;

        // We're assuming that the texture files are stored with the obj file
        let containing_folder = path.parent().context("Directory has no parent")?;

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.diffuse_texture;
            let diffuse_texture =
                texture::Texture::load(device, queue, containing_folder.join(diffuse_path))?;

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: None,
            });

            materials.push(Material {
                name: mat.name,
                diffuse_texture,
                bind_group,
            });
        }

        let mut meshes = Vec::new();
        for m in obj_models {
            let mut vertices = Vec::new();
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                    bone_ids: 0,
                    bone_weights: [1.0, 0.0, 0.0, 0.0],
                });
            }

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", path)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", path)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsage::INDEX,
            });

            meshes.push(Mesh {
                name: m.name,
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }

    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        model: impl AsRef<Path>,
    ) -> Result<Self> {
        let p = model.as_ref();
        match p.extension().map(|osstr| osstr.to_str().unwrap()) {
            Some("obj") => Self::load_obj(device, queue, layout, p),
            _ => panic!("Unsupported model format {:?}", p),
        }
    }
    pub fn from_gltf(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        g: &gltf::Document,
        bufs: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
        mesh: gltf::Mesh,
    ) -> Self {
        let mut materials: Vec<_> = g
            .materials()
            .map(|mat| {
                let diffuse = mat
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .unwrap()
                    .texture();
                let gltf::image::Data{pixels,format,width,height} = images[diffuse.source().index()].clone();
                let sam = diffuse.sampler();
                use gltf::image::Format;
                use image::DynamicImage as DI;
                let diffuse_texture = texture::Texture::from_image(
                    device,
                    queue,
                    &match format {
                        Format::R8 => DI::ImageLuma8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        Format::R8G8 => DI::ImageLumaA8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        Format::R8G8B8 => DI::ImageRgb8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        Format::R8G8B8A8 => DI::ImageRgba8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        Format::B8G8R8 => DI::ImageBgr8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        Format::B8G8R8A8 => DI::ImageBgra8(
                            image::ImageBuffer::from_raw(
                                width,
                                height,
                                pixels,
                            )
                            .unwrap(),
                        ),
                        // Format::R16 => DI::ImageLuma16(image::ImageBuffer::from_raw(diffuse_image.width, diffuse_image.height, diffuse_image.pixels).unwrap()),
                        // Format::R16G16 => DI::ImageLumaA16(image::ImageBuffer::from_raw(diffuse_image.width, diffuse_image.height, diffuse_image.pixels).unwrap()),
                        // Format::R16G16B16 => DI::ImageRgb16(image::ImageBuffer::from_raw(diffuse_image.width, diffuse_image.height, diffuse_image.pixels).unwrap()),
                        // Format::R16G16B16A16 => DI::ImageRgba16(image::ImageBuffer::from_raw(diffuse_image.width, diffuse_image.height, diffuse_image.pixels).unwrap())
                        _ => panic!("This is just ridiculous"),
                    },
                    diffuse.name(),
                    convert_wrap(sam.wrap_s()),
                    convert_wrap(sam.wrap_t()),
                    wgpu::AddressMode::default(),
                    convert_min_filter(sam.min_filter()),
                    convert_mag_filter(sam.mag_filter()),
                )
                .unwrap();
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                        },
                    ],
                    label: None,
                });
                Material {
                    name: mat.name().unwrap_or("").to_string(),
                    diffuse_texture,
                    bind_group,
                }
            })
            .collect();
        if materials.len() == 0 {
            // TODO if empty use a default material
            materials.push( {
                use image::DynamicImage as DI;
                let diffuse_texture = texture::Texture::from_image(
                    device,
                    queue,
                    &DI::ImageRgb8(image::ImageBuffer::from_pixel(16,16,image::Rgb([255,0,255]))),
                    Some("Default Material"),
                    wgpu::AddressMode::Repeat,
                    wgpu::AddressMode::Repeat,
                    wgpu::AddressMode::default(),
                    wgpu::FilterMode::Nearest,
                    wgpu::FilterMode::Nearest,
                )
                .unwrap();
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                        },
                    ],
                    label: None,
                });
                Material {
                    name: "Default Material".to_string(),
                    diffuse_texture,
                    bind_group,
                }
            })
        }
        let mut meshes = Vec::new();
        for prim in mesh.primitives() {
            let reader = prim.reader(|b| Some(&bufs[b.index()]));
            // positions, normals,tex_coords, weights, joints
            let positions: Vec<_> = reader.read_positions().unwrap().collect();
            // indices
            let indices: Vec<u32> = match reader.read_indices() {
                Some(gltf::mesh::util::ReadIndices::U8(idxs)) => idxs.map(|i| i as u32).collect(),
                Some(gltf::mesh::util::ReadIndices::U16(idxs)) => idxs.map(|i| i as u32).collect(),
                Some(gltf::mesh::util::ReadIndices::U32(idxs)) => idxs.map(|i| i as u32).collect(),
                // TODO be smarter about indices
                None => (0..(positions.len() as u32)).collect()
            };
            let normal: Vec<[f32; 3]> = reader
                .read_normals()
                .map(|nr| nr.collect())
                .unwrap_or_else(|| {
                    // calc normals
                    let mut norms = vec![Vec3::zero(); positions.len()];
                    for triple in indices.chunks_exact(3) {
                        match triple {
                            [a, b, c] => {
                                let a = *a as usize;
                                let b = *b as usize;
                                let c = *c as usize;
                                let va: Vec3 = positions[a].into();
                                let vb: Vec3 = positions[b].into();
                                let vc: Vec3 = positions[c].into();
                                let norm = (vb - va).cross(vc - va);
                                norms[a] += norm;
                                norms[b] += norm;
                                norms[c] += norm;
                            }
                            _ => unreachable!("chunks_exact(3)"),
                        }
                    }
                    norms.into_iter().map(|n| n.normalize().into()).collect()
                });
            // assumption: only one set of each of tex coords, weights, joints
            let tex_coords = match reader.read_tex_coords(0) {
                None => vec![[0.0, 0.0]; positions.len()],
                Some(gltf::mesh::util::ReadTexCoords::F32(tcs)) => tcs.collect(),
                _ => panic!("Unsupported tex coord format"),
            };
            let bone_weights = match reader.read_weights(0) {
                None => vec![[0.0; 4]; positions.len()],
                Some(gltf::mesh::util::ReadWeights::F32(wts)) => wts.collect(),
                _ => panic!("Unsupported weight format"),
            };
            let joints = match reader.read_joints(0) {
                None => vec![[255; 4]; positions.len()],
                Some(gltf::mesh::util::ReadJoints::U8(js)) => js.collect(),
                Some(gltf::mesh::util::ReadJoints::U16(js)) => js
                    .map(|j| {
                        assert!(j[0] <= 255);
                        assert!(j[1] <= 255);
                        assert!(j[2] <= 255);
                        assert!(j[3] <= 255);
                        [j[0] as u8, j[1] as u8, j[2] as u8, j[3] as u8]
                    })
                    .collect(),
            };
            let vertices: Vec<_> = positions
                .into_iter()
                .zip(tex_coords.into_iter())
                .zip(normal.into_iter())
                .zip(joints.into_iter())
                .zip(bone_weights.into_iter())
                .map(|((((p, tc), n), bi), bw)| ModelVertex {
                    position: p,
                    tex_coords: tc,
                    normal: n,
                    bone_ids: {
                        ((bi[0] as u32) << 24) | ((bi[1] as u32) << 16) | ((bi[2] as u32) << 8) | (bi[3] as u32)
                    },
                    bone_weights: bw,
                })
                .collect();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", mesh.name().unwrap_or(""))),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", mesh.name().unwrap_or(""))),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsage::INDEX,
            });

            // make buffers
            meshes.push(Mesh {
                name: mesh.name().unwrap_or("").to_string(),
                material: prim.material().index().unwrap_or(0),
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32,
            })
        }
        Model { materials, meshes }
    }
}

pub trait DrawModel<'a, 'b>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        instances: Range<u32>,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    );

    fn draw_model(
        &mut self,
        model: &'b Model,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    );
    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, uniforms, light);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        instances: Range<u32>,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, &uniforms, &[]);
        self.set_bind_group(2, &light, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(
        &mut self,
        model: &'b Model,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.draw_model_instanced(model, 0..1, uniforms, light);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        uniforms: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), uniforms, light);
        }
    }
}
