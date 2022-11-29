use cracklepop::{matching_noises_or_number, CRACKLE, POP};

fn main() {
    let noises = [CRACKLE, POP];
    for n in 1..=100 {
        let noises_or_number = matching_noises_or_number(&noises, n);
        println!("{}", noises_or_number)
    }
}
