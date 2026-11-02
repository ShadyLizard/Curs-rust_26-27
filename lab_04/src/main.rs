use std::{io, fs};


fn main() {
    do_stuff();
}

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/inputfile.txt")?;

    let mut char_count : u32 = 0;


    print_most_bytes_line(&s);
    print_most_chars_line(&s);

    //s::write("src/copy.rs", &s)?;

    Ok(())
}


fn print_most_bytes_line( textfile : &String ) -> ()
{
    let mut max_bytes = 0;
    let mut line_index = 0;

    let mut current_index = 0;

    for line in textfile.lines()
    {
        let current_bytes = line.len();

        if current_bytes > max_bytes
        {
            max_bytes = current_bytes;
            line_index = current_index;
        }
        current_index +=1;
    }
    
    current_index = 0;
    for line in textfile.lines()
    {
        if current_index == line_index
        {
            println!("{line}");
        }
        current_index +=1;
    }

}


fn print_most_chars_line( textfile : &String ) -> ()
{
    let mut max_char_count = 0;
    let mut line_index = 0;

    let mut current_index = 0;

    for line in textfile.lines()
    {
        let current_char_count = line.chars().count();

        if current_char_count > max_char_count
        {
            max_char_count = current_char_count;
            line_index = current_index;
        }
        current_index +=1;
    }
    
    current_index = 0;
    for line in textfile.lines()
    {
        if current_index == line_index
        {
            println!("{line}");
        }
        current_index +=1;
    }

}

//DEJA STIU CA URASC ABORDAREA DE MAI SUS