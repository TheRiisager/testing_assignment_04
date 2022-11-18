pub fn has_won(rows: [[char; 3]; 3], symbol: char) -> bool {
    for row in rows {
        if row_win(row, symbol) {
            return true;
        }
    }

    if column_win(rows, symbol) {
        return true;
    }

    if diagonal_win(rows, symbol) {
        return true;
    }

    return false;
}

fn row_win(row: [char; 3], symbol: char) -> bool {
   for field in row {
    if field != symbol {
        return false;
    }
   }
   return true;
}

fn column_win([row1, row2, row3]: [[char; 3]; 3], symbol: char) -> bool {
    let columns: Vec<[char; 3]> = row1.iter()
        .zip(row2.iter())
        .zip(row3.iter())
        .map(|((x, y), z)| [*x, *y, *z])
        .collect();
    
    for column in columns {
        if row_win(column, symbol) {
            return true;
        }
    }

    return false;
}

fn diagonal_win(rows: [[char; 3]; 3], symbol: char) -> bool {
    let mut win_at_first = true;
    for n in 0..=2 {
        if rows[n][n] != symbol {
            win_at_first = false;
        }
    }

    if win_at_first {
        return true;
    }

    let mut count = 2;
    for n in 0..=2 {
        println!("{}", rows[n][count]);
        if rows[n][count] != symbol {
            return false;
        }
        if !(count <= 0) {count = count - 1;}
    }

    return true;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_win_win() {
        let symbol = 'X';
        let winning_row: [char; 3] = [symbol, symbol, symbol];
        assert_eq!(row_win(winning_row, symbol), true);
    }

    #[test]
    fn test_row_win_lose() {
        let symbol = 'X';
        let row: [char; 3] = ['O', symbol, 'O'];
        assert_eq!(row_win(row, symbol), false);
    }

    #[test]
    fn test_column_win_win() {
        let symbol = 'X';
        let rows = [
            ['O', symbol, symbol],
            [symbol, symbol, 'O'],
            ['O', symbol, symbol]
        ];

        assert_eq!(column_win(rows, symbol), true);
    }

    #[test]
    fn test_column_win_lose() {
        let symbol = 'X';
        let rows = [
            ['O', symbol, 'O'],
            [symbol, 'O', 'O'],
            ['O', symbol, symbol]
        ];

        assert_eq!(column_win(rows, symbol), false);
    }

    #[test]
    fn diagonal_win_first_diagonal_win() {
        let symbol = 'X';
        let rows = [
            [symbol, symbol, 'O'],
            [symbol, symbol, 'O'],
            ['O', 'O', symbol]
        ];

        assert_eq!(diagonal_win(rows, symbol), true);
    }

    #[test]
    fn diagonal_win_second_diagonal_win() {
        let symbol = 'X';
        let rows = [
            ['O', symbol, symbol],
            [symbol, symbol, 'O'],
            [symbol, 'O', symbol]
        ];

        assert_eq!(diagonal_win(rows, symbol), true);
    }

    #[test]
    fn diagonal_win_lose() {
        let symbol = 'X';
        let rows = [
            ['O', symbol, symbol],
            [symbol, 'O', 'O'],
            ['O', symbol, symbol]
        ];

        assert_eq!(diagonal_win(rows, symbol), false);
    }
}