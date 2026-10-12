fn main() {
    // LAB 02 EXAMPLES
    //
    // let mut s: String = String::from("str 1 ");

    // println!("s = {s} \n");

    // println!("string length = {} \n",s.len() );

    // println!("concatenations");
    // s += "str 2 ";
    // s.push_str("str_3 ");
    // s.push('4');
    // println!("s = {s} \n");

    // //ways of setting string
    // s = "9876543210".to_string();
    // println!("s changed to {s} ");
    // s = String::from("0123456789");
    // println!("s changed to {s} \n");

    // println!("slice of s {} \n", &s[1..3]);

    // let slice :&str = &s[2..=5];
    // println!("stored slice string refference = {}", slice);

    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        //s = add_chars_n(s, c, 26 - i);
        //ref_add_chars_n(&mut s, c, 26 - i);
        i += 1;
    }

    s = add_space(s, 60);
    s = add_str(s, "I 💚");
    s = add_space(s, 60);

    s = add_chars_n(s, '\n', 1);

    s = add_space(s, 60);
    s = add_str(s, "RUST.");
    s = add_space(s, 60);

    s = add_chars_n(s, '\n', 2);

    s = add_str(
        s,
        "    Most            crate      306_437_968           and     lastest         is ",
    );
    s = add_integer(s, 306437968);
    s = add_str(s, "          and     lastest         is \n");

    s = add_str(
        s,
        "         downloaded        has             downloads     the         version    ",
    );

    s = add_float(s, 2.038);

    s = add_chars_n(s, '\n', 1);

    print!("{}", s);
}

// ---------- P1 ----------
fn add_chars_n(mut s: String, c: char, size: u32) -> String {
    for _i in 1..=size {
        s.push(c);
    }
    s
}

// ---------- P2 ----------
fn _ref_add_chars_n(s: &mut String, c: char, size: u32) {
    for _i in 1..=size {
        s.push(c);
    }
}

// ---------- P3 ----------
fn add_space(mut s: String, size: u32) -> String {
    for _i in 1..=size {
        s.push(' ');
    }
    s
}

fn add_str(mut s: String, z: &str) -> String {
    s.push_str(z);
    s
}

fn add_integer(mut s: String, mut n: u32) -> String {
    let mut spacing: u8 = 0;
    let mut z: String = String::from("");

    while n > 0 {
        let char_num = ((n % 10) as u8 + b'0') as char;
        z.push(char_num);

        spacing += 1;
        if spacing == 3 {
            z.push('_');
            spacing = 0
        }

        n /= 10;
    }

    z = z.chars().rev().collect();

    s.push_str(&z);

    s
}

fn get_comma(n: f32) -> i32 {
    let mut x = n as i32;
    let mut y = n - x as f32;

    let mut intsz: i32 = 0;
    let mut fullsz: i32 = 0;

    while x != 0 {
        x /= 10;
        intsz += 1;
    }

    let mut zero_padding = 3;

    while zero_padding != 0 {
        y *= 10.0;
        if ((y as i32) % 10) == 0 {
            zero_padding -= 1;
        } else {
            zero_padding = 3;
        }

        fullsz += 1;
    }

    fullsz -= 2;

    //print!("fullsz = {} , intsz = {}", fullsz, intsz);

    fullsz - intsz
}

fn add_float(mut s: String, mut n: f32) -> String {
    let mut z: String = String::from("");

    let mut comma_pos = get_comma(n);

    for i in 1..=comma_pos {
        n *= 10.0;
    }

    let mut n: i32 = n as i32;

    while n != 0 {
        let char_num = ((n % 10) as u8 + b'0') as char;
        z.push(char_num);

        comma_pos -= 1;

        if comma_pos == 0 {
            z.push('.');
        }

        n /= 10;
    }

    if (comma_pos >= 0) {
        z.push('0');
    }

    z = z.chars().rev().collect();

    s.push_str(&z);

    s
}
