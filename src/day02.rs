pub fn part1(input: String) {
    let mut x = 0;
    let mut y = 0;
    input.split("\n").for_each(|line| {
        if !line.is_empty() {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let dir = parts[0];
            let magnitude = parts[1].parse::<i32>().unwrap();

            match dir {
                "forward" => {
                    x += magnitude;
                },
                "down" => {
                    y += magnitude;
                },
                "up" => {
                    y -= magnitude;
                },
                _ => panic!("No direction match for {}", dir)
            }
        }
    });

    println!("x={} y={}, mult={}", x, y, x * y);
}

pub fn part2(input: String) {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    input.split("\n").for_each(|line| {
        if !line.is_empty() {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let dir = parts[0];
            let magnitude = parts[1].parse::<i32>().unwrap();

            match dir {
                "forward" => {
                    x += magnitude;
                    y += aim * magnitude;
                },
                "down" => {
                    aim += magnitude;
                },
                "up" => {
                    aim -= magnitude;
                },
                _ => panic!("No direction match for {}", dir)
            }
        }
    });

    println!("x={} y={}, mult={}", x, y, x * y);
}
