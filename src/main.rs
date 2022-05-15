use std::time::Instant;

fn print_result(result: [[u8; 9]; 9]) {
    let separator = "-------------------------";
    println!("{}", separator);

    for (row_index, row) in result.iter().enumerate() {
        for (index, col) in row.iter().enumerate() {
            if index == 0 {
                print!("| ");
            }
            print!("{} ", col);
            // Print column separator after each square
            if ((index + 1) % 3) == 0 {
                print!("| ");
            }
        }
        println!();
        // Print row separator below each square
        if ((row_index + 1) % 3) == 0 {
            println!("{}", separator);
        }
    }
}

fn possible(puzzle: [[u8; 9]; 9], x: usize, y: usize, n: u8) -> bool {
    if puzzle[x][y] != 0 {
        return false;
    }
    // Check that n does not appear in row or column
    for i in 0..9 {
        if puzzle[x][i] == n || puzzle[i][y] == n {
            return false;
        }
    }

    // Check that n does not appear in square
    let x_start = (x / 3) * 3;
    let y_start = (y / 3) * 3;
    for i in x_start..x_start + 3 {
        for j in y_start..y_start + 3 {
            if puzzle[i][j] == n {
                return false;
            }
        }
    }
    return true;
}

fn next_empty_cell(puzzle: [[u8; 9]; 9]) -> Option<(usize, usize)> {
    for x in 0..9 {
        for y in 0..9 {
            if puzzle[x][y] == 0 {
                return Some((x, y));
            }
        }
    }
    return None;
}

fn is_solved(puzzle: [[u8; 9]; 9]) -> bool {
    for x in 0..9 {
        for y in 0..9 {
            if puzzle[x][y] == 0 {
                return false;
            }
        }
    }
    true
}

fn guesses(puzzle: [[u8; 9]; 9], x: usize, y: usize) -> Vec<u8> {
    let mut guesses: Vec<u8> = Vec::new();

    for n in 1..10 {
        if possible(puzzle, x, y, n) {
            guesses.push(n);
        }
    }
    guesses
}

fn solve(puzzle: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    let mut result: [[u8; 9]; 9] = puzzle;

    let next_cell = next_empty_cell(puzzle);

    if next_cell.is_some() {
        let (x, y) = next_cell.unwrap();
        let guesses = guesses(result, x, y);

        for guess in guesses {
            result[x][y] = guess;
            result = solve(result);
            if is_solved(result) {
                return result;
            }
        }
    }

    puzzle
}

fn main() {
    // Start timer to calculate execution time
    let start = Instant::now();

    // Define a puzzle as an array of arrays
    // let puzzle: [[u8; 9]; 9] = [
    //     [0, 0, 4, 6, 7, 8, 9, 1, 2],
    //     [6, 7, 2, 1, 9, 5, 3, 4, 8],
    //     [1, 9, 8, 3, 4, 2, 5, 6, 7],
    //     [8, 5, 9, 7, 6, 1, 4, 2, 3],
    //     [4, 2, 6, 8, 5, 3, 7, 9, 1],
    //     [7, 1, 3, 9, 2, 4, 8, 5, 6],
    //     [9, 6, 1, 5, 3, 7, 2, 8, 4],
    //     [2, 8, 7, 4, 1, 9, 6, 3, 5],
    //     [3, 4, 5, 2, 8, 6, 1, 7, 9],
    // ];

    let puzzle: [[u8; 9]; 9] = [
        [0, 0, 8, 0, 0, 2, 6, 0, 3],
        [0, 3, 0, 6, 0, 0, 0, 0, 9],
        [6, 0, 4, 0, 0, 3, 0, 2, 0],
        [7, 6, 1, 0, 0, 9, 4, 0, 0],
        [0, 8, 0, 1, 0, 4, 0, 6, 0],
        [0, 0, 2, 7, 0, 0, 3, 1, 5],
        [0, 2, 0, 9, 0, 0, 1, 0, 4],
        [3, 0, 0, 0, 0, 8, 0, 7, 0],
        [1, 0, 6, 3, 0, 0, 2, 0, 0],
    ];

    // solve the puzzle
    let result = solve(puzzle);

    // Stop timer
    let duration = start.elapsed();

    println!("Solution found in {:?}", duration);
    print_result(result);
}
