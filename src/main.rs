fn main() {
    let noises: Vec<Box<dyn Noise>> = vec![Box::new(Crackle), Box::new(Pop)];
    for n in 1..=100 {
        let noises_or_number = matching_noises_or_number(&noises, n);
        println!("{}", noises_or_number)
    }
}

fn matching_noises_or_number(noises: &[Box<dyn Noise>], n: i32) -> String {
    // example combined_noises values: "CracklePop", "Crackle", ""
    let combined_noises: String = noises
        .iter()
        // filter out any of our 'noises' that don't match the current number
        .filter(|noise| noise.matches(n))
        // for noises that DO match, convert them to their string name
        .map(|noise| noise.to_str())
        // combine all the string names/noises we found into one
        .collect::<String>();

    if !combined_noises.is_empty() {
        combined_noises
    } else {
        // if no noises at all matched this number, just convert that num to string
        n.to_string()
    }
}

trait Noise {
    fn matches(&self, n: i32) -> bool;
    // it'd be more idiomatic to make Noise implement Rust's builtin Display trait, but that would
    // add more boilerplate and ultimately be a bit harder to follow
    fn to_str(&self) -> &str;
}

struct Crackle;
struct Pop;

impl Noise for Crackle {
    fn matches(&self, n: i32) -> bool {
        n % 3 == 0
    }
    fn to_str(&self) -> &str {
        "Crackle"
    }
}

impl Noise for Pop {
    fn matches(&self, n: i32) -> bool {
        n % 5 == 0
    }
    fn to_str(&self) -> &str {
        "Pop"
    }
}

// TODO
//   - Refactor out some kind of main code module? This means having a 'lib' and a 'binary' probably
//   - Add tests
//     - In their own file
//     - Have test read from a file line-by-line and see that my main fn behaves correctly
//   - Add fancy tests for a SnapCracklePopBoing? Matching numbers like 2, 4, or 12
//   - Update README with brief description and instructions for installing Rust, then doing `cargo run`
//   - In README, link to Rust book's simple fizzbuzz implementation
//   - In README, explain this solution's extensibility
//   - Squash git history
