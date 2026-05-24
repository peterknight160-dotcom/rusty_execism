
use std::collections::HashMap;

fn build_codon_table() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", "STOP"),
        ("UAG", "STOP"),
        ("UGA", "STOP"),
    ])
}



pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let codon_table    : HashMap<&str, &str> = build_codon_table();
    
    let mut proteins = Vec::new();
    for codon in rna.as_bytes().chunks(3) {
        let codon_str = std::str::from_utf8(codon).ok()?;
        if let Some(&protein) = codon_table.get(codon_str) {
            if protein == "STOP" {
                break;
            }
            proteins.push(protein);
        } else {
            return None; // Invalid codon
        }
    }
    Some(proteins)
    
}
