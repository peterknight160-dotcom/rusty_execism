pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Function that returns the spiral matrix of square size {size}
    let mut matrix = vec![vec![0; size as usize]; size as usize];
     if size == 0 {
        return matrix;
    }
    let mut num = 1;
    let mut left = 0;
    let mut right = size as usize - 1;
    let mut top = 0;
    let mut bottom = size as usize - 1; 
   
    while left <= right && top <= bottom {
        // Fill top row
        for col in left..=right {
            matrix[top][col] = num;
            num += 1;
        }
        top += 1;

        // Fill right column
        for row in top..=bottom {
            matrix[row][right] = num;
            num += 1;
        }
        if right > 0 {
            right -= 1;
        }

        // Fill bottom row (if exists)
        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom][col] = num;
                num += 1;
            }
            bottom -= 1;
        }

        // Fill left column (if exists)
        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}
