pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {

    // Diagram rows are separated by newlines
    let rows: Vec<&str> = diagram.lines().collect();
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let plant_map = |c| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => unreachable!(),
    };
    let student_index = students.iter().position(|&s| s == student).unwrap();
    let mut result = Vec::new();
    for row in &rows {
        let chars: Vec<char> = row.chars().collect();
        let start = student_index * 2;
        result.push(plant_map(chars[start]));
        result.push(plant_map(chars[start + 1]));
    }
    result
    
}
