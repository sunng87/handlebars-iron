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
    fn watch(&self);
}

impl Watchable for Arc<HandlebarsEngine> {
    fn watch (&self) {
        let hbs = self.clone();
        thread::spawn(move || {
            println!("watching path: {}", hbs.prefix);
            let prefix = hbs.prefix.clone();
            let path = Path::new(&prefix);
            loop {
                match _watch(&path) {
                    Ok(_) => {
                        println!("things changed");
                        hbs.reload();
                    },
                    Err(e) => {
                        println!("Failed to watch directory: {:?}", e);
                        panic!();
                    }
                }
            }
        });
    }
}
