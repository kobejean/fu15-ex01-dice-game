use rand::{Rng, rngs::ThreadRng};

fn roll(rng: &mut ThreadRng, die: u32) -> u32 {
    let num = rng.gen_range(1..=6);
    println!("Die {}: {}", die, num);
    return num;
}

fn main() {
    let num_rolls: u32 = 2;
    let mut rng = rand::thread_rng();

    println!("Rolling dice...");
    let mut sum = 0;
    for n in 1..=num_rolls {
        sum += roll(&mut rng, n);
    }
    println!("Total value: {}", sum);

    if sum > 7 {
        println!("You won!");
    } else {
        println!("You lost!");
    }

    
}
