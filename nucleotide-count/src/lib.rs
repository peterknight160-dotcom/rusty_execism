use std::collections::HashMap;



pub fn count(nucleotide: char, dna: &str) -> Result<usize,char> {
   // Count the number of occurrences of `nucleotide` in `dna`, returning an error if `nucleotide` is not one of 'A', 'C', 'G', or 'T'.
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {
            let count_map = nucleotide_counts(dna)?;
            Ok(*count_map.get(&nucleotide).unwrap())
        },
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut count_map: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();

    
    for c in dna.chars() {
        if !['A', 'C', 'G', 'T'].contains(&c) {
            return Err(c);
        }
        *count_map.entry(c).or_insert(0) += 1;
    }
    Ok(count_map)
}


// Scratch
/*

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();

    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }

    Ok(counts)
}
*/