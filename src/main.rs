#![allow(dead_code)]

use std::{cmp::min, collections::HashSet, fs::File, io::Read, path::Path};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2);
    // Get desired alignment
    let chars_aligned: usize = args[1].parse().unwrap();
    // Read names from file
    let names = get_names();

    // Find pairs permutations of names
    let mut permuted_names: Vec<(String, String)> = Vec::new();
    for idx in 0..(names.len() - 1) {
        if check_if_permutation(names[idx].clone(), names[idx + 1].clone()) {
            permuted_names.push((names[idx].clone(), names[idx + 1].clone()));
        }
    }

    // Find permuted names with the first `chars_aligned` letters being equal
    let like_names = filter_names_with_alignment(permuted_names, chars_aligned);
    println!("{:?}", like_names);
}

fn get_names() -> Vec<String> {
    let mut file = File::open(Path::new("src/names.json")).expect("Could not open names.json");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read names.json");

    serde_json::from_str::<Vec<String>>(&contents).expect("Could not convert to vector from JSON")
}

fn check_if_permutation(name1: String, name2: String) -> bool {
    let name1_chars: HashSet<char> = name1.chars().collect();
    let name2_chars: HashSet<char> = name2.chars().collect();

    name1_chars == name2_chars
}

fn filter_names_with_alignment(
    permuted_names: Vec<(String, String)>,
    char_aligned: usize,
) -> Vec<(String, String)> {
    let mut filtered_names: Vec<(String, String)> = Vec::new();

    'name_roll: for (name1, name2) in permuted_names {
        let chars_name1: Vec<char> = name1.chars().collect();
        let chars_name2: Vec<char> = name2.chars().collect();

        for (idx, (char_1, char_2)) in
            std::iter::zip(chars_name1.clone(), chars_name2.clone()).enumerate()
        {
            if idx < char_aligned && char_1 != char_2 {
                break 'name_roll;
            }
        }
        filtered_names.push((name1, name2));
    }

    filtered_names
}

trait SymmetricDifference<T>
where
    T: PartialOrd,
{
    fn symmetric_difference(&self, rhs: Vec<(T, T)>) -> Vec<(T, T)>;
}

impl SymmetricDifference<String> for Vec<(String, String)> {
    fn symmetric_difference(&self, rhs: Vec<(String, String)>) -> Vec<(String, String)> {
        let mut diffed: Vec<(String, String)> = Vec::new();
        for idx in 0..min(self.len(), rhs.len()) {
            if !self.contains(&rhs[idx]) {
                diffed.push(rhs[idx].clone())
            }
        }

        diffed
    }
}
