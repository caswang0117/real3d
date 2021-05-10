use real3d::{events::*, grid::*, geom::*, lights::Light, render::InstanceGroups, run, Engine, DT, camera_control::*};
use rand;
use winit;
// mod camera_control;
// use camera_control::CameraController;

const NUM_SPHERES: usize = 50;
const NUM_BOXES: usize = 50;

#[derive(Clone, Debug)]
pub struct Marbles {
    pub body: Vec<Sphere>,
}

impl Marbles {
    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render_batch(
            rules.marble_model,
            self.body.iter().map(|body| real3d::render::InstanceRaw {
                model: (Mat4::from_translation(body.c.to_vec()) * Mat4::from_scale(body.r)).into(),
            }),
        );
    }
}
#[derive(Clone, Debug)]
pub struct Boxes {
    pub body: Vec<AABB>,
}

impl Boxes {
    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render_batch(
            rules.box_model,
            self.body.iter().map(|body| {
                let scale = body.half_sizes * 2.0;
                real3d::render::InstanceRaw {
                    model: (Mat4::from_translation(body.c.to_vec())
                        * Mat4::from_nonuniform_scale(scale.x, scale.y, scale.y))
                        .into(),
                }
            }),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Wall {
    pub body: Plane,
}

impl Wall {
    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render(
            rules.wall_model,
            real3d::render::InstanceRaw {
                model: (Mat4::from(cgmath::Quaternion::between_vectors(
                    Vec3::new(0.0, 1.0, 0.0),
                    self.body.n,
                )) * Mat4::from_translation(Vec3::new(0.0, -0.025, 0.0))
                    * Mat4::from_nonuniform_scale(0.5, 0.05, 0.5))
                    .into(),
            },
        );
    }
}

struct Game {
    marbles: Marbles,
    boxes: Boxes,
    wall: Wall,
    light: Light,
    camera_controller: CameraController,
}
struct GameData {
    marble_model: real3d::assets::ModelRef,
    box_model: real3d::assets::ModelRef,
    wall_model: real3d::assets::ModelRef,
}

impl real3d::Game for Game {
    type StaticData = GameData;
    fn start(engine: &mut Engine) -> (Self, Self::StaticData) {
        use rand::Rng;
        let wall = Wall {
            body: Plane {
                n: Vec3::new(0.0, 1.0, 0.0),
                d: 0.0,
            },
        };
        let mut rng = rand::thread_rng();
        let marbles = Marbles {
            body: (0..NUM_SPHERES)
                .map(|_x| {
                    let x = rng.gen_range(-20.0..20.0);
                    let y = rng.gen_range(1.0..20.0);
                    let z = rng.gen_range(-20.0..20.0);
                    let r = rng.gen_range(0.1..1.0);
                    Sphere {
                        c: Pos3::new(x, y, z),
                        r,
                    }
                })
                .collect::<Vec<_>>(),
        };
        let boxes = Boxes {
            body: (0..NUM_BOXES)
                .map(|_x| {
                    let x = rng.gen_range(-20.0..20.0);
                    let y = rng.gen_range(1.0..20.0);
                    let z = rng.gen_range(-20.0..20.0);
                    AABB {
                        c: Pos3::new(x, y, z),
                        half_sizes: Vec3::new(
                            rng.gen_range(0.25..1.0),
                            rng.gen_range(0.25..1.0),
                            rng.gen_range(0.25..1.0),
                        ),
                    }
                })
                .collect::<Vec<_>>(),
        };
        let wall_model = engine.load_model("floor.obj");
        let marble_model = engine.load_model("sphere.obj");
        let box_model = engine.load_model("cube.obj");
        engine.set_ambient(0.05);
        let light = Light::point(Pos3::new(10.0, 10.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let camera_controller = CameraController::new(0.2);
        (
            Self {
                camera_controller,
                marbles,
                boxes,
                wall,
                light,
            },
            GameData {
                wall_model,
                marble_model,
                box_model,
            },
        )
    }
    fn render(&mut self, rules: &Self::StaticData, assets: &real3d::assets::Assets, igs: &mut InstanceGroups) {
        self.wall.render(rules, igs);
        self.boxes.render(rules, igs);
        self.marbles.render(rules, igs);
        // TODO: draw a light model with a shader that just renders it in its solid color?
    }
    fn update(&mut self, _rules: &Self::StaticData, engine: &mut Engine) {
        self.camera_controller.update(engine);
        // a, d rotate the point light
        let light_pos = self.light.position();
        let light_pos = if engine.events.key_held(KeyCode::A) {
            Quat::from(cgmath::Euler::new(
                cgmath::Deg(0.0),
                cgmath::Deg(-90.0 * DT),
                cgmath::Deg(0.0),
            ))
                .rotate_point(light_pos)
        } else if engine.events.key_held(KeyCode::D) {
            Quat::from(cgmath::Euler::new(
                cgmath::Deg(0.0),
                cgmath::Deg(90.0 * DT),
                cgmath::Deg(0.0),
            ))
                .rotate_point(light_pos)
        } else {
            light_pos
        };
        self.light = Light::point(light_pos, self.light.color());
        engine.set_lights(vec![self.light]);
    }
}

fn main() {
    env_logger::init();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new().with_title(title);
    run::<GameData, Game>(window, std::path::Path::new("content"));
}
