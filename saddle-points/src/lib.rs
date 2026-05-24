pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    //todo!("find the saddle points of the following matrix: {input:?}")
    let mut saddle_points = Vec::new();
    for (i, row) in input.iter().enumerate() {   
        //  iterating through rows with index i, row is the current row (i.e a vector of u64) being iterated over
        for (j, &value) in row.iter().enumerate() {
            // iterating through columns with index j, value is the current value being iterated over
            let is_saddle_point = input.
                iter().   // iterating through rows again to check the column condition 
                all(|r| r[j] >= value) // all values in the column must be greater than or equal to the current value
                   && row.iter().
                   all(|&v| v <= value); // all values in the row must be less than or equal to the current value
            if is_saddle_point {
                saddle_points.push((i, j));
            }
        }
    }
    saddle_points
}
