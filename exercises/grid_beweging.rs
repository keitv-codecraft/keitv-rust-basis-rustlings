enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn move_player(grid: &mut [[i32; 3]; 3], direction: Direction) {
    // TODO Implementeer de functie
}

fn main() {}

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
