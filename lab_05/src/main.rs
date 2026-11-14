use serde_derive::Deserialize;
use std::{fs};

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}

fn main() {
    //ex1

    let mut s: [Option<Student>; 100] = std::array::from_fn(|_| None);
    read_student_data(&mut s);

    let mut oldest_student: Option<&Student> = None;
    let mut youngest_student: Option<&Student> = None;
    let mut max_age = 0;
    let mut min_age = 255;

    for student_option in &s {
        if let Some(student) = student_option {
            if oldest_student.is_none() || student.age > max_age {
                max_age = student.age;
                oldest_student = Some(student);
            }

            if youngest_student.is_none() || student.age < min_age {
                min_age = student.age;
                youngest_student = Some(student);
            }
        }
    }

    if let Some(youngest) = youngest_student {
        println!("the youngest student is {}", youngest.name);
    }
    if let Some(oldest) = oldest_student {
        println!("the oldest student is {}", oldest.name);
    }

    // ^ still seems overkill that i need to write all of these checks and extra variables ^

    //ex2

    let mut matrix = [[' '; 55]; 100];
    paint(0, 0, 'O', &mut matrix);
    paint(3, 0, 'O', &mut matrix);
    paint(0, 2, 'B', &mut matrix);
    paint(1, 3, 'B', &mut matrix);
    paint(2, 3, 'B', &mut matrix);
    paint(3, 2, 'B', &mut matrix);

    for row in matrix {
        for y in row {
            print!("{}", y);
        }
        println!()
    }

    //ex3

   //ex3

    let content = fs::read_to_string("student_data.json").unwrap();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[" || trimmed == "]" || trimmed.is_empty() {
            continue;
        }

        let clean_line = trimmed.strip_suffix(',').unwrap_or(trimmed);

        let s: Student = serde_json::from_str(clean_line).unwrap();
        println!("{:?}", s);
        
        //honestly , ma bate,  incerc sa revin maine

    }

}

fn paint(x: usize, y: usize, ch: char, canvas: &mut [[char; 55]; 100]) {
    canvas[y][x] = ch;
}

//urmeaza sa fac cu marime fixa , array de option<Student>
fn read_student_data(students: &mut [Option<Student>]) {
    let textfile = fs::read_to_string("src/student_data.txt").expect("could not find file");
    let mut student_index: usize = 0;

    for line in textfile.lines() {
        if student_index >= students.len() {
            println!("too many students , only first 100 students have been read");
            break;
        }

        let mut data = line.split(',');

        let name = match data.next() {
            Some(n) => n.to_string(),
            None => continue,
        };

        let phone = match data.next() {
            Some(p) => p.to_string(),
            None => continue,
        };

        let age = match data.next().and_then(|a| a.parse::<u8>().ok()) {
            Some(a) => a,
            None => continue,
        };

        let s: Student = Student { name, phone, age };

        students[student_index] = Some(s);
        student_index += 1;
    }
}