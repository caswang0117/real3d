use crate::model::{DrawModel, Model};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Bone {
    position: [f32; 4],
    rotation: [f32; 4], // a quaternion
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
