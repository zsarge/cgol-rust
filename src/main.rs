use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Square {
    Dead,
    Alive,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Square::Alive => 'X',
            Square::Dead => ' ',
        };
        write!(f, "{}", printable)
    }
}

struct Board {
    height: i32,
    width: i32,
    squares: [[Square; 80]; 10],
}

impl Board {
    fn print_header(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }

    fn show(&self) {
        self.print_header();
        for line in self.squares {
            print!("|");
            for square in line {
                print!("{}", square);
            }
            println!("|");
        }
        self.print_header();
    }

    fn wrap(&self, mut x: i32, mut y: i32) -> (usize, usize) {
        if y < 0 {
            y = self.height + y;
        }
        if x < 0 {
            x = self.width + x;
        }
        if y >= self.height {
            y = self.height - y;
        }
        if x >= self.width {
            x = self.width - x;
        }
        (x as usize, y as usize)
    }

    fn set(&mut self, x: usize, y: usize, value: Square) {
        self.squares[y][x] = value;
    }

    fn get(&self, x: i32, y: i32) -> Square {
        let (xx, yy) = self.wrap(x, y);
        self.squares[yy][xx]
    }

    fn get_number_of_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut n = 0;
        // dx and dy are deltas from the given (x, y) point,
        // Here is a graph:
        /*
          (x-1, y-1) | (x, y-1) | (x+1, y-1)
          (x-1, y  ) | (x, y  ) | (x+1, y  )
          (x-1, y+1) | (x, y+1) | (x+1, y+1)
        */
        for dx in -1..=1 {
            for dy in -1..=1 {
                if x + dx != x || y + dy != y {
                    if self.get(x + dx, y + dy) == Square::Alive {
                        n += 1;
                    }
                }
            }
        }
        n
    }
}

fn main() {
    let mut b = Board {
        width: 80,
        height: 10,
        squares: [[Square::Dead; 80]; 10],
    };
    b.set(5, 5, Square::Alive);
    b.set(6, 6, Square::Alive);
    b.set(6, 7, Square::Alive);
    b.set(5, 7, Square::Alive);
    b.set(4, 7, Square::Alive);
    b.show();
    println!("{}", b.get(5, 5));
    println!("{}", b.get_number_of_neighbors(4, 4));
}
