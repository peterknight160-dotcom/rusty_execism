use std::collections::{HashMap, HashSet};
pub struct School {
    students: HashMap<u32, HashSet<String>>, // A HashMap where the key is the grade and the value is a HashSet of student names
}

impl School {
    pub fn new() -> School {
        School  {
            students: HashMap::new(), // A HashMap where the key is the grade and the value is a HashSet of student names
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // Check if the student is already in any grade 
        for (_existing_grade, existing_students) in &self.students {
            if existing_students.contains(student) {
                // If student is already in a grade, do not add them to the new grade
                return;
            }
        }
        // Add student to the specified grade
        self.students.entry(grade).or_insert_with(HashSet::new).insert(student.to_string());

  

    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.keys().cloned().collect();
        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {

        match self.students.get(&grade) {
            Some(students) => {
                let mut students_vec: Vec<String> = students.iter().cloned().collect();
                students_vec.sort_unstable();
                students_vec
            },
            None => Vec::new(),
        }
    }
}
