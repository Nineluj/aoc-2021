use std::collections::HashMap;

use crate::to_num_arr_with_split;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
// A line between two points, the start
// always has the lower x value
struct Line {
    start: Pos,
    end: Pos,
}

fn get_min_max(a: i32, b: i32) -> (i32, i32) {
    if a > b { (b,a) } else { (a,b) }
}

impl Line {
    fn from_str(s: &str) -> Line {
        let parts = s.split_once(" -> ").unwrap();
        let from = to_num_arr_with_split(parts.0, ',');
        let to = to_num_arr_with_split(parts.1, ',');

        let pos1 = Pos { x: from[0], y: from[1] };
        let pos2 = Pos { x: to[0], y: to[1] };

        if pos1.x < pos2.x {
            Line {
                start: pos1,
                end: pos2,
            }
        } else {
            Line {
                start: pos2,
                end: pos1,
            }
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    // Get points
    fn get_points(&self) -> Vec<Pos> {
        let mut points = Vec::new();

        if self.start.x == self.end.x {
            let (init,end) = get_min_max(self.start.y, self.end.y);
            for y in init..=end {
                points.push(Pos{x: self.start.x, y});
            }
        } else if self.start.y == self.end.y {
            let (init,end) = (self.start.x, self.end.x);
            for x in init..=end {
                points.push(Pos{x, y: self.start.y});
            }
        } else {
            // Top left to bottom right
            if self.end.y > self.start.y {
                let mut cur = self.start;
                while cur != self.end {
                    points.push(cur);
                    cur.x += 1;
                    cur.y += 1;
                }
                points.push(cur);
            } else { // Bottom left to top right
                let mut cur = self.end;
                while cur != self.start {
                    points.push(cur);
                    cur.x -= 1;
                    cur.y += 1;
                }
                points.push(cur);
            }
        }
        points
    }
}

fn get_lines(input: String) -> Vec<Line> {
    let mut lines = Vec::new();
    for text_line in input.split("\n") {
        if !text_line.is_empty() {
            lines.push(Line::from_str(text_line));
        }
    }
    lines
}

pub fn part1(input: String) {
    let mut lines = get_lines(input);
    lines.retain(Line::is_horizontal_or_vertical);
    let mut points: HashMap<Pos,usize> = HashMap::new();
    for l in lines {
        for p in l.get_points() {
            let count = points.entry(p).or_insert(0);
            *count += 1;
        }
    }

    let overlaps = points.values().filter(|x| **x >= 2).collect::<Vec<_>>().len();
    println!("{}", overlaps);
}

pub fn part2(input: String) {
    let lines = get_lines(input);
    let mut points: HashMap<Pos,usize> = HashMap::new();
    for l in lines {
        for p in l.get_points() {
            let count = points.entry(p).or_insert(0);
            *count += 1;
        }
    }

    let overlaps = points.values().filter(|x| **x >= 2).collect::<Vec<_>>().len();
    println!("{}", overlaps);
}
