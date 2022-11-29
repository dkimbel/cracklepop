pub struct Noise<'a> {
    pub name: &'a str,
    pub matches: fn(i32) -> bool,
}

pub const CRACKLE: Noise = Noise {
    name: "Crackle",
    matches: |n| n % 3 == 0,
};

pub const POP: Noise = Noise {
    name: "Pop",
    matches: |n| n % 5 == 0,
};

pub fn matching_noises_or_number(noises: &[Noise], n: i32) -> String {
    // example combined_noises values: "CracklePop", "Crackle", ""
    let combined_noises: String = noises
        .iter()
        // filter out any of our 'noises' that don't match the current number
        .filter(|noise| (noise.matches)(n))
        // for noises that DO match, convert them to their string name
        .map(|noise| noise.name)
        // combine all the string names/noises we found into one
        .collect::<String>();

    if !combined_noises.is_empty() {
        combined_noises
    } else {
        // if no noises at all matched this number, just convert that num to string
        n.to_string()
    }
}
