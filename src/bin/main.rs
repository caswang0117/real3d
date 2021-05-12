use kira::{manager::AudioManager, manager::AudioManagerSettings, sound::SoundSettings, Tempo};
use rand;
use real3d::{
    audio::*, camera_control::*, events::*, geom::*, grid::*, lights::Light,
    render::InstanceGroups, run, Engine, serialization::*, network::Server,
};
use std::ops::Add;
use winit;
use serde_json;
use real3d::save::{save, load};


#[derive(Clone, Debug)]
pub struct Blocks {
    pub vec: Vec<Block>,
}

impl Blocks {
    fn new(grid: &Grid, offset: i32) -> Self {
        let mut blocks: Vec<Block> = vec![
            // T
            Block { c: GridCoord::new(-0 + 8 + 2 + offset, 5 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-1 + 8 + 2 + offset, 5 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-2 + 8 + 2 + offset, 5 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-1 + 8 + 2 + offset, 4 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-1 + 8 + 2 + offset, 3 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-1 + 8 + 2 + offset, 2 + 20, 20), color: TetrisColor::Red },
            Block { c: GridCoord::new(-1 + 8 + 2 + offset, 1 + 20, 20), color: TetrisColor::Red },
            // E
            Block { c: GridCoord::new(-0 + 4 + 2 + offset, 5 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-1 + 4 + 2 + offset, 5 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-2 + 4 + 2 + offset, 5 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-0 + 4 + 2 + offset, 4 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-0 + 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-1 + 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-2 + 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-0 + 4 + 2 + offset, 2 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-0 + 4 + 2 + offset, 1 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-1 + 4 + 2 + offset, 1 + 20, 20), color: TetrisColor::Blue },
            Block { c: GridCoord::new(-2 + 4 + 2 + offset, 1 + 20, 20), color: TetrisColor::Blue },
            // T
            Block { c: GridCoord::new(-0 + 0 + 2 + offset, 5 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-1 + 0 + 2 + offset, 5 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-2 + 0 + 2 + offset, 5 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-1 + 0 + 2 + offset, 4 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-1 + 0 + 2 + offset, 3 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-1 + 0 + 2 + offset, 2 + 20, 20), color: TetrisColor::Yellow },
            Block { c: GridCoord::new(-1 + 0 + 2 + offset, 1 + 20, 20), color: TetrisColor::Yellow },
            // R
            Block { c: GridCoord::new(-1 - 4 + 2 + offset, 5 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-2 - 4 + 2 + offset, 5 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-0 - 4 + 2 + offset, 4 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-3 - 4 + 2 + offset, 4 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-0 - 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-1 - 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-2 - 4 + 2 + offset, 3 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-0 - 4 + 2 + offset, 2 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-2 - 4 + 2 + offset, 2 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-0 - 4 + 2 + offset, 1 + 20, 20), color: TetrisColor::Green },
            Block { c: GridCoord::new(-3 - 4 + 2 + offset, 1 + 20, 20), color: TetrisColor::Green },
            // I
            Block { c: GridCoord::new(-0 - 9 + 2 + offset, 5 + 20, 20), color: TetrisColor::Cyan },
            Block { c: GridCoord::new(-0 - 9 + 2 + offset, 4 + 20, 20), color: TetrisColor::Cyan },
            Block { c: GridCoord::new(-0 - 9 + 2 + offset, 3 + 20, 20), color: TetrisColor::Cyan },
            Block { c: GridCoord::new(-0 - 9 + 2 + offset, 2 + 20, 20), color: TetrisColor::Cyan },
            Block { c: GridCoord::new(-0 - 9 + 2 + offset, 1 + 20, 20), color: TetrisColor::Cyan },
            // S
            Block { c: GridCoord::new(-1 - 11 + 2 + offset, 5 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-2 - 11 + 2 + offset, 5 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-3 - 11 + 2 + offset, 5 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-0 - 11 + 2 + offset, 4 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-1 - 11 + 2 + offset, 3 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-2 - 11 + 2 + offset, 3 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-3 - 11 + 2 + offset, 2 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-0 - 11 + 2 + offset, 1 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-1 - 11 + 2 + offset, 1 + 20, 20), color: TetrisColor::Magenta },
            Block { c: GridCoord::new(-2 - 11 + 2 + offset, 1 + 20, 20), color: TetrisColor::Magenta },
        ];
        for t in grid.tetris.iter() {
            for b in t.blocks.iter() {
                blocks.push(Block {
                    c: b.c + grid.origin,
                    color: b.color,
                });
            }
        }
        Self { vec: blocks }
    }

