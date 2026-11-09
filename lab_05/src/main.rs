use std::{fs, io};

struct Student
{
    name : String,
    phone : String,
    age : u8
}

fn main() {
    let mut matrix = [[' '; 55]; 100];

    paint(0,0,'B',& mut matrix);

    for y in 0..matrix.len()
    {
        for x in 0..matrix[y].len()
        {
            print!("{}",matrix[y][x]);
        }
        print!("\n");
    }

    let mut s = [];

    read_student_data(&mut s);

}

fn paint(x:usize, y:usize, ch:char, canvas : &mut[[char;55];100])
{
    canvas[y][x] = ch;
}



//urmeaza sa fac cu marime fixa , array de option<Student>
fn read_student_data(students: &mut[Student]) -> Result<(), io::Error >
{
    let textfile = fs::read_to_string("src/student_data.txt")?;
    let array_size = textfile.lines().count();
    let students = [Student; array_size];
{
    for line in textfile.lines()
    {
        let mut data = line.split(',');

        let name : String = data.next()?.to_string();
        let phone : String = data.next()?.to_string();
        let age = data.next()?.parse::<u8>().ok()?;

        let s :Student = Student{name,phone,age};
        println!("{name}{phone}{age}");
    }
    
    Ok(())
}

//de terminat