use std::fmt;

// Unsigned boolean
#[derive(Clone)]
struct Bin {
    data: Vec<bool>
}

impl fmt::Debug for Bin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bin").field("data", &self.data).finish()
    }
}

impl Bin {
    const BASE: usize = 2;

    fn new() -> Bin {
        Bin { data: Vec::new() }
    }

    fn from_binstr(binstr: &str) -> Bin {
        Bin { data: binstr.chars().map(|x| if x == '1' { true } else { false }).collect() }
    }

    fn to_u32(&self) -> usize {
        let mut result = 0;
        for i in 0..self.data.len() {
            if self.data[i] {
                result += Bin::BASE.pow((self.data.len() - 1 - i) as u32);
            }
        }
        result
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, val: bool) {
        self.data.push(val);
    }

    fn get(&self, index: usize) -> bool {
        self.data[index]
    }
}

fn most_common(bins: &Vec<Bin>, index: usize) -> bool {
    let mut count = 0;
    for bvec in bins {
        if bvec.get(index) {
            count += 1;
        }
    }
    return 2 * count >= bins.len()
}

fn get_bins(input: String) -> Vec<Bin> {
    input.split("\n").filter(|item| !item.is_empty()).map(Bin::from_binstr).collect::<Vec<_>>()
}

fn get_gamma_epsilon(bins: &Vec<Bin>) -> (Bin, Bin) {
    let bin_len = bins[0].len();  // we assume that all the bins are the same length
    let mut gbin = Bin::new();
    let mut ebin = Bin::new();
    for i in 0..bin_len {
        if most_common(&bins, i) {
            gbin.push(true);
            ebin.push(false);
        } else {
            gbin.push(false);
            ebin.push(true);
        }
    }

    (gbin, ebin)
}

pub fn part1(input: String) {
    let bins = get_bins(input);
    let (gbin, ebin) = get_gamma_epsilon(&bins);
    let g = gbin.to_u32();
    let e = ebin.to_u32();

    println!("g {}, e {}, product {}", g, e, g * e);
}

pub fn part2(input: String) {
    let bins = get_bins(input);
    let bin_len = bins[0].len();

    // find o2r
    let mut candidates: Vec<Bin> = bins.to_vec();
    for ii in 0..bin_len {
        let (g, _) = get_gamma_epsilon(&candidates);
        let expected = g.get(ii);
        candidates = candidates.into_iter().filter(|b| b.get(ii) == expected).collect();

        if candidates.len() == 1 {
            break;
        }
    }
    assert!(candidates.len() == 1);
    let o2r = &candidates[0].to_u32();

    // find co2r
    let mut candidates: Vec<Bin> = bins.to_vec();
    for ii in 0..bin_len {
        let (_, e) = get_gamma_epsilon(&candidates);
        let expected = e.get(ii);
        candidates = candidates.into_iter().filter(|b| b.get(ii) == expected).collect();

        if candidates.len() == 1 {
            break;
        }
    }
    assert!(candidates.len() == 1);
    let co2r = &candidates[0].to_u32();

    println!("o2 rating {}, co2 rating {}, product {}", o2r, co2r, o2r * co2r);
}
