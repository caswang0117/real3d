use kira::arrangement::Arrangement;
use kira::arrangement::LoopArrangementSettings;
use kira::instance::handle::InstanceHandle;
use kira::instance::InstanceSettings;
use kira::instance::InstanceState;
use kira::instance::StopInstanceSettings;
use kira::manager::AudioManager;
use kira::parameter::tween::Tween;
use kira::sound::handle::SoundHandle;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct SoundID(pub usize);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AlreadyPlayingAction {
    Play,
    Retrigger,
    CancelWithFade(f64), // f64: duration of the fade
    CancelWithoutFade,
    Nothing,
}

pub struct Audio {
    pub manager: AudioManager,
    pub sound_handles: Vec<SoundHandle>,
    pub instance_handles: Vec<Vec<InstanceHandle>>,
}

impl Audio {
    pub fn new(manager: AudioManager, sound_handles: Vec<SoundHandle>) -> Self {
        let num_sounds = sound_handles.len();
        let mut instance_handles = vec![];
        for _i in 0..num_sounds {
            instance_handles.push(vec![]);
        }
        Self {
            manager,
            sound_handles,
            instance_handles,
        }
    }

    pub fn play(
        &mut self,
        id: SoundID,
        loops: bool,
        loop_start: Option<f64>,
        action: AlreadyPlayingAction,
    ) {
        self.remove_stopped_instances();
        let settings = InstanceSettings::default();
        if self.instance_handles[id.0].is_empty() {
            // if sound is not currently playing, play it and add the instance to self
            let instance_handle = if !loops {
                self.sound_handles[id.0].play(settings)
            } else {
                let sound = &self.sound_handles[id.0];
                let arrangement = Arrangement::new_loop(sound, LoopArrangementSettings::default());
                let mut arrangement_handle = self.manager.add_arrangement(arrangement).unwrap();
                arrangement_handle.play(InstanceSettings::default())
            };
            if let Ok(instance_handle) = instance_handle {
                self.instance_handles[id.0].push(instance_handle);
            }
        } else {
            // if the sound is currently playing, handle appropriately
            match action {
                AlreadyPlayingAction::Play => {
                    // play sound and add the instance to self
                    if let Ok(instance_handle) = self.sound_handles[id.0].play(settings) {
                        self.instance_handles[id.0].push(instance_handle);
                    }
                }
                AlreadyPlayingAction::Retrigger => {
                    self.stop(id, None);
                    self.play(id, loops, loop_start, action);
                }
                AlreadyPlayingAction::CancelWithFade(duration) => {
                    let tween = Tween::linear(duration);
                    self.stop(id, Some(tween));
                }
                AlreadyPlayingAction::CancelWithoutFade => self.stop(id, None),
                AlreadyPlayingAction::Nothing => {}
            }
        }
    }

    pub fn stop(&mut self, id: SoundID, fade_tween: Option<Tween>) {
        let settings = StopInstanceSettings::default();
        if let Some(tween) = fade_tween {
            settings.fade_tween(tween);
        }
        // stop sound and remove all instances
        let stop = self.sound_handles[id.0].stop(settings);
        if self.sound_handles[id.0].stop(settings).is_ok() {
            self.instance_handles[id.0] = vec![];
        };
    }

    pub fn remove_stopped_instances(&mut self) {
        let mut to_remove = vec![];
        // get idxs of stopped instances
        for (i, sound) in self.instance_handles.iter().enumerate() {
            for (j, handle) in sound.iter().enumerate() {
                if handle.state() == InstanceState::Stopped {
                    to_remove.push((i, j));
                }
            }
        }
        // remove instances
        for (i, j) in to_remove {
            self.instance_handles[i].remove(j);
        }
    }
}
