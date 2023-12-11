use std::{
    fmt::{Debug, Write},
    fs::read_to_string,
};

const SIZE: usize = 140;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Heading {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Heading {
    fn turned(self, by: Heading) -> Self {
        let rot_sum = (self as u8 + by as u8) % 4;

        match rot_sum {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            _ => Heading::West,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PipeKind {
    Unknown,
    Start,
    Straight,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Pipe {
    symbol: char,
    kind: PipeKind,
    start_heading: Heading, // heading into pipe
    is_loop: bool,
    is_enclosed: bool,
}

impl Pipe {
    fn from_char(symbol: char) -> Self {
        Pipe {
            symbol,
            ..Default::default()
        }
    }

    /// parse pipe from char and deflect the heading
    fn traverse(&mut self, heading: Heading) -> Heading {
        self.start_heading = heading;

        let (kind, new_heading) = match self.symbol {
            '.' => panic!(),
            'S' => (PipeKind::Start, heading),
            '|' => (
                PipeKind::Straight,
                match heading {
                    Heading::North | Heading::South => heading,
                    _ => panic!(),
                },
            ),
            '-' => (
                PipeKind::Straight,
                match heading {
                    Heading::West | Heading::East => heading,
                    _ => panic!(),
                },
            ),
            'L' => match heading {
                Heading::South => (PipeKind::Left, Heading::East),
                Heading::West => (PipeKind::Right, Heading::North),
                _ => panic!(),
            },
            'J' => match heading {
                Heading::South => (PipeKind::Right, Heading::West),
                Heading::East => (PipeKind::Left, Heading::North),
                _ => panic!(),
            },
            '7' => match heading {
                Heading::North => (PipeKind::Left, Heading::West),
                Heading::East => (PipeKind::Right, Heading::South),
                _ => panic!(),
            },
            'F' => match heading {
                Heading::North => (PipeKind::Right, Heading::East),
                Heading::West => (PipeKind::Left, Heading::South),
                _ => panic!(),
            },
            _ => panic!(),
        };

        self.kind = kind;
        new_heading
    }

    fn get_left_successors(&self) -> Vec<Heading> {
        // assumes entry from south
        let succ = match self.kind {
            PipeKind::Straight => vec![Heading::North, Heading::West],
            PipeKind::Unknown | PipeKind::Start => {
                vec![Heading::North, Heading::East, Heading::South, Heading::West]
            }
            PipeKind::Left => vec![Heading::West],
            PipeKind::Right => vec![Heading::West, Heading::North, Heading::East],
        };

        succ.iter().map(|h| h.turned(self.start_heading)).collect()
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self {
            symbol: 'X',
            kind: PipeKind::Unknown,
            start_heading: Heading::North,
            is_loop: false,
            is_enclosed: false,
        }
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.symbol {
            '|' => " ║",
            '-' => "══",
            'L' => " ╚",
            'J' => "═╝",
            '7' => "═╗",
            'F' => " ╔",
            'S' => "╳╳",
            '.' => "░░",
            _ => panic!(),
        };

        if self.is_loop {
            // print red
            f.write_fmt(format_args!("\x1b[31m{}\x1b[0m", c))?;
        } else if self.is_enclosed {
            // print green
            f.write_fmt(format_args!("\x1b[32m{}\x1b[0m", c))?;
        } else {
            f.write_str(c)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct GridPos {
    x: i32,
    y: i32,
}

impl GridPos {
    fn modified(&self, heading: Heading) -> Self {
        // TODO bounds check
        let vec = match heading {
            Heading::North => (0, -1),
            Heading::South => (0, 1),
            Heading::West => (-1, 0),
            Heading::East => (1, 0),
        };
        Self {
            x: vec.0 + self.x,
            y: vec.1 + self.y,
        }
    }
}

struct Grid {
    pipes: [[Pipe; SIZE]; SIZE],
    start_pos: GridPos,
}

impl Grid {
    fn from_str(map: &str) -> Self {
        let mut pipes = [[Pipe::default(); SIZE]; SIZE];
        let mut start_pos = None;

        for (y, row) in map.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                pipes[y][x] = Pipe::from_char(c);

                if c == 'S' {
                    start_pos = Some(GridPos {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        Grid {
            pipes,
            start_pos: start_pos.unwrap(),
        }
    }

    fn get_pipe(&mut self, pos: GridPos) -> Option<&mut Pipe> {
        if pos.x < 0 || pos.x >= SIZE as i32 || pos.y < 0 || pos.y >= SIZE as i32 {
            None
        } else {
            Some(&mut self.pipes[pos.y as usize][pos.x as usize])
        }
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

    // === traverse loop ===
    let mut heading = Heading::South; // initial guess
    let mut pos = grid.start_pos;

    loop {
        pos = pos.modified(heading);
        let current = grid.get_pipe(pos).unwrap();

        heading = current.traverse(heading);
        current.is_loop = true;

        if current.kind == PipeKind::Start {
            break;
        }
    }

    // === floodfill ===
    let mut stack = vec![grid.start_pos];
    let mut count = 0;

    while let Some(current) = stack.pop() {
        if let Some(pipe) = grid.get_pipe(current) {
            if !pipe.is_enclosed {
                pipe.is_enclosed = true;

                if !pipe.is_loop {
                    count += 1;
                }

                for next_heading in pipe.get_left_successors() {
                    stack.push(current.modified(next_heading));
                }
            }
        }
    }

    println!("{:?}", grid);
    println!("\ncount: {count}");
}
