use cracklepop::{matching_noises_or_number, Crackle, Noise, Pop};

fn main() {
    let noises: Vec<Box<dyn Noise>> = vec![Box::new(Crackle), Box::new(Pop)];
    for n in 1..=100 {
        let noises_or_number = matching_noises_or_number(&noises, n);
        println!("{}", noises_or_number)
    }
}

// TODO
//   - Update README with brief description and instructions for installing Rust, then doing `cargo run`
//   - In README, link to Rust book's simple fizzbuzz implementation
//   - In README, explain this solution's extensibility
//   - Squash git history
