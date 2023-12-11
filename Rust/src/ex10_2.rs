use std::{
    fmt::{Debug, Write},
    fs::read_to_string,
};

const SIZE: usize = 140;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn reverse(&self) -> Self {
        match self {
            Heading::North => Heading::South,
            Heading::South => Heading::North,
            Heading::East => Heading::West,
            Heading::West => Heading::East,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    None,
    Start,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Pipe::None,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::BendNE,
            'J' => Pipe::BendNW,
            '7' => Pipe::BendSW,
            'F' => Pipe::BendSE,
            'S' => Pipe::Start,
            _ => panic!(),
        }
    }

    fn conduct(&self, heading: Heading) -> Option<Heading> {
        let connect = |from, to| match heading.reverse() {
            cur if cur == from => Some(to),
            cur if cur == to => Some(from),
            _ => None,
        };

        match self {
            Pipe::Vertical => connect(Heading::North, Heading::South),
            Pipe::Horizontal => connect(Heading::East, Heading::West),
            Pipe::BendNE => connect(Heading::North, Heading::East),
            Pipe::BendNW => connect(Heading::North, Heading::West),
            Pipe::BendSE => connect(Heading::South, Heading::East),
            Pipe::BendSW => connect(Heading::South, Heading::West),
            _ => None,
        }
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct GridPos {
    x: usize,
    y: usize,
}

impl GridPos {
    fn go(&self, heading: Heading) -> Self {
        // TODO bounds check
        match heading {
            Heading::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Heading::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Heading::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Heading::East => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
struct PipeState {
    pipe: Pipe,
    //distance: u32,
    is_loop: bool,
}

impl Debug for PipeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.pipe {
            Pipe::None => '.',
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::BendNE => 'L',
            Pipe::BendNW => 'J',
            Pipe::BendSW => '7',
            Pipe::BendSE => 'F',
            Pipe::Start => 'S',
        };

        if self.is_loop {
            // print red
            f.write_fmt(format_args!("\x1b[31m{c}\x1b[0m"))?;
        } else {
            f.write_char(c)?;
        }

        Ok(())
    }
}

struct Grid {
    pipes: [[PipeState; SIZE]; SIZE],
    current_pos: GridPos,
}

impl Grid {
    fn from_str(map: &str) -> Self {
        let mut pipes = [[PipeState::default(); SIZE]; SIZE];

        let mut start_pos = None;

        for (y, row) in map.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pipe = Pipe::from_char(c);
                pipes[y][x] = PipeState {
                    pipe,
                    // distance: 0,
                    is_loop: false,
                };
                if pipe == Pipe::Start {
                    start_pos = Some(GridPos { x, y });
                }
            }
        }

        Grid {
            pipes,
            current_pos: start_pos.unwrap(),
        }
    }

    fn go_next(&mut self, heading: Heading) -> &mut PipeState {
        self.current_pos = self.current_pos.go(heading);
        &mut self.pipes[self.current_pos.y][self.current_pos.x]
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pipes {
            for pipe in row {
                pipe.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn run() {
    let map = read_to_string("./ex10-1.txt").unwrap();

    let mut grid = Grid::from_str(&map);

    let mut heading = Heading::South; // initial guess
    let mut distance = 0;

    loop {
        let current = grid.go_next(heading);
        current.is_loop = true;
        distance += 1;

        if let Some(next_heading) = current.pipe.conduct(heading) {
            heading = next_heading;
        } else {
            break;
        }
    }

    println!("{:?}", grid);
    println!("\ndistance: {distance}");
    println!("\nfurthest: {}", distance / 2);
}
