use futures::channel::mpsc::{Receiver, channel};
use notify::{Config, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

use crate::listener::ListenerError;

pub struct Listener {
    watcher: RecommendedWatcher,
    rx: Receiver<Result<Event, Error>>,
}

fn is_rom(path: &Path, extensions: &[String]) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| extensions.iter().any(|e| e == ext))
}

impl Listener {
    pub fn new<P: AsRef<Path>>(path: P, extensions: &[&str]) -> Result<Self, ListenerError> {
        let (mut tx, rx) = channel(1);

        let extensions: Vec<String> = extensions.iter().map(|s| s.to_string()).collect();
        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                let Ok(ev) = &res else { return };
                if matches!(ev.kind, EventKind::Create(_) | EventKind::Modify(_))
                    && ev.paths.iter().any(|p| is_rom(p, &extensions))
                {
                    let _ = tx.try_send(res);
                }
            },
            Config::default(),
        )?;
        watcher.watch(path.as_ref(), RecursiveMode::Recursive);

        Ok(Self { watcher, rx })
    }

    pub async fn run(mut self, mut on_rom: impl FnMut(PathBuf)) -> Result<(), ListenerError> {
        while let Ok(Ok(ev)) = self.rx.recv().await {
            on_rom(ev.paths.first().cloned().unwrap_or_default());
        }

        Ok(())
    }
}
