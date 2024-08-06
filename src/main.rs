struct NeighborSum {
    grid: Vec<Vec<i32>>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        NeighborSum { grid }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let mut sum = 0;
        let directions = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ]; // Right, Down, Left, Up

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == value {
                    for &(dx, dy) in &directions {
                        let x = (i as isize) + dx;
                        let y = (j as isize) + dy;
                        if
                            x >= 0 &&
                            x < (self.grid.len() as isize) &&
                            y >= 0 &&
                            y < (row.len() as isize)
                        {
                            sum += self.grid[x as usize][y as usize];
                        }
                    }
                }
            }
        }

        sum
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let mut sum = 0;
        let directions = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ]; // Bottom-right, Bottom-left, Top-right, Top-left

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == value {
                    for &(dx, dy) in &directions {
                        let x = (i as isize) + dx;
                        let y = (j as isize) + dy;
                        if
                            x >= 0 &&
                            x < (self.grid.len() as isize) &&
                            y >= 0 &&
                            y < (row.len() as isize)
                        {
                            sum += self.grid[x as usize][y as usize];
                        }
                    }
                }
            }
        }

        sum
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

fn main() {
    let grid = vec![vec![10, 100, 99], vec![2, 5, 101], vec![55, 46, 78]];
    let obj = NeighborSum::new(grid);
    let ret_1: i32 = obj.adjacent_sum(5);
    let ret_2: i32 = obj.diagonal_sum(5);
    println!("Adjacent sum: {}", ret_1);
    println!("Diagonal sum: {}", ret_2);
}
