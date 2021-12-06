use crate::to_num_arr_with_split;

const NUM_AGES: usize = 9;

#[derive(Debug)]
struct FishCounter {
    data: [usize;NUM_AGES]
}

impl FishCounter {
    fn init_from_str(input: String) -> FishCounter {
        let fish_input = to_num_arr_with_split(&input, ',');
        let mut data = [0; NUM_AGES];

        for age in fish_input {
            data[age as usize] += 1;
        }
        FishCounter { data }
    }

    fn tick(&mut self) {
        let mut new_data = [0; NUM_AGES];

        // - other numbers decreases by 1 if it was present at the start of the day.
        for i in 0..(NUM_AGES-1) {
            new_data[i] = self.data[i + 1];
        }
        // handle births and birthing
        // - 0 becomes a 6 and adds a new 8 to the end of the list
        new_data[6] += self.data[0];
        new_data[8] += self.data[0];
        self.data = new_data;
    }

    fn sum(&self) -> usize {
        self.data.iter().sum()
    }
}

pub fn part1(input: String) {
    let mut counter = FishCounter::init_from_str(input);
    const DAYS: usize = 80;

    for _ in 0..DAYS {
        counter.tick();
    }
    println!("Sum {}", counter.sum());
}

pub fn part2(input: String) {
    let mut counter = FishCounter::init_from_str(input);
    const DAYS: usize = 256;

    for _ in 0..DAYS {
        counter.tick();
    }
    println!("Sum {}", counter.sum());
}
