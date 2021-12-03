#[derive (Debug, Clone, Copy)]
struct BinCount {
    num_ones: u32,
    num_zeroes: u32,
}

pub fn part1(input: String) {
    let bins = input.split("\n").collect::<Vec<_>>();
    let mut counts = vec!(BinCount{num_ones: 0, num_zeroes: 0}; bins[0].len());

    for b in bins {
        for (ii, digit) in b.chars().enumerate() {
            match digit {
                '0' => counts[ii].num_zeroes += 1,
                '1' => counts[ii].num_ones += 1,
                _ => panic!("not valid bin"),
            }
        }
    }

    let mut most_common = String::new();
    let mut least_common = String::new();

    for c in counts {
        if c.num_ones > c.num_zeroes {
            most_common += "1";
            least_common += "0";
        } else {
            most_common += "0";
            least_common += "1";
        }
    }

    let gamma = isize::from_str_radix(&most_common, 2).unwrap();
    let epsilon = isize::from_str_radix(&least_common, 2).unwrap();

    println!("gamma {} epsilon {}, product {}", gamma, epsilon, gamma * epsilon);
}

pub fn part2(input: String) {
    println!("hello");
}
