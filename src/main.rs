use rand::{rngs::ThreadRng, Rng};
mod player;

fn main() {
    // trying to generate random number
    let mut rng: ThreadRng = rand::thread_rng();
    let num: i32 = rng.gen();
    let random_number_between_10_and_20: i32 = rng.gen_range(10..20);
    println!("ranodm number: {}", num);
    println!(
        "Random number between 10 and 20 is {}",
        random_number_between_10_and_20
    );
}

// modules are not the same as folders!!!!!!!!!!
mod clean {
    #[allow(dead_code)]
    fn perform_cleanup_private() {
        println!("Cleaning private");
    }
    #[allow(dead_code)]
    pub fn perform_cleanup_public() {
        println!("Cleaning up public");
    }
}
