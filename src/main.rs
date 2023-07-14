use std::{
    io::stdin,
    time::Instant,
};

use clap::Parser;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    times: u8,
    #[arg(long, short)]
    peoples: u32,
    #[arg(long, short)]
    second: u32,
}

fn main() {
    let cli = Args::parse();
    let times = cli.times;
    let peoples = cli.peoples;
    let second = cli.second;

    let mut timer = Instant::now();
    let mut rng = rand::thread_rng();

    let mut count = 0;

    let mut peoples_vec = (1..=peoples).collect::<Vec<_>>();

    loop {
        let pb = ProgressBar::new_spinner();
        // pb.enable_steady_tick(Duration::from_millis(1000));
        if times == count {
            break;
        }
        let mut lucky = None;
        loop {
            if timer.elapsed().as_secs_f32() >= second as f32 {
                pb.finish();
                println!("Input Enter to continue ...");
                let mut buffer = String::new();
                loop {
                    let stdin = stdin().read_line(&mut buffer);
                    if stdin.is_ok() {
                        let index = peoples_vec
                            .iter()
                            .position(|x| x == lucky.unwrap())
                            .unwrap();
                        peoples_vec.remove(index);
                        timer = Instant::now();
                        pb.finish_and_clear();
                        break;
                    }
                }
                break;
            }
            lucky = peoples_vec.choose(&mut rng);
            if let Some(lucky) = lucky {
                pb.set_message(format!("Lucky number: {lucky}"));
            }
        }

        count += 1;
    }

    drop(count);
}
