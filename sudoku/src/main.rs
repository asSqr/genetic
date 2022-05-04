use std::ops::{Index, IndexMut};

struct Grid([[u8; 9]; 9]);

type Index2d = (usize, usize);

impl Index<Index2d> for Grid {
    type Output = u8;

    fn index(&self, index: Index2d) -> &Self::Output {
        let grid = &self.0;
        &grid[index.0][index.1]
    }
}

impl IndexMut<Index2d> for Grid {
    fn index_mut(&mut self, index: Index2d) -> &mut Self::Output {
        let mut grid = &mut self.0;
        &mut grid[index.0][index.1]
    }
}

type Population = Vec<Grid>;

fn generate_initial_population() -> Population {
    
}

fn main() {
    println!("Hello, world!");
}
