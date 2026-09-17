use futures::channel::mpsc::{Receiver, channel};
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
    event::{ModifyKind, RenameMode},
};
use std::path::{Path, PathBuf};

use crate::listener::ListenerError;

pub struct Listener {
    watcher: RecommendedWatcher,
    rx: Receiver<PathBuf>,
}

fn is_rom(path: &Path, extensions: &[String]) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| extensions.iter().any(|e| e == ext))
}

impl Listener {
    pub fn new<P: AsRef<Path>>(root_path: P, extensions: &[&str]) -> Result<Self, ListenerError> {
        let (mut tx, rx) = channel(1);
        let extensions: Vec<String> = extensions.iter().map(|s| s.to_string()).collect();

        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                let Ok(ev) = &res else { return };

                let is_eligible = matches!(
                    ev.kind,
                    EventKind::Create(_)
                        | EventKind::Modify(
                            ModifyKind::Data(_) | ModifyKind::Metadata(_) | ModifyKind::Other
                        )
                        | EventKind::Modify(ModifyKind::Name(RenameMode::To | RenameMode::Both))
                );

                if is_eligible {
                    let is_rename = matches!(ev.kind, EventKind::Modify(ModifyKind::Name(_)));
                    let filtered_path = if is_rename {
                        ev.paths.last()
                    } else {
                        ev.paths.first()
                    };
                    if let Some(path) = filtered_path.filter(|p| is_rom(p, &extensions)) {
                        let _ = tx.try_send(path.clone());
                    }
                }
            },
            Config::default(),
        )?;
        let _ = watcher.watch(root_path.as_ref(), RecursiveMode::Recursive);

        Ok(Self { watcher, rx })
    }

    pub async fn run(mut self, mut on_rom: impl FnMut(PathBuf)) -> Result<(), ListenerError> {
        while let Ok(path) = self.rx.recv().await {
            on_rom(path);
        }

        Ok(())
    }
}
