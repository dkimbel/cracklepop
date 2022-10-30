pub trait Noise {
    fn matches(&self, n: i32) -> bool;
    // it'd be more idiomatic to make Noise implement Rust's builtin Display trait, but that would
    // add more boilerplate and ultimately be a bit harder to follow
    fn to_str(&self) -> &str;
}

pub struct Crackle;
impl Noise for Crackle {
    fn matches(&self, n: i32) -> bool {
        n % 3 == 0
    }
    fn to_str(&self) -> &str {
        "Crackle"
    }
}

pub struct Pop;
impl Noise for Pop {
    fn matches(&self, n: i32) -> bool {
        n % 5 == 0
    }
    fn to_str(&self) -> &str {
        "Pop"
    }
}

pub fn matching_noises_or_number(noises: &[Box<dyn Noise>], n: i32) -> String {
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
