use std::ops::{Index, IndexMut};
use rand::random;

// 空きセルは 0
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

impl Grid {
    fn get_block_occurences(&self, block_index: usize) -> Vec<u8> {
        let base_row = (block_index / 3) * 3;
        let base_col = (block_index % 3) * 3;

        let mut rtnVec = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                let row = base_row + i;
                let col = base_col + j;

                let value = self[(row, col)];

                rtnVec.push(value);
            }
        }

        rtnVec
    }

    fn get_block_absences(&self, block_index: usize) -> Vec<u8> {
        let occurences = self.get_block_occurences(block_index);

        let mut rtnVec = Vec::new();

        for i in 1..10 {
            if !occurences.contains(&i) {
                rtnVec.push(i);
            }
        }

        rtnVec
    }
}

type Population = Vec<Grid>;

/*fn generate_random_grid(clues: Grid) -> Population {
    for block_index in 0..9 {
        let rng = random::<u8>();

        let absences = clues.get_block_absences(block_index);
        let pick_index = rng % (absences.len() as u8) as usize;

        absences[pick_index]
    }
}*/

fn main() {
    println!("Hello, world!");
}
