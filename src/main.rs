use std::{io::stdin, time::Instant};

use clap::Parser;
use indicatif::ProgressBar;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, value_parser = clap::value_parser!(u8).range(1..))]
    times: u8,
    #[arg(long, short, value_parser = clap::value_parser!(u32).range(2..))]
    peoples: u32,
    #[arg(long, short, value_parser = clap::value_parser!(u32).range(1..))]
    secs: u32,
}

fn main() {
    let Args {
        times,
        peoples,
        secs,
    } = Args::parse();

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
