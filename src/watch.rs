use middleware::HandlebarsEngine;

use notify::{RecommendedWatcher, Error, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

fn _watch (p: String) -> Result<(), Error>{
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));
    try!(watcher.watch(&Path::new(&p)));
    let _ = rx.recv();
    Ok(())
}

pub trait Watchable {
    fn watch(&self);
}

impl Watchable for Arc<HandlebarsEngine> {
    #[allow(while_true)]
    fn watch (&self) {
        let hbs = self.clone();
        thread::spawn(move || {
            println!("watching path: {}", hbs.prefix);
            while true {
                if let Ok(_) = _watch(hbs.prefix.clone()) {
                    println!("things changed");
                    hbs.reload();
                } else {
                    panic!("Failed to watch template directory.");
                }
            }
        });

    }
}
