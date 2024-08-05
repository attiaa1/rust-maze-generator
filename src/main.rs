use std::io;
use rand::prelude::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
    SolvedPath,
}

struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Wall; width]; height];
        Maze { width, height, grid }
    }
    
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut stack = vec![(0, 0)];
        self.grid[0][0] = Cell::Path;

        while let Some((x, y)) = stack.pop() {
            let mut neighbors = vec![];

            if x > 1 && self.grid[y][x - 2] == Cell::Wall {
                neighbors.push((x - 2, y));
            }
            if x < self.width - 2 && self.grid[y][x + 2] == Cell::Wall {
                neighbors.push((x + 2, y));
            }
            if y > 1 && self.grid[y - 2][x] == Cell::Wall {
                neighbors.push((x, y - 2));
            }
            if y < self.height - 2 && self.grid[y + 2][x] == Cell::Wall {
                neighbors.push((x, y + 2));
            }

            if !neighbors.is_empty() {
                stack.push((x, y));

                let &(nx, ny) = neighbors.choose(&mut rng).unwrap();

                self.grid[(y + ny) / 2][(x + nx) / 2] = Cell::Path;
                self.grid[ny][nx] = Cell::Path;

                stack.push((nx, ny));
            }
        }

        // Ensure there is an entrance and an exit
        self.grid[0][0] = Cell::Path; // Entrance
        self.grid[self.height - 1][self.width - 1] = Cell::Path; // Exit
    }

    fn solve(&mut self) -> bool {
        self.solve_recursive(0, 0)
    }

    fn solve_recursive(&mut self, x: usize, y: usize) -> bool {
        if x == self.width - 1 && y == self.height - 1 {
            self.grid[y][x] = Cell::SolvedPath;
            return true;
        }

        if self.grid[y][x] == Cell::Path {
            self.grid[y][x] = Cell::SolvedPath;

            let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for &(dx, dy) in &directions {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);

                if nx < self.width && ny < self.height && self.solve_recursive(nx, ny) {
                    return true;
                }
            }

            self.grid[y][x] = Cell::Path;
        }

        false
    }

    fn display(&self) {
        let mut buffer = String::new();
        buffer.push_str("Maze:\n");

        // Top border
        buffer.push_str("█");
        for _ in 0..self.width {
            buffer.push_str("█");
        }
        buffer.push_str("█\n");

        // Maze rows with side borders
        for row in &self.grid {
            buffer.push_str("█"); // Left border
            for cell in row {
                match cell {
                    Cell::Wall => buffer.push('█'), // Using a solid block character for walls
                    Cell::Path => buffer.push(' '), // Using a space for paths
                    Cell::SolvedPath => buffer.push_str("\x1b[31m█\x1b[0m"), // Using red color for solved path
                }
            }
            buffer.push_str("█\n"); // Right border
        }

        // Bottom border
        buffer.push_str("█");
        for _ in 0..self.width {
            buffer.push_str("█");
        }
        buffer.push_str("█\n");

        println!("{}", buffer);
    }
}

fn main() {
    let mut input = String::new();
    let size: usize;

    loop {
        println!("The maze will be a n*n grid. Please enter an odd value for the dimensions of the generated maze: ");
        io::stdin().read_line(&mut input).expect("Failed to read user input...");
        
        match input.trim().parse::<usize>() {
            Ok(n) if n % 2 != 0 => {
                size = n;
                break;
            }
            _ => {
                println!("Invalid input. Please enter an odd number.");
                input.clear();
            }
        }
    }

    let mut maze = Maze::new(size, size);

    maze.generate();

    println!("Grid after generation:");
    maze.display();

    if maze.solve() {
        println!("Solved maze:");
        maze.display();
    } else {
        println!("No solution found.");
    }
}

