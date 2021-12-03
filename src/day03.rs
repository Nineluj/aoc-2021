use crate::bin_string_to_i32;

#[derive (Debug, Clone, Copy)]
struct BinCount {
    num_ones: u32,
    num_zeroes: u32,
}

fn get_bin_counts(data: Vec<&str>) -> Vec<BinCount> {
    let mut counts = vec!(BinCount{num_ones: 0, num_zeroes: 0}; data[0].len());

    for b in data {
        for (ii, digit) in b.chars().enumerate() {
            match digit {
                '0' => counts[ii].num_zeroes += 1,
                '1' => counts[ii].num_ones += 1,
                _ => panic!("not valid bin"),
            }
        }
    }

    counts
}

fn get_most_least_common(bin_counts: Vec<BinCount>) -> (String, String) {
    let mut most_common = String::new();
    let mut least_common = String::new();

    for c in bin_counts {
        if c.num_ones > c.num_zeroes {
            most_common += "1";
            least_common += "0";
        } else {
            most_common += "0";
            least_common += "1";
        }
    }

    (most_common, least_common)
}

pub fn part1(input: String) {
    let bins = input.split("\n").collect::<Vec<_>>();
    let counts = get_bin_counts(bins);

    let (most_common, least_common) = get_most_least_common(counts);
    let gamma = bin_string_to_i32(&most_common);
    let epsilon = bin_string_to_i32(&least_common);

    println!("gamma {} epsilon {}, product {}", gamma, epsilon, gamma * epsilon);
}

fn prefix_match_length(s1: &str, s2: &str) -> u32 {
    match s1.strip_prefix(s2) {
        Some(a) => (s1.len() - a.len()) as u32,
        None => 0,
    }
}

// Finds the item in the haystack that shares the most number of characters with the needle
fn longest_prefix_match(needle: String, haystack: Vec<&str>) -> Option<String> {
    print!("{} {:?}\n", needle, haystack);
    let mut longest_match = 0;
    let mut longest_result: Option<String> = None;

    for h in haystack {
        let plen = prefix_match_length(&needle, &h);
        if plen > longest_match {
            println!("{} {} {}", h, plen, longest_match);
            longest_match = plen;
            longest_result = Some(h.to_string());
        }
    }

    longest_result
}

pub fn part2(input: String) {
    let bins = input.split("\n").collect::<Vec<_>>();
    let counts = get_bin_counts(bins.clone());

    let (most_common, least_common) = get_most_least_common(counts);

    let oxygen = longest_prefix_match(most_common, bins.clone()).unwrap();
    let co2 = longest_prefix_match(least_common, bins.clone()).unwrap();
    // get longest prefix match
    println!("{} {}", oxygen, co2);
}
