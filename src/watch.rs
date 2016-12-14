use middleware::HandlebarsEngine;

use notify::{RecommendedWatcher, Error, Watcher, RecursiveMode};
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

fn _watch(p: &Path, hbs: &Arc<HandlebarsEngine>) -> Result<(), Error> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));
    try!(watcher.watch(p, RecursiveMode::Recursive));
    loop {
        let _ = rx.recv();
        info!("Template directory changed");
        if let Err(e) = hbs.reload() {
            error!("Failed to reload directory: {:?}", e);
        }
    }
}

pub trait Watchable {
    fn watch(&self, path: &str);
}

impl Watchable for Arc<HandlebarsEngine> {
    fn watch(&self, path: &str) {
        let hbs = self.clone();
        let watch_path = path.to_owned();
        thread::spawn(move || {
            match _watch(Path::new(&watch_path), &hbs) {
                Ok(_) => (),
                Err(e) => {
                    warn!("Failed to watch directory: {:?}", e);
                    panic!();
                }
            }
        });
    }
}
