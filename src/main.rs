use ignore::WalkBuilder;
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use std::time::Instant;
use walkdir::WalkDir;

macro_rules! bench {
    ($name:expr,$code:block) => {
        print!("{}: ", $name);
        let mut times = (0..5)
            .map(|_| {
                let start = Instant::now();
                $code;
                let elapsed = start.elapsed();
                print!("{:.2?} ", elapsed);
                io::stdout().flush().expect("stdout");
                elapsed
            })
            .collect::<Vec<Duration>>();

        times.sort();

        println!(
            "[avg best 4: {:.2?}]",
            times.iter().take(4).sum::<Duration>() / 4
        );
    };
}

fn main() -> Result<(), String> {
    let dir = std::env::args_os().nth(1).ok_or_else(|| {
        format!(
            "Usage: {} [path]",
            std::env::args().nth(0).expect("WHAT AM I?!")
        )
    })?;

    bench!("ignore", {
        let mut builder = WalkBuilder::new(&dir);
        builder.standard_filters(false);

        builder.build_parallel().run(|| {
            Box::new(|entry| {
                entry
                    .expect("entry")
                    .metadata()
                    .expect("metadata")
                    .modified()
                    .expect("mtime");
                ignore::WalkState::Continue
            })
        });
    });

    bench!("walkdir", {
        for entry in WalkDir::new(&dir) {
            entry
                .expect("entry")
                .metadata()
                .expect("metadata")
                .modified()
                .expect("mtime");
        }
    });

    Ok(())
}
