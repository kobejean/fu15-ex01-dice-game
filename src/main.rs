use std::io;
use rand::{Rng, rngs::ThreadRng};

fn roll(rng: &mut ThreadRng, die: u32) -> u32 {
    let num = rng.gen_range(1..=6);
    println!("Die {}: {}", die, num);
    return num;
}

fn main() -> io::Result<()> {
    let mut name = String::new();
    let stdin = io::stdin();

    println!("What is your name?");
    let n = stdin.read_line(&mut name)?;
    name = name[0..(n-1)].to_string();

    println!("Hello, {}!", name);

    let num_rolls: u32 = 2;
    let mut rng = rand::thread_rng();

    println!("Rolling dice...");
    let mut sum = 0;
    for n in 1..=num_rolls {
        sum += roll(&mut rng, n);
    }
    println!("Total value: {}", sum);

    if sum > 7 {
        println!("{} won!", name);
    } else {
        println!("{} lost!", name);
    }
    
    Ok(())
}
