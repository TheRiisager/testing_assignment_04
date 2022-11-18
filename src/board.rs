use std::fmt;
use rand::{thread_rng, Rng};

pub struct Board {
    pub rows: [[char; 3]; 3]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first = format!("{} | {} | {}", self.rows[0][0], self.rows[0][1], self.rows[0][2]);
        let second = format!("{} | {} | {}", self.rows[1][0], self.rows[1][1], self.rows[1][2]);
        let third = format!("{} | {} | {}", self.rows[2][0], self.rows[2][1], self.rows[2][2]);
        write!(f, "{}\n{}\n{}", first, second, third)
    }
}

impl Board {
    pub fn make_move(&mut self, row: i32, field: i32, symbol: char) -> bool {
      if row > 2 || row < 0  {
        println!("invalid input!");
        return false;
      }  
      if field > 2 || field < 0  {
        println!("invalid input!");
        return false;
      }

      return self.place_if_valid(row as usize, field as usize, symbol);
    }

    fn place_if_valid(&mut self, row: usize, field: usize, symbol: char) -> bool {
        if self.rows[row][field] != ' ' {
            return false;
        }
        self.rows[row][field] = symbol;
        return true;
    }

    pub fn cpu_turn(&mut self) {
        let mut rng = thread_rng();
        let row: usize = rng.gen_range(0..=2);
        let column: usize = rng.gen_range(0..=2);
        if !self.place_if_valid(row, column, 'O') {
            self.cpu_turn()
        }
    }
}