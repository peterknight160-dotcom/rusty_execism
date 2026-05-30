#[derive(PartialEq, Eq, Debug)]

pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // Check if the goal is greater than both buckets' capacities
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }   
    // Check if the goal is a multiple of the GCD of the two capacities
    if goal % gcd(capacity_1, capacity_2) != 0 {
        return None;
    }
    // Check if the goal is equal to one of the bucket capacities
    if goal == capacity_1 {
        return Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::One,
            other_bucket: 0,
        });
    }
    if goal == capacity_2 {
        return Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,    
        });
    }
    //  Need to use the Extended Euclidean Algorithm to find the number of moves
    // bucket1 × x + bucket2 × y = goal
   let (x, y, moves) = extended_euclid(capacity_1, capacity_2, goal)?;
    // Determine which bucket is the goal bucket and how many liters are left in the other bucket
    let (goal_bucket, other_bucket) = match start_bucket {
        Bucket::One => {
            if x > 0 {
                (Bucket::One, (y * capacity_2 as i8).abs() as u8)
            } else {
                (Bucket::Two, (x * capacity_1 as i8).abs() as u8)
            }   
        }
        Bucket::Two => {
            if y > 0 {
                (Bucket::Two, (x * capacity_1 as i8).abs() as u8)
            } else {
                (Bucket::One, (y * capacity_2 as i8).abs() as u8)
            }
        }
    };
    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })


}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}   
fn extended_euclid( b1:  u8, b2: u8, Option<(i8, i8, u8)> {
    //
    let mut s = 0;
    let mut old_s = 1;  
    let mut t = 1;
    let mut old_t = 0;  
    let mut r = b2 as i8;
    let mut old_r = b1 as i8;   
    let mut moves = 0;
    while r != 0 {
        moves += 1;
        let quotient = old_r / r;
        let temp_r = r;
        r = old_r - quotient * r;
        old_r = temp_r;
        let temp_s = s;
        s = old_s - quotient * s;
        old_s = temp_s;
        let temp_t = t;
        t = old_t - quotient * t;
        old_t = temp_t;
    }
    
    Some((old_s, old_t, moves))
}