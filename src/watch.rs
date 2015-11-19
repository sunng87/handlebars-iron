use middleware::HandlebarsEngine;

use notify::{RecommendedWatcher, Error, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

fn _watch (p: &Path) -> Result<(), Error>{
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));
    try!(watcher.watch(p));
    let _ = rx.recv();
    Ok(())
}

pub trait Watchable {
    fn watch(&self, path: &str);
}

impl Watchable for Arc<HandlebarsEngine> {
    fn watch (&self, path: &str) {
        let hbs = self.clone();
        let watch_path = path.to_owned();
        thread::spawn(move || {
            info!("watching path: {}", watch_path);
            let path = Path::new(&watch_path);
            loop {
                match _watch(&path) {
                    Ok(_) => {
                        info!("Template directory changed");
                        hbs.reload();
                    },
                    Err(e) => {
                        warn!("Failed to watch directory: {:?}", e);
                        panic!();
                    }
                }
            }
        });
    }
}
