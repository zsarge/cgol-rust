use std::fmt;

#[derive(Debug, Copy, Clone)]
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

    fn get(mut self, x: i32, y: i32) -> Square {
        let (xx, yy) = self.wrap(x,y);
        self.squares[yy][xx]
    }
}


fn main() {
    let mut b = Board {
        width: 80,
        height: 10,
        squares: [[Square::Dead; 80]; 10],
    };
    b.set(5,5, Square::Alive);
    b.show();
    println!("{}", b.get(5,5));
}
