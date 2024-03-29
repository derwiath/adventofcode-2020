use std::env;
use std::fmt;
use std::fs;
use std::ops;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Seat {
    F,
    E,
    O,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Seat::F => '.',
            Seat::E => 'L',
            Seat::O => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Size {
    width: usize,
    height: usize,
}

impl Size {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.width, self.height)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn inc(&mut self, other: &Pos) {
        self.x += other.x;
        self.y += other.y;
    }

    fn index(&self, size: &Size) -> Option<usize> {
        if self.x < 0 || self.y < 0 {
            return None;
        }

        let x = self.x as usize;
        let y = self.y as usize;
        if x >= size.width {
            None
        } else if y >= size.height {
            None
        } else {
            Some(x + y * size.width)
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct OccupiedCounts {
    size: Size,
    counts: Vec<u8>,
}

impl OccupiedCounts {
    fn new(size: Size, counts: Vec<u8>) -> Self {
        Self { size, counts }
    }

    fn create_from_adjacent(layout: &Layout) -> OccupiedCounts {
        let mut counts = Vec::<u8>::with_capacity(layout.seats.len());

        let offsets: [Pos; 8] = [
            Pos::new(-1, -1),
            Pos::new(0, -1),
            Pos::new(1, -1),
            Pos::new(-1, 0),
            Pos::new(1, 0),
            Pos::new(-1, 1),
            Pos::new(0, 1),
            Pos::new(1, 1),
        ];

        for y in 0..layout.size.height {
            for x in 0..layout.size.width {
                let pos = Pos::new(x as isize, y as isize);
                let count = offsets
                    .iter()
                    .filter(|offset| {
                        let neighbour = pos.clone() + (*offset).clone();
                        layout.get_seat_at(&neighbour) == Some(&Seat::O)
                    })
                    .count();
                counts.push(count as u8);
            }
        }

        OccupiedCounts::new(layout.size.clone(), counts)
    }

    fn create_from_line_of_sight(layout: &Layout) -> OccupiedCounts {
        let mut counts = Vec::<u8>::with_capacity(layout.seats.len());

        let directions: [Pos; 8] = [
            Pos::new(0, -1),
            Pos::new(1, -1),
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(0, 1),
            Pos::new(-1, 1),
            Pos::new(-1, 0),
            Pos::new(-1, -1),
        ];

        for y in 0..layout.size.height {
            for x in 0..layout.size.width {
                let pos = Pos::new(x as isize, y as isize);
                let count = directions
                    .iter()
                    .filter(|direction| layout.ray_cast(&pos, direction) == Some(&Seat::O))
                    .count();
                counts.push(count as u8);
            }
        }

        OccupiedCounts::new(layout.size.clone(), counts)
    }
}

impl fmt::Display for OccupiedCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pos = 0;
        for _ in 0..self.size.height {
            let next_pos = pos + self.size.width;
            for count in &self.counts[pos..next_pos] {
                write!(f, "{}", count)?;
            }
            write!(f, "\n")?;
            pos = next_pos;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Layout {
    size: Size,
    seats: Vec<Seat>,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pos = 0;
        for _ in 0..self.size.height {
            let next_pos = pos + self.size.width;
            for seat in &self.seats[pos..next_pos] {
                write!(f, "{}", seat)?;
            }
            write!(f, "\n")?;
            pos = next_pos;
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl Layout {
    fn new(s: &str) -> Self {
        let width = s
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .filter(|c| *c != '\r' && *c != '\n')
            .count();
        let seats: Vec<Seat> = s
            .replace("\n", "")
            .replace("\r", "")
            .chars()
            .map(|seat| match seat {
                '.' => Seat::F,
                'L' => Seat::E,
                '#' => Seat::O,
                c => panic!("Whats up with '{}'", c as u8),
            })
            .collect();
        let height = seats.len() / width;

        assert_eq!(seats.len() % width, 0);
        Self {
            size: Size::new(width, height),
            seats,
        }
    }

    fn step(&self) -> Self {
        let mut seats = Vec::<Seat>::with_capacity(self.seats.len());
        let occupied = OccupiedCounts::create_from_adjacent(self);
        for (i, seat) in self.seats.iter().enumerate() {
            let count = occupied.counts[i];
            seats.push(match seat {
                Seat::E => {
                    if count == 0 {
                        Seat::O
                    } else {
                        Seat::E
                    }
                }
                Seat::O => {
                    if count >= 4 {
                        Seat::E
                    } else {
                        Seat::O
                    }
                }
                Seat::F => Seat::F,
            })
        }

        let transformed = Self {
            size: self.size.clone(),
            seats,
        };
        /*
        println!("{}\n", self);
        println!("{}\n", occupied);
        println!("{}", transformed);
        */
        transformed
    }

    fn step2(&self) -> Self {
        let mut seats = Vec::<Seat>::with_capacity(self.seats.len());
        let occupied = OccupiedCounts::create_from_line_of_sight(self);
        for (i, seat) in self.seats.iter().enumerate() {
            let count = occupied.counts[i];
            seats.push(match seat {
                Seat::E => {
                    if count == 0 {
                        Seat::O
                    } else {
                        Seat::E
                    }
                }
                Seat::O => {
                    if count >= 5 {
                        Seat::E
                    } else {
                        Seat::O
                    }
                }
                Seat::F => Seat::F,
            })
        }

        let transformed = Self {
            size: self.size.clone(),
            seats,
        };
        /*
        println!("{}\n", self);
        println!("{}\n", occupied);
        println!("{}", transformed);
        */
        transformed
    }

    fn get_seat_at(&self, pos: &Pos) -> Option<&Seat> {
        if let Some(index) = pos.index(&self.size) {
            self.seats.get(index)
        } else {
            None
        }
    }

    fn ray_cast(&self, pos: &Pos, dir: &Pos) -> Option<&Seat> {
        let mut pos = pos.clone();
        pos.inc(dir);
        while let Some(seat) = self.get_seat_at(&pos) {
            if seat != &Seat::F {
                return Some(seat);
            }
            pos.inc(dir);
        }
        None
    }
}

fn solve_part1(input: &str) -> usize {
    let mut layout = Layout::new(input);
    let mut prev_layout: Option<Layout> = None;
    let mut steps = 0;
    while prev_layout.is_none() || layout != prev_layout.unwrap() {
        steps += 1;
        println!("{}", steps);
        let new_layout = layout.step();
        prev_layout = Some(layout);
        layout = new_layout;
    }
    layout.seats.iter().filter(|s| **s == Seat::O).count()
}

fn solve_part2(input: &str) -> usize {
    let mut layout = Layout::new(input);
    let mut prev_layout: Option<Layout> = None;
    let mut steps = 0;
    while prev_layout.is_none() || layout != prev_layout.unwrap() {
        steps += 1;
        println!("{}", steps);
        let new_layout = layout.step2();
        prev_layout = Some(layout);
        layout = new_layout;
    }
    layout.seats.iter().filter(|s| **s == Seat::O).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day11 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[allow(dead_code)]
#[cfg(test)]
mod tests11 {
    use super::*;

    const STATES: [&str; 6] = [
        "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
        "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
    ];

    const SMALL_STATES: [&str; 2] = [
        "\
#.##.
#####
#.#.#
####.
#.##.",
        "\
#.LL.
#LLLL
L.L.L
#LLL.
#.LL.",
    ];

    #[test]
    fn test1_parse_layout_size() {
        assert_eq!(Layout::new(STATES[0]).size, Size::new(10, 10));
    }

    #[test]
    fn test1_parse_layout_seats_len() {
        assert_eq!(Layout::new(STATES[0]).seats.len(), 10 * 10);
    }

    #[test]
    fn test1_step_01() {
        let layouts: Vec<Layout> = STATES.iter().map(|state| Layout::new(state)).collect();
        assert_eq!(layouts[0].step(), layouts[1]);
    }

    #[test]
    fn test1_step_12() {
        let layouts: Vec<Layout> = STATES.iter().map(|state| Layout::new(state)).collect();
        assert_eq!(layouts[1].step(), layouts[2]);
    }

    #[test]
    fn test1_step_12_small_occupied_4() {
        let state = "\
            #.##\n\
            ####\n\
            #.#.\n\
            ####";
        let answer = vec![
            2, 5, 4, 3, //
            3, 6, 5, 4, //
            4, 8, 6, 5, //
            2, 4, 3, 2, //
        ];

        let layout = Layout::new(state);
        let occupied = OccupiedCounts::create_from_adjacent(&layout);
        let facit = OccupiedCounts::new(Size::new(4, 4), answer);
        println!("{}\n{}\n{}", layout, occupied, facit);

        assert_eq!(occupied, facit);
    }

    #[test]
    fn test1_step_12_small_occupied_3() {
        let state = "\
            #.#\n\
            ###\n\
            #.#";
        let answer = vec![
            2, 5, 2, //
            3, 6, 3, //
            2, 5, 2, //
        ];

        let layout = Layout::new(state);
        let occupied = OccupiedCounts::create_from_adjacent(&layout);
        let facit = OccupiedCounts::new(Size::new(3, 3), answer);
        println!("{}\n{}\n{}", layout, occupied, facit);

        assert_eq!(occupied, facit);
    }

    #[test]
    fn test1_step_12_small_occupied_2() {
        let state = "\
            #.\n\
            ##";
        let answer = vec![
            2, 3, //
            2, 2, //
        ];

        let layout = Layout::new(state);
        let occupied = OccupiedCounts::create_from_adjacent(&layout);
        let facit = OccupiedCounts::new(Size::new(2, 2), answer);
        println!("{}\n{}\n{}", layout, occupied, facit);

        assert_eq!(occupied, facit);
    }

    #[test]
    fn test1_step_23() {
        let layouts: Vec<Layout> = STATES.iter().map(|state| Layout::new(state)).collect();
        assert_eq!(layouts[2].step(), layouts[3]);
    }

    #[test]
    fn test1_step_34() {
        let layouts: Vec<Layout> = STATES.iter().map(|state| Layout::new(state)).collect();
        assert_eq!(layouts[3].step(), layouts[4]);
    }

    #[test]
    fn test1_step_45() {
        let layouts: Vec<Layout> = STATES.iter().map(|state| Layout::new(state)).collect();
        assert_eq!(layouts[4].step(), layouts[5]);
    }

    #[test]
    fn test1_occupied_seats() {
        assert_eq!(solve_part1(STATES[0]), 37);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_occupied_seats() {
        assert_eq!(solve_part2(STATES[0]), 26);
    }
}
