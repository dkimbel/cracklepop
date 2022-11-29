use std::fs::File;
use std::io::{BufRead, BufReader};

use cracklepop::{matching_noises_or_number, Noise, CRACKLE, POP};

#[test]
fn crackle_pop_100() {
    // Assert that all 100 lines of the CracklePop exercise are correct
    let noises = [CRACKLE, POP];
    let expected_outputs_file = File::open("tests/resources/crackle_pop").unwrap();
    let reader = BufReader::new(expected_outputs_file);

    for (line_index, line_result) in reader.lines().enumerate() {
        let expected_noises = line_result.unwrap();
        let line_number = line_index as i32 + 1;
        let actual_noises = matching_noises_or_number(&noises, line_number);
        assert_eq!(expected_noises, actual_noises);
    }
}

#[test]
fn snap_crackle_pop_boom_100() {
    let snap = Noise {
        name: "Snap",
        matches: |n| n % 4 == 0,
    };
    let boom = Noise {
        name: "Boom",
        matches: |n| n % 12 == 0,
    };
    // Assert that all 100 lines of a more elaborate 'SnapCracklePopBoom' exercise
    // would be correct
    let noises = [snap, CRACKLE, POP, boom];
    let expected_outputs_file = File::open("tests/resources/snap_crackle_pop_boom").unwrap();
    let reader = BufReader::new(expected_outputs_file);

    for (line_index, line_result) in reader.lines().enumerate() {
        let expected_noises = line_result.unwrap();
        let line_number = line_index as i32 + 1;
        let actual_noises = matching_noises_or_number(&noises, line_number);
        assert_eq!(expected_noises, actual_noises);
    }
}
