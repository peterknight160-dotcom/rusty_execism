// Two ways to do this, one is to create the structure and fill it in the new method,
// the other is to create it in the rows method each time it's called - will do the former.

pub struct PascalsTriangle {
    // Generate the vector of vectors representing Pascal's Triangle up to the given number of rows.
    triangle: Vec<Vec<u32>>,
         
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_count {
            let mut row: Vec<u32> = Vec::new();
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let left_parent = triangle[(i - 1) as usize][(j - 1) as usize];
                    let right_parent = triangle[(i - 1) as usize][j as usize];
                    row.push(left_parent + right_parent);
                }
            }
            triangle.push(row);
        }

        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}

     