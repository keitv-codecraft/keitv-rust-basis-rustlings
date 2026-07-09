enum Direction {
    Up,
    Left,
    Right,
    Down,
}

// Eerste oplossing: expliciet en imperatief
fn move_player(grid: &mut [[i32; 3]; 3], direction: Direction) {
    let mut player_location = None;
    'outer: for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if *cell == 1 {
                player_location = Some((row_index, column_index));
                break 'outer;
            }
        }
    }

    if let Some((row, column)) = player_location {
        grid[row][column] = 0;
        let (row, column) = match direction {
            Direction::Up => {
                if row == 0 {
                    (0, column)
                } else {
                    (row - 1, column)
                }
            }
            Direction::Left => {
                if column == 0 {
                    (row, 0)
                } else {
                    (row, column - 1)
                }
            }
            Direction::Right => {
                if column == 2 {
                    (row, 2)
                } else {
                    (row, column + 1)
                }
            }
            Direction::Down => {
                if row == 2 {
                    (2, column)
                } else {
                    (row + 1, column)
                }
            }
        };
        grid[row][column] = 1;
    }
}

use std::convert::TryFrom;

// Tweede oplossing: iterators en refactoring
fn move_player2(grid: &mut [[i32; 3]; 3], direction: Direction) {
    if let Some((row, column)) = grid.iter().enumerate().find_map(|(row_index, row)| {
        row.iter()
            .enumerate()
            .find(|(_, cell)| **cell == 1)
            .map(|(column_index, _)| (row_index, column_index))
    }) {
        grid[row][column] = 0;
        let (delta_row, delta_column) = match direction {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        };
        if let Ok(row) = i32::try_from(row)
            && let Ok(column) = i32::try_from(column)
            && let Ok(new_row) = usize::try_from((row + delta_row).clamp(0, 2))
            && let Ok(new_column) = usize::try_from((column + delta_column).clamp(0, 2))
        {
            grid[new_row][new_column] = 1;
        }
    }
}

fn main() {
    // De speler begint in het midden van het 3x3 grid
    let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
    move_player(&mut grid, Direction::Up);
    move_player(&mut grid, Direction::Left);
    move_player2(&mut grid, Direction::Down);
    move_player2(&mut grid, Direction::Right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_right() {
        // De speler begint in het midden van het 3x3 grid
        let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        move_player(&mut grid, Direction::Right);
        assert_eq!(grid, [[0, 0, 0], [0, 0, 1], [0, 0, 0]]);
    }

    #[test]
    fn move_left() {
        // De speler begint in het midden van het 3x3 grid
        let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        move_player(&mut grid, Direction::Left);
        assert_eq!(grid, [[0, 0, 0], [1, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn move_up() {
        // De speler begint in het midden van het 3x3 grid
        let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        move_player(&mut grid, Direction::Up);
        assert_eq!(grid, [[0, 1, 0], [0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn move_down() {
        // De speler begint in het midden van het 3x3 grid
        let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        move_player(&mut grid, Direction::Down);
        assert_eq!(grid, [[0, 0, 0], [0, 0, 0], [0, 1, 0]]);
    }

    #[test]
    fn stay_in_bounds() {
        // De speler begint in het midden van het 3x3 grid
        let mut grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        move_player(&mut grid, Direction::Down);
        move_player(&mut grid, Direction::Down);
        assert_eq!(grid, [[0, 0, 0], [0, 0, 0], [0, 1, 0]]);

        move_player(&mut grid, Direction::Right);
        move_player(&mut grid, Direction::Right);
        assert_eq!(grid, [[0, 0, 0], [0, 0, 0], [0, 0, 1]]);

        move_player(&mut grid, Direction::Up);
        move_player(&mut grid, Direction::Up);
        move_player(&mut grid, Direction::Up);
        move_player(&mut grid, Direction::Up);
        assert_eq!(grid, [[0, 0, 1], [0, 0, 0], [0, 0, 0]]);

        move_player(&mut grid, Direction::Left);
        move_player(&mut grid, Direction::Left);
        move_player(&mut grid, Direction::Left);
        move_player(&mut grid, Direction::Left);
        assert_eq!(grid, [[1, 0, 0], [0, 0, 0], [0, 0, 0]]);
    }
}
