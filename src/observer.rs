use std::path::Path;

use crossbeam_channel::Sender;
use notify::{
    event::{AccessKind, AccessMode, ModifyKind, RemoveKind, RenameMode},
    Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};

use crate::error::ThemeError;

#[allow(dead_code)]
pub struct FileObserver {
    watcher: RecommendedWatcher,
}

impl FileObserver {
    pub fn new(path: &Path, sender: Sender<()>, recursive: bool) -> Result<Self, ThemeError> {
        let mut watcher = notify::recommended_watcher(move |res| handle_event(res, &sender))?;
        let mode = if path.is_dir() && recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        watcher.watch(path, mode)?;
        Ok(Self { watcher })
    }
}

fn handle_event(res: Result<Event, Error>, sender: &Sender<()>) {
    match res {
        Ok(Event {
            kind:
                EventKind::Access(AccessKind::Close(AccessMode::Write))
                | EventKind::Modify(ModifyKind::Name(RenameMode::To))
                | EventKind::Modify(ModifyKind::Name(RenameMode::From))
                | EventKind::Remove(RemoveKind::File)
                | EventKind::Remove(RemoveKind::Folder),
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
