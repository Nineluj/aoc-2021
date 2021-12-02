// Days
pub mod day01;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}

pub fn to_num_arr(input: String) -> Vec<i32> {
    input.split("\n").filter(|line| {
        !line.is_empty()
    }).map(|line| {
        line.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>()
}
