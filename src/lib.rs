// Days
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),

        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}

pub fn to_num_arr(input: &str) -> Vec<i32> {
    to_num_arr_with_split(input, '\n')
}

pub fn to_num_arr_with_split(input: &str, sep: char) -> Vec<i32> {
    input.split(sep).filter(|line| {
        !line.is_empty()
    }).map(|line| {
        line.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>()
}

pub fn bin_string_to_i32(input: &str) -> i32 {
    isize::from_str_radix(&input, 2).unwrap() as i32
}
