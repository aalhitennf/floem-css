use std::path::Path;

use crossbeam_channel::Sender;
use notify::{Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::error::ThemeError;

#[allow(dead_code)]
pub struct FileObserver {
    watcher: RecommendedWatcher,
}

impl FileObserver {
    pub fn new(path: &Path, sender: Sender<()>, recursive: bool) -> Result<Self, ThemeError> {
        let mut watcher = notify::recommended_watcher(move |res| handle_event(res, &sender))?;
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        watcher.watch(path, mode)?;
        Ok(FileObserver { watcher })
    }
}

fn handle_event(res: Result<Event, Error>, sender: &Sender<()>) {
    match res {
        Ok(Event {
            kind: EventKind::Create(_) | EventKind::Modify(_),
            ..
        }) => {
            if let Err(e) = sender.send(()) {
                log::error!("Observer send error: {e:?}");
            }
        }
        Ok(_) => (),
        Err(e) => {
            log::error!("{e}");
        }
    };
}