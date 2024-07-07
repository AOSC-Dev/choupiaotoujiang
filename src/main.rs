use std::{
    fs,
    io::{self, stdin, BufReader},
    path::PathBuf,
    time::Instant,
};

use clap::Parser;
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use sha2::{Digest, Sha512};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum App {
    /// Seed from a file
    Seed {
        file: PathBuf,
        #[arg(long, short, value_parser = clap::value_parser!(u32).range(2..))]
        peoples: u32,
    },
    /// Generate random number
    Random {
        #[arg(long, short, value_parser = clap::value_parser!(u8).range(1..))]
        times: u8,
        #[arg(long, short, value_parser = clap::value_parser!(u32).range(2..))]
        peoples: u32,
        #[arg(long, short, value_parser = clap::value_parser!(u32).range(1..))]
        secs: u32,
    },
}

fn main() {
    let app = App::parse();

    match app {
        App::Seed { file, peoples } => {
            let num = seed_from_file(peoples, file);
            println!("{num}");
        }
        App::Random {
            times,
            peoples,
            secs,
        } => {
            random(times, peoples, secs);
        }
    }
}

fn random(times: u8, peoples: u32, secs: u32) {
    let mut rng = thread_rng();
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

            index = Some(rng.gen_range(0..=peoples_vec.len() - 1));
            lucky = peoples_vec[index.unwrap()];
            pb.set_message(format!("Lucky number: {lucky}"));
        }

        count += 1;
    }
}

fn seed_from_file(peoples: u32, file: PathBuf) -> u32 {
    let f = fs::File::open(file).unwrap();
    let mut reader = BufReader::new(f);
    let mut sha512 = Sha512::new();
    io::copy(&mut reader, &mut sha512).unwrap();
    let v = sha512.finalize();
    let mut rng: Pcg64 = Seeder::from(&v).make_rng();

    rng.gen_range(1..=peoples)
}
