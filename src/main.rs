use cracklepop::{matching_noises_or_number, Crackle, Noise, Pop};

fn main() {
    let noises: Vec<Box<dyn Noise>> = vec![Box::new(Crackle), Box::new(Pop)];
    for n in 1..=100 {
        let noises_or_number = matching_noises_or_number(&noises, n);
        println!("{}", noises_or_number)
    }
}
