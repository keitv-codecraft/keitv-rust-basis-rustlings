// Een speler, aangegeven met 1, bevindt zich op een 3x3 grid.
// Lege plaatsen in dit grid zijn aangegeven met een 0.
// De speler kan een stap zetten in een van de vier hoofdrichtingen.

// Bijvoorbeeld, de speler start in het midden en doet een stap naar rechts:
// 0 0 0       0 0 0
// 0 1 0  -->  0 0 1
// 0 0 0       0 0 0

// De speler mag niet uit het speelveld lopen, dus als hij nog een stap
// naar rechts wil zetten blijft zijn positie gelijk:
// 0 0 0       0 0 0
// 0 0 1  -->  0 0 1
// 0 0 0       0 0 0

// We beschrijven het grid als een array van 3 arrays, ieder met 3 getallen
// let grid = [
//   [0, 0, 0],
//   [0, 1, 0],
//   [0, 0, 0],
// ];

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn move_player(grid: &mut [[i32; 3]; 3], direction: Direction) {
    // TODO: Implementeer deze functie om de speler 1 stap te laten
    // zetten in de richting `direction` binnen het grid.
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
