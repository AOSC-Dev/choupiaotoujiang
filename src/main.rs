use std::{
    fs,
    io::{self, stdin, BufReader},
    path::PathBuf,
    time::Instant,
};

use clap::Parser;
use indicatif::ProgressBar;
use sha2::{Digest, Sha512};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum App {
    Seed {
        file: PathBuf,
        #[arg(long, short, value_parser = clap::value_parser!(u32).range(2..))]
        peoples: u32,
    },
    Random(Args),
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, value_parser = clap::value_parser!(u8).range(1..))]
    times: u8,
    #[arg(long, short, value_parser = clap::value_parser!(u32).range(2..))]
    peoples: u32,
    #[arg(long, short, value_parser = clap::value_parser!(u32).range(1..))]
    secs: u32,
}

fn main() {
    let app = App::parse();

    match app {
        App::Seed { file, peoples } => {
            let num = seed_from_file(peoples, file);
            println!("{num}");
        }
        App::Random(args) => {
            random(args);
        }
    }
}

fn random(args: Args) {
    let Args {
        times,
        peoples,
        secs,
    } = args;
    let mut timer = Instant::now();

    let mut count = 0;
    let mut peoples_vec = (1..=peoples).collect::<Vec<_>>();

    loop {
        let pb = ProgressBar::new_spinner();
        if times == count {
            break;
        }
        let mut lucky: u32;
        let mut index: Option<usize> = None;
        loop {
            if timer.elapsed().as_secs_f32() >= secs as f32 {
                pb.finish();
                println!("Input Enter to continue ...");
                let mut buffer = String::new();
                loop {
                    let stdin = stdin().read_line(&mut buffer);
                    if stdin.is_ok() {
                        peoples_vec.remove(index.unwrap());
                        timer = Instant::now();
                        pb.finish_and_clear();
                        break;
                    }
                }
                break;
            }

            index = Some(fastrand::usize(..peoples_vec.len()));
            lucky = peoples_vec[index.unwrap()];
            pb.set_message(format!("Lucky number: {lucky}"));
        }

        count += 1;
    }
}

fn seed_from_file(peoples: u32, file: PathBuf) -> usize {
    let f = fs::File::open(file).unwrap();
    let mut reader = BufReader::new(f);
    let mut sha512 = Sha512::new();
    io::copy(&mut reader, &mut sha512).unwrap();
    let v = sha512.finalize();

    let num: u64 = v.into_iter().map(|x| x as u64).sum();
    fastrand::seed(num);

    fastrand::usize(1..peoples as usize)
}
