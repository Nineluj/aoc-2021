use crate::to_num_arr_with_split;

#[derive(Debug, Copy, Clone)]
struct BingoNumber {
    num: i32,
    is_crossed: bool,
}

#[derive(Debug, Clone)]
struct BingoBoard {
    data: Vec<BingoNumber>
}

impl BingoBoard {
    fn new(raw: &[&str]) -> BingoBoard {
        BingoBoard {
            data: raw.iter().fold(Vec::new(), |mut acc, item| {
                to_num_arr_with_split(item, ' ').into_iter().for_each(|ele| {
                    acc.push(BingoNumber {num: ele, is_crossed: false});
                });
                acc
            })
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> BingoNumber {
        self.data[x + 5 * y]
    }

    fn check_win(&self, init_x: usize, init_y: usize, x_step: usize, y_step: usize) -> bool {
        let mut x = init_x;
        let mut y = init_y;
        while x < 5 && y < 5 {
            if !self.get_cell(x, y).is_crossed {
                return false
            }
            x += x_step;
            y += y_step;
        }
        true
    }

    fn is_won(&self) -> bool {
        // vertical and horizontal
        for i in 0..5 {
            if self.check_win(i, 0, 0, 1) || self.check_win(0, i, 1, 0) {
                return true
            }
        }
        false
    }

    fn mark(&mut self, num: i32) {
        match self.data.iter_mut().find(|bn| bn.num == num ) {
            Some(mut x) => x.is_crossed = true,
            None => (),
        }
    }

    fn get_uncrossed_sum(&self) -> i32 {
        self.data.iter().filter(|bn| !bn.is_crossed).map(|bn| bn.num).sum()
    }
}

fn get_boards(lines: &[&str]) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    for i in (1..lines.len()-1).step_by(6) {
        let items = lines[i..i+6].to_vec();
        let new_board = BingoBoard::new(&items);
        boards.push(new_board);
    }
    boards
}

// Read the input containing a list of drawn numbers and bingo boards
// and find the sum of the uncrossed numbers multiplied by the number drawn
// to get a winning board
pub fn part1(input: String) {
    let lines = input.split("\n").collect::<Vec<_>>();
    let feed = to_num_arr_with_split(lines[0], ',');
    let mut boards = get_boards(&lines);

    for n in feed {
        for b in boards.iter_mut() {
            b.mark(n);

            if b.is_won() {
                println!("{}", n * b.get_uncrossed_sum());
                return
            }
        }
    }
}

// Same as part one but print the results for the last bingo board
// to win the game
pub fn part2(input: String) {
    let lines = input.split("\n").collect::<Vec<_>>();
    let feed = to_num_arr_with_split(lines[0], ',');
    let mut boards = get_boards(&lines);

    for n in feed {
        let mut remove_indices: Vec<usize> = Vec::new();
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark(n);

            if b.is_won() {
                remove_indices.push(i);
            }
        }

        if boards.len() == 1 && remove_indices.len() == 1 {
            println!("{}", n * boards[remove_indices[0]].get_uncrossed_sum());
            return
        }

        for i in remove_indices.iter().rev() {
            boards.remove(*i);
        }
    }
}
