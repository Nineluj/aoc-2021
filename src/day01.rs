use crate::to_num_arr;

fn increasing_count(data: Vec<i32>) -> i32 {
    let mut incr_count: i32 = 0;
    let mut prev_m = data[0];
    for ii in 1..data.len() {
        let m = data[ii];
        if m > prev_m {
            incr_count += 1;
        }

        prev_m = m;
    }

    return incr_count;
}

pub fn part1(input: String) {
    let data = to_num_arr(&input);
    let depth_incr_count = increasing_count(data);
    println!("{}", depth_incr_count)
}

pub fn part2(input: String) {
    let data = to_num_arr(&input);
    assert!(data.len() >= 3);

    let mut new_vec = Vec::<i32>::new();
    let mut curr_window = data[0] + data[1] + data[2];
    new_vec.push(curr_window);

    // Skip first 2 indeces
    for ii in 3..data.len() {
        curr_window = curr_window + data[ii] - data[ii-3];
        new_vec.push(curr_window);
    }

    let depth_incr_count = increasing_count(new_vec);
    println!("{}", depth_incr_count);
}
