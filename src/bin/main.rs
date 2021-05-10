use rand;
use real3d::{
    camera_control::*, events::*, geom::*, grid::*, lights::Light, render::InstanceGroups, run,
    Engine, DT,
};
use std::ops::Add;
use winit;
// mod camera_control;
// use camera_control::CameraController;

#[derive(Clone, Debug)]
pub struct Blocks {
    pub vec: Vec<Block>,
}

impl Blocks {
    fn new(grid: &Grid) -> Self {
        let mut blocks: Vec<Block> = vec![];
        for t in grid.tetris.iter() {
            for b in t.blocks.iter() {
                blocks.push(Block {
                    c: b.c + grid.origin,
                });
            }
        }
        Self { vec: blocks }
    }

    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render_batch(
            rules.box_model,
            self.vec.iter().map(|block| {
                let scale = 1.0;
                real3d::render::InstanceRaw {
                    model: (Mat4::from_translation(block.c.to_vec().cast::<f32>().unwrap())
                        * Mat4::from_nonuniform_scale(0.5, 0.5, 0.5))
                    .into(),
                }
            }),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Base {
    pub origin: Vec3,
}

impl Base {
    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render(
            rules.wall_model,
            real3d::render::InstanceRaw {
                model: (Mat4::from(
                    Mat4::from_translation(self.origin)
                        * Mat4::from_nonuniform_scale(1.0, 1.0, 1.0),
                )
                .into()),
            },
        );
    }
}

struct Game {
    blocks: Blocks,
    grid: Grid,
    wall: Base,
    light: Light,
    camera_controller: CameraController,
}
struct GameData {
    box_model: real3d::assets::ModelRef,
    wall_model: real3d::assets::ModelRef,
}

impl real3d::Game for Game {
    type StaticData = GameData;
    fn start(engine: &mut Engine) -> (Self, Self::StaticData) {
        use rand::Rng;
        let base = Base {
            origin: Vec3::new(0.0, 0.0, 0.0),
        };
        let mut rng = rand::thread_rng();

        let box_model = engine.load_model("block.obj");
        let wall_model = engine.load_model("floor.obj");
        engine.set_ambient(0.5);
        let mut grid = Grid::new(cgmath::Vector3::<i32>::new(-4, 1, -3));
        // for _ in 0..18 {
        //     grid.lower_tetris(0);
        // }
        // grid.tetris[0].blocks.push(Block {
        //     c: GridCoord::new(0, 0, 0),
        // });
        println!("{:#?}", grid.tetris[0]);

        let blocks = Blocks::new(&grid);

        //  let boxes = Boxes {
        //     tetris_block: (0..NUM_BOXES)
        //         .map(|_x| {
        //             let x = rng.gen_range(-20.0..20.0);
        //             let y = rng.gen_range(1.0..20.0);
        //             let z = rng.gen_range(-20.0..20.0);
        //             AABB {
        //                 c: Pos3::new(x, y, z),
        //                 half_sizes: Vec3::new(
        //                     rng.gen_range(0.25..1.0),
        //                     rng.gen_range(0.25..1.0),
        //                     rng.gen_range(0.25..1.0),
        //                 ),
        //             }
        //         })
        //         .collect::<Vec<_>>(),
        // };
        let light = Light::point(Pos3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let camera_controller = CameraController::new(1.0);
        (
            Self {
                camera_controller,
                blocks,
                grid,
                wall: base,
                light,
            },
            GameData {
                wall_model,
                box_model,
            },
        )
    }
    fn render(
        &mut self,
        rules: &Self::StaticData,
        assets: &real3d::assets::Assets,
        igs: &mut InstanceGroups,
    ) {
        self.wall.render(rules, igs);
        self.blocks.render(rules, igs);
        // TODO: draw a light model with a shader that just renders it in its solid color?
    }
    fn update(&mut self, _rules: &Self::StaticData, engine: &mut Engine) {
        self.camera_controller.update(engine);
        let curr = self.grid.current;

        // when current piece lands, check to clear plane and spawn new piece
        if !self.grid.tetris[curr].falling {
            // TODO check if plane needs to be cleared
            self.grid.add_tetris();
            self.blocks = Blocks::new(&self.grid);
        }
        if self.grid.tetris[curr].falling && engine.frame % 60 == 0 {
            self.grid.lower_tetris(curr);
            self.blocks = Blocks::new(&self.grid);
        }

        if engine.events.key_pressed(KeyCode::D) {
            self.grid.move_xz(curr, 0);
        } else if engine.events.key_pressed(KeyCode::A) {
            self.grid.move_xz(curr, 1);
        } else if engine.events.key_pressed(KeyCode::W) {
            self.grid.move_xz(curr, 2);
        } else if engine.events.key_pressed(KeyCode::S) {
            self.grid.move_xz(curr, 3);
        } else if engine.events.key_held(KeyCode::Down) {
            self.grid.lower_tetris(curr);
            self.blocks = Blocks::new(&self.grid);
        }

        // a, d rotate the point light
        let light_pos = self.light.position();
        // let light_pos = if engine.events.key_held(KeyCode::A) {
        //     Quat::from(cgmath::Euler::new(
        //         cgmath::Deg(0.0),
        //         cgmath::Deg(-90.0 * DT),
        //         cgmath::Deg(0.0),
        //     ))
        //     .rotate_point(light_pos)
        // } else if engine.events.key_held(KeyCode::D) {
        //     Quat::from(cgmath::Euler::new(
        //         cgmath::Deg(0.0),
        //         cgmath::Deg(90.0 * DT),
        //         cgmath::Deg(0.0),
        //     ))
        //     .rotate_point(light_pos)
        // } else {
        //     light_pos
        // };
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
