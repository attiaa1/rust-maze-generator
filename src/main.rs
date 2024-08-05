use std::io;
use rand::prelude::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
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
                buffer.push(match cell {
                    Cell::Wall => '█', // Using a solid block character for walls
                    Cell::Path => ' ', // Using a space for paths
                });
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
    println!("The maze will be a n*n grid. Please enter a value for the width and height of the generated maze: ");
    io::stdin().read_line(&mut input).expect("Failed to read user input...");
    let size: usize = input.trim().parse().expect("Please enter a number.");

    let mut maze = Maze::new(size, size);

    println!("Grid after initialization:");
    maze.display();

    maze.generate();

    println!("Grid after generation:");
    maze.display();
}
