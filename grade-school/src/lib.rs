use std::collections::BTreeMap;

pub struct School {
    roster: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(graded_students) = self.roster.get_mut(&grade) {
            graded_students.push(student.to_string());
        } else {
            self.roster.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).map(|students| {
            let mut sorted_students = students.clone();
            sorted_students.sort_unstable();
            sorted_students
        })
    }
}

impl Default for School {
    fn default() -> Self {
        School::new()
    }
}
