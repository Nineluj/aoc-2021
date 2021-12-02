use crate::to_num_arr;

pub fn part1(input: String) {
    let data = to_num_arr(input);

    let mut depth_incr_count = 0;
    let mut prev_m = data[0];
    for ii in 1..data.len() {
        let m = data[ii];
        if m > prev_m {
            depth_incr_count += 1;
        }

        prev_m = m;
    }

    println!("{}", depth_incr_count)
}

pub fn part2(input: String) {
    let data = to_num_arr(input);
    assert!(data.len() >= 3);

    let mut past_window = data[..=2].iter().sum::<i32>();
    let mut curr_window = past_window;
    let mut depth_incr_count = 0;

    for ii in 3..data.len() {
        curr_window = curr_window + data[ii] - data[ii-3];
        if curr_window > past_window {
            depth_incr_count += 1;
        }

        past_window = curr_window;
    }

    println!("{}", depth_incr_count);
}
