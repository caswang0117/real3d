use crate::geom::*;

#[derive(Debug, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Light {
    pub pos: [f32; 4],
    // pub dir:[f32;4],
    pub color: [f32; 4],
}
impl Light {
    pub fn point(pos: Pos3, color: Vec3) -> Self {
        Self {
            pos: [pos.x, pos.y, pos.z, 1.0],
            // dir:[0.0,0.0,0.0,0.0],
            color: [color.x, color.y, color.z, 0.0],
        }
    }
    // pub fn directed(dir:Vec3, color:Vec3) -> Self {
    //     Self {
    //         pos:[0.0,0.0,0.0,0.0],
    //         dir:[dir.x,dir.y,dir.z,1.0],
    //         color:[color.x,color.y,color.z],
    //     }
    // }
    // pub fn spot(pos:Pos3, dir:Vec3, color:Vec3) -> Self {
    //     Self {
    //         pos:[pos.x,pos.y,pos.z,1.0],
    //         dir:[dir.x,dir.y,dir.z,1.0],
    //         color:[color.x,color.y,color.z],
    //     }
    // }

    pub fn position(&self) -> Pos3 {
        Pos3::new(self.pos[0], self.pos[1], self.pos[2])
    }
    pub fn color(&self) -> Vec3 {
        Vec3::new(self.color[0], self.color[1], self.color[2])
    }
}
