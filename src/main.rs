extern crate notify;

use notify::{ RecommendedWatcher, Error, Watcher };
use std::thread;
use std::sync::mpsc::channel;
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("rs-auto-sync local_path remote_path");
        return;
    }

    let local_path_string = args[1].clone();
    let remote_path_string = args[2].clone();
    let local_path = Path::new(&local_path_string);

    if !local_path.starts_with("/") {
        println!("plz rewrite local_path as absolute path");
        return;
    }

    // rsync(&local_path_string, &remote_path_string);

    let (tx, rx) = channel();
    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut Watcher) => {
            Watcher.watch(&local_path_string);

            loop {
                match rx.recv() {
                    Ok(event) => {
                        let mut remote_path = remote_path_string.clone();
                        match event.path {
                            Some(ref x) => {
                                let local_path = x.to_string_lossy();
                                remote_path.push_str(&local_path[local_path_string.len()..]);
                                rsync(&local_path, &remote_path);
                            },
                            None => println!("Some error")
                        }
                    },
                    Err(e) => println!("watch error {:?}", e)
                }
            }
        },
        Err(_) => println!("Error")
        }
    }

fn rsync (source :&str, target :&str) {
    println!(">> rsync {} {}", source, target);
    let options = vec![
        "-r",
        "-v",
        "--exclude='.git/",
        "--delete"];
    let output = Command::new("rsync")
        .args(&options)
        .arg(source)
        .arg(target)
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    if output.stdout.len() > 0 {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
