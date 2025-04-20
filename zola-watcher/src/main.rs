use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{env, path::PathBuf, thread};

fn run_zola_command(dir: &PathBuf, args: &[&str]) -> bool {
    let status = Command::new("zola")
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("`zola {:?}` failed with exit code: {}", args, status);
            false
        }
        Err(e) => {
            eprintln!("Failed to run `zola {:?}`: {}", args, e);
            false
        }
    }
}

fn main() -> notify::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <directory-to-watch>", args[0]);
        std::process::exit(1);
    }
    let watch_path = PathBuf::from(&args[1]);
    println!("Watching directory: {:?}", watch_path);

    let (sender, receiver) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        sender,
        notify::Config::default()
            .with_poll_interval(Duration::from_secs(1))
            .with_compare_contents(true),
    )?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    loop {
        match receiver.recv() {
            Ok(event) => {
                if matches!(
                    event.unwrap().kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                ) {
                    _ = clear_screen::clear();
                    println!("Change detected, checking...");

                    thread::sleep(Duration::from_millis(500));

                    if run_zola_command(&watch_path, &["check"]) {
                        println!("✔ check passed, running build...");
                        run_zola_command(&watch_path, &["build"]);
                    }
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}
