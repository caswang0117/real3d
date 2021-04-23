use crate::model::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, TryRecvError};

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ModelRef(usize);

pub struct Assets {
    asset_root: PathBuf,
    models: HashMap<ModelRef, Model>,
    model_refs: HashMap<PathBuf, ModelRef>,
    rx: Receiver<notify::DebouncedEvent>,
}
impl Assets {
    pub fn new(asset_root: impl AsRef<Path>) -> Self {
        // ... register filesystem watchers with crate notify = "4.0.15":
        use notify::{RecommendedWatcher, RecursiveMode, Watcher};
        use std::time::Duration;
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
        watcher
            .watch(&asset_root, RecursiveMode::Recursive)
            .unwrap();
        // Warning warning, memory leak and loss of ability to unwatch.
        // I don't want to deal with putting the right Watcher implementor
        // into Assets as a trait object or as a generic arugment.
        Box::leak(Box::new(watcher));
        Self {
            asset_root: asset_root.as_ref().to_owned(),
            models: HashMap::new(),
            model_refs: HashMap::new(),
            rx,
        }
    }
    fn update_model(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        p: PathBuf,
    ) {
        let p = p.strip_prefix(std::env::current_dir().unwrap()).unwrap();
        let p = p.strip_prefix(&self.asset_root).unwrap();
        if let Some(mref) = self.model_refs.get(p) {
            self.models
                .insert(*mref, Model::load(device, queue, layout, &p).unwrap());
        };
    }
    pub fn check_events(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) {
        use notify::DebouncedEvent;
        loop {
            match self.rx.try_recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(path)
                    | DebouncedEvent::Write(path)
                    | DebouncedEvent::Create(path) => {
                        match path.extension().map(|s| s.to_str().unwrap()) {
                            Some("obj") => self.update_model(device, queue, layout, path),
                            Some("png") | Some("jpg") | Some("mtl") => self.update_model(
                                device,
                                queue,
                                layout,
                                Path::new(path.file_stem().unwrap().to_str().unwrap())
                                    .with_extension("obj"),
                            ),
                            _ => {}
                        }
                    }
                    _ => {}
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Connection to asset watcher broken!"),
            }
        }
    }
    pub fn load_model(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        model: impl AsRef<Path>,
    ) -> ModelRef {
        let mref = self.model_ref_for(&model);
        let ar = &self.asset_root;
        self.models
            .entry(mref)
            .or_insert_with(|| Model::load(device, queue, layout, ar.join(&model)).unwrap());
        mref
    }
    pub fn model_ref_for(&mut self, p: impl AsRef<Path>) -> ModelRef {
        let new_ref = ModelRef(self.model_refs.len());
        *self.model_refs.entry(p.as_ref().into()).or_insert(new_ref)
    }
    pub fn path_for_model_ref(&self, mr: ModelRef) -> &Path {
        self.model_refs
            .iter()
            .find(|(_p, mmr)| **mmr == mr)
            .map(|(p, _)| p)
            .unwrap()
    }
    pub fn get_model(&self, model: ModelRef) -> Option<&Model> {
        self.models.get(&model)
    }
}