    fn from_serialized(grid: &SerializableGrid, origin: cgmath::Vector3<i32>) -> Self {
        let mut v = vec![];
        for t in grid.tetris.iter() {
            for b in t.blocks.iter() {
                let mut b = b.to_block();
                b.c += origin;
                v.push(b)
            }
        }

        Self {
            vec: v
        }
    }

    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        for b in self.vec.iter() {
            igs.render(
                match b.color {
                    TetrisColor::Red => rules.tetris_models[0],
                    TetrisColor::Green => rules.tetris_models[1],
                    TetrisColor::Blue => rules.tetris_models[2],
                    TetrisColor::Cyan => rules.tetris_models[3],
                    TetrisColor::Magenta => rules.tetris_models[4],
                    TetrisColor::Yellow => rules.tetris_models[5],
                    _ => rules.tetris_models[6],
                },
                real3d::render::InstanceRaw {
                    model: (Mat4::from_translation(b.c.to_vec().cast::<f32>().unwrap())
                        * Mat4::from_nonuniform_scale(0.5, 0.5, 0.5))
                        .into(),
                },
            )
        }

        // igs.render_batch(
        //     rules.box_model,
        //     self.vec.iter().map(|block| {
        //         let scale = 1.0;
        //
        //     }),
        // );
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Base {
    pub origin: Vec3,
    pub use_other: bool,
}

impl Base {
    fn render(&self, rules: &GameData, igs: &mut InstanceGroups) {
        igs.render(
            if !self.use_other { rules.base_model } else { rules.other_base_model },
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
    other_blocks: Option<Blocks>,
    grid: Grid,
    base: Base,
    other_bases: Vec<Base>,
    light: Light,
    audio: Audio,
    camera_controller: CameraController,
    server: Server,
    multiplayer_init: bool,
    multiplayer_offset: i32,
}

struct GameData {
    base_model: real3d::assets::ModelRef,
    other_base_model: real3d::assets::ModelRef,
    tetris_models: Vec<real3d::assets::ModelRef>,
}

impl Game {
    fn recalc_blocks(&mut self) {
        if self.multiplayer_init {
            self.blocks = Blocks::new(&self.grid, -5);
        } else {
            self.blocks = Blocks::new(&self.grid, self.multiplayer_offset);
        }
    }
}

impl real3d::Game for Game {
    type StaticData = GameData;
    fn start(engine: &mut Engine) -> (Self, Self::StaticData) {
        use rand::Rng;
        let base = Base {
            origin: Vec3::new(0.0, 0.0, 0.0),
            use_other: false,
        };

        let base_model = engine.load_model("floor.obj");
        let other_base_model = engine.load_model("other_floor.obj");

        let tetris_models = vec![
            // RGB CMY and kinda K
            engine.load_model("block-red.obj"),
            engine.load_model("block-green.obj"),
            engine.load_model("block-blue.obj"),
            engine.load_model("block-cyan.obj"),
            engine.load_model("block-magenta.obj"),
            engine.load_model("block-yellow.obj"),
            engine.load_model("block.obj"),
        ];

        engine.set_ambient(1.0);
        // let mut grid = Grid::new(cgmath::Vector3::<i32>::new(-4, 1, -3));
        let mut grid = load(
            "tetris_save.json",
            cgmath::Vector3::<i32>::new(-4, 1, -3),
        );

        let blocks = Blocks::new(&grid, 0);

        let light = Light::point(Pos3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

        let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
        let gameplay = audio_manager
            .load_sound(
                "content/Tetris 99 - Main Theme.mp3",
                SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
            )
            .unwrap();
        let clear = audio_manager
            .load_sound(
                "content/Clear Sound.mp3",
                SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
            )
            .unwrap();
        let sound_handles = vec![gameplay, clear];
        let audio = Audio::new(audio_manager, sound_handles);
        let mut server = Server::new();
        server.connect("45.10.152.68:16512");
        let camera_controller = CameraController::new(1.0);
        (
            Self {
                camera_controller,
                blocks,
                grid,
                base,
                audio,
                light,
                server,
                other_blocks: None,
                other_bases: vec![],
                multiplayer_init: false,
                multiplayer_offset: 0,
            },
            GameData {
                base_model,
                other_base_model,
                tetris_models,
            },
        )
    }
    fn render(
        &mut self,
        rules: &Self::StaticData,
        assets: &real3d::assets::Assets,
        igs: &mut InstanceGroups,
    ) {
        self.base.render(rules, igs);
        self.blocks.render(rules, igs);
        if let Some(o) = &self.other_blocks {
            o.render(rules, igs);
        }
        for b in self.other_bases.iter() {
            b.render(rules, igs);
        }
    }

    fn update(&mut self, _rules: &Self::StaticData, engine: &mut Engine) {
        self.camera_controller.update(engine);
        // background audio
        self.audio
            .play(SoundID(0), true, Some(0.0), AlreadyPlayingAction::Nothing);
        let curr = self.grid.current;

        // when current piece lands, check to clear plane and spawn new piece
        if !self.grid.tetris[curr].falling && !self.grid.end {
            // check if plane needs to be cleared
            let planes = self.grid.check_planes();
            // println!("planes: {:?}", planes);
            if !planes.is_empty() {
                for p in planes {
                    self.grid.clear_plane(p);
                    self.recalc_blocks();
                    self.audio
                        .play(SoundID(1), true, Some(0.0), AlreadyPlayingAction::Nothing);
                }
            } else if self.grid.tetris.len() % 15 == 0 {
                self.grid.clear_plane(2);
                self.recalc_blocks();
                self.audio
                    .play(SoundID(1), false, Some(0.0), AlreadyPlayingAction::Nothing);
            }
            // spawn new piece
            self.grid.add_tetris();
            self.recalc_blocks();
        }

        if self.grid.tetris[curr].falling && engine.frame % 30 == 0 {
            self.grid.lower_tetris(curr);
            self.recalc_blocks();
        }

        if engine.events.key_pressed(KeyCode::D) {
            self.grid.move_xz(curr, 0);
            self.recalc_blocks();
        } else if engine.events.key_pressed(KeyCode::A) {
            self.grid.move_xz(curr, 1);
            self.recalc_blocks();
        } else if engine.events.key_pressed(KeyCode::W) {
            self.grid.move_xz(curr, 2);
            self.recalc_blocks();
        } else if engine.events.key_pressed(KeyCode::S) {
            self.grid.move_xz(curr, 3);
            self.recalc_blocks();
        } else if engine.events.key_held(KeyCode::Down) {
            self.grid.lower_tetris(curr);
            self.recalc_blocks();
        } else if engine.events.key_pressed(KeyCode::Return) {
            save(&self.grid, "tetris_save.json");
            println!("Game saved");
        } else if engine.events.key_pressed(KeyCode::N) {
            println!("Game restarted");
            self.grid = Grid::new(cgmath::Vector3::<i32>::new(-4, 1, -3));
        }
        let other = self.server.update_grid(&self.grid);
        if other.len() > 0 {
            self.other_blocks = Some(Blocks::from_serialized(&other[0], cgmath::Vector3::<i32>::new(-15, 1, -3)));
            self.other_bases = vec![Base { origin: Vec3::new(-11.0, 0.0, 0.0), use_other: true }];
            if !self.multiplayer_init && engine.camera_mut().eye.x >= -5.0 {
                engine.camera_mut().eye.x -= 1.0;
                engine.camera_mut().target.x -= 1.0;
                // engine.camera_mut().eye.z -= 0.1;
                self.multiplayer_offset -= 1;
                self.recalc_blocks();
            } else {
                self.multiplayer_init = true
            }
        }
        let light_pos = self.light.position();
        self.light = Light::point(light_pos, self.light.color());
        engine.set_lights(vec![self.light]);
    }
}

fn main() {
    env_logger::init();
    let window = winit::window::WindowBuilder::new().with_title("Tetris 3D");
    run::<GameData, Game>(window, std::path::Path::new("content"));
}
