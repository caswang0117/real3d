pub use cgmath::prelude::*;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Pos3 = cgmath::Point3<f32>;
pub type Vec4 = cgmath::Vector4<f32>;
pub type Mat3 = cgmath::Matrix3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;
pub type Quat = cgmath::Quaternion<f32>;
pub const PI: f32 = std::f32::consts::PI;
use crate::render::InstanceGroups;

pub trait Shape {
    fn translate(&mut self, v: Vec3);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub c: Pos3,
    pub r: f32,
}

impl Shape for Sphere {
    fn translate(&mut self, v: Vec3) {
        self.c += v;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane {
    pub n: Vec3,
    pub d: f32,
}

impl Shape for Plane {
    fn translate(&mut self, _v: Vec3) {
        panic!();
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Box {
    pub c: Pos3,
    pub axes: Mat3,
    pub half_sizes: Vec3,
}

impl Shape for Box {
    fn translate(&mut self, v: Vec3) {
        self.c += v;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AABB {
    pub c: Pos3,
    pub half_sizes: Vec3,
}

impl Shape for AABB {
    fn translate(&mut self, v: Vec3) {
        self.c += v;
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub p: Pos3,
    pub dir: Vec3,
}

impl Shape for Ray {
    fn translate(&mut self, v: Vec3) {
        self.p += v;
    }
}

pub trait Collide<S: Shape>: Shape {
    fn touching(&self, s2: &S) -> bool {
        self.disp(s2).is_some()
    }
    fn disp(&self, s2: &S) -> Option<Vec3>;
}

impl Collide<Sphere> for Sphere {
    fn touching(&self, s2: &Sphere) -> bool {
        // Is the (squared) distance between the centers less than the
        // (squared) sum of the radii?
        s2.c.distance2(self.c) <= (self.r + s2.r).powi(2)
    }
    /// What's the offset I'd need to push s1 and s2 out of each other?
    fn disp(&self, s2: &Sphere) -> Option<Vec3> {
        let offset = s2.c - self.c;
        let distance = offset.magnitude();
        if distance < self.r + s2.r {
            // Make sure we don't divide by 0
            let distance = if distance == 0.0 { 1.0 } else { distance };
            // How much combined radius is "left over"?
            let disp_mag = (self.r + s2.r) - distance;
            // Normalize offset and multiply by the amount to push
            Some(offset * (disp_mag / distance))
        } else {
            None
        }
    }
}

impl Collide<Plane> for Sphere {
    fn touching(&self, p: &Plane) -> bool {
        // Find the distance of the sphere's center to the plane
        (self.c.dot(p.n) - p.d).abs() <= self.r
    }
    fn disp(&self, p: &Plane) -> Option<Vec3> {
        // Find the distance of the sphere's center to the plane
        let dist = self.c.dot(p.n) - p.d;
        if dist.abs() <= self.r {
            // If we offset from the sphere position opposite the normal,
            // we'll end up hitting the plane at `dist` units away.  So
            // the displacement is just the plane's normal * dist.
            Some(p.n * (self.r - dist))
        } else {
            None
        }
    }
}

type CastHit = Option<(Pos3, f32)>;

trait Cast<S: Shape> {
    fn cast(&self, s: &S) -> CastHit;
}

impl Cast<Sphere> for Ray {
    fn cast(&self, s: &Sphere) -> CastHit {
        let m = self.p - s.c;
        let b = self.dir.dot(m);
        let c = m.dot(m) - s.r * s.r;
        let discr = b * b - c;
        if (c > 0.0 && b > 0.0) || discr < 0.0 {
            return None;
        }
        let t = (-b - discr.sqrt()).max(0.0);
        Some((self.p + t * self.dir, t))
    }
}
impl Cast<Plane> for Ray {
    fn cast(&self, b: &Plane) -> CastHit {
        let denom = self.dir.dot(b.n);
        if denom == 0.0 {
            return None;
        }
        let t = (b.d - self.p.dot(b.n)) / denom;
        if t >= 0.0 {
            Some((self.p + self.dir * t, t))
        } else {
            None
        }
    }
}
impl Cast<Box> for Ray {
    fn cast(&self, b: &Box) -> CastHit {
        let mut tmin = 0.0_f32;
        let mut tmax = f32::MAX;
        let delta = b.c - self.p;
        for i in 0..3 {
            let axis = b.axes[i];
            let e = axis.dot(delta);
            let mut f = self.dir.dot(axis);
            if f.abs() < f32::EPSILON {
                if -e - b.half_sizes[i] > 0.0 || -e + b.half_sizes[i] < 0.0 {
                    return None;
                }
                f = f32::EPSILON;
            }
            let mut t1 = (e + b.half_sizes[i]) / f;
            let mut t2 = (e - b.half_sizes[i]) / f;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return None;
            }
        }
        Some((self.p + self.dir * tmin, tmin))
    }
}
impl Cast<AABB> for Ray {
    fn cast(&self, b: &AABB) -> CastHit {
        let mut tmin = 0.0_f32;
        let mut tmax = f32::MAX;
        let min = b.c - b.half_sizes;
        let max = b.c + b.half_sizes;
        for i in 0..3 {
            if self.dir[i].abs() < f32::EPSILON {
                if self.p[i] < min[i] {
                    return None;
                }
                continue;
            }
            let ood = 1.0 / self.dir[i];
            let mut t1 = (min[i] - self.p[i]) * ood;
            let mut t2 = (max[i] - self.p[i]) * ood;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return None;
            }
        }
        Some((self.p + self.dir * tmin, tmin))
    }
}
