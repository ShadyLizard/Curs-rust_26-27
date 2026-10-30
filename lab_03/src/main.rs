fn main() {
    let mut number = 0;

    while let Some(prime) = next_prime(number) {
        println!("{}", prime);
        number = prime;
    }

    if let Some(x) = ch_add(3, 5) {
        println!("3 + 5 = {x}");
    }
    if let Some(x) = ch_mull(3, 5) {
        println!("3 * 5 = {x}");
    }

    match ch_add_r(3, 5) {
        Ok(x) => println!("3 + 5 = {x}"),
        Err(y) => println!("Error: {y}"),
    };
    match ch_mull_r(3, 5) {
        Ok(x) => println!("3 * 5 = {x}"),
        Err(y) => println!("Error: {y}"),
    };

    //LOWERCASE TEST
    match to_lowercase('A') {
        Ok(x) => println!("lowercase of A is {x}"),
        Err(y) => print_error(y), // nu mergea cu println!
    };
    match to_lowercase('3') {
        Ok(x) => println!("{x}"),
        Err(y) => print_error(y), // nu mergea cu println!
    };

    //UPPERCASE TEST
    match to_uppercase('b') {
        Ok(x) => println!("uppercase of b is {x}"),
        Err(y) => print_error(y), // nu mergea cu println!
    };
    match to_uppercase('3') {
        Ok(x) => println!("{x}"),
        Err(y) => print_error(y), // nu mergea cu println!
    };

    //print char test
    if let Err(y) = print_char('C') {
        print_error(y);
    }
    if let Err(y) = print_char('°') {
        print_error(y);
    }

    //text to digit test
    match char_to_number('7') {
        Ok(x) => println!("the char 7 is the digit {x} in base 10"),
        Err(y) => print_error(y), // nu mergea cu println!
    };
    match char_to_number('Z') {
        Ok(x) => println!("the char 7 is the digit {x} in base 10"),
        Err(y) => print_error(y), // nu mergea cu println!
    };

    //text to base 16 digit test
    match char_to_number_hex('C') {
        Ok(x) => println!("the char C is the digit {x} in base 16"),
        Err(y) => print_error(y), // nu mergea cu println!
    };
    match char_to_number_hex('Z') {
        Ok(x) => println!("the char C is the digit {x} in base 16"),
        Err(y) => print_error(y), // nu mergea cu println!
    };

    //get score test
    print_player_score("Alin");
    print_player_score("Miruna");
    print_player_score("Mircea");
}

fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    };
    let mut prime = true;

    for i in 2..n {
        if i >= 256 || i * i > n {
            break;
        } // iese din bounds daca i >=256
        if n.is_multiple_of(i) {
            prime = false;
            break;
        }
    }
    prime
}

fn next_prime(mut n: u16) -> Option<u16> {
    if n == u16::MAX {
        return None;
    }
    if n <= 1 {
        return Some(2);
    }

    if n.is_multiple_of(2) {
        n += 1;
    } else {
        n += 2;
    }

    loop {
        if n == u16::MAX {
            return None;
        }
        if is_prime(n) {
            return Some(n);
        }
        n += 2;
    }
}

///////////////////////////// VV EX2 VV /////////////////////////////

fn ch_add(a: u32, b: u32) -> Option<u32> {
    if a > u32::MAX - b {
        panic!("u32({a}) + u32({b}) outgrows u32 type");
    } else {
        Some(a + b)
    }
}

fn ch_mull(a: u32, b: u32) -> Option<u32> {
    let mut mul = 0;

    for _ in 1..=b {
        mul = ch_add(mul, a)? // o problema ar fii ca primeste panic-ul inmultirii si nu unul separat
    }

    Some(mul)
}

///////////////////////////// VV EX3 VV /////////////////////////////

fn ch_add_r(a: u32, b: u32) -> Result<u32, &'static str> // nu inteleg exact sintaxa de la &'static str
{
    if a > u32::MAX - b {
        Err("addition overflow")
    } else {
        Ok(a + b)
    }
}

fn ch_mull_r(a: u32, b: u32) -> Result<u32, &'static str> {
    let mut mul = 0;

    for _ in 1..=b {
        mul = ch_add_r(mul, a)?; // cum pot sa schimb eroarea ?
    }

    Ok(mul)
}

///////////////////////////// VV EX4 VV /////////////////////////////

pub enum CustomError {
    NotAsciiCharacter,
    NotADigit,
    NotABase16Digit,
    NotALetter,
    NotPrintable,

    MissingScore,
}

fn to_uppercase(ch: char) -> Result<char, CustomError> {
    if ch.is_ascii_uppercase() {
        //ma cearta clippy daca nu folosesc ch.is_ascii_uppercase()
        return Ok(ch);
    }

    if ch.is_ascii_lowercase() {
        let dif: u8 = b'a' - b'A';

        return Ok((ch as u8 - dif) as char);
    }

    Err(CustomError::NotALetter)
}

fn to_lowercase(ch: char) -> Result<char, CustomError> {
    if ch.is_ascii_lowercase() {
        return Ok(ch);
    }

    if ch.is_ascii_uppercase() {
        //ma cearta clippy daca nu folosesc ch.is_ascii_uppercase()
        let dif = b'a' - b'A';
        return Ok((ch as u8 + dif) as char);
    }

    Err(CustomError::NotALetter)
}

fn print_char(ch: char) -> Result<(), CustomError> {
    //nu stiu cum trebuie facuta functia asta luand in considerare ca ar trebuii sa poata sa printeze orice caracter UTF8
    let val = ch as u8;

    if (32..=126).contains(&val) {
        println!("{ch}");
        return Ok(());
    }

    Err(CustomError::NotPrintable)
}

fn char_to_number(ch: char) -> Result<u8, CustomError> {
    if (ch as u8) > 127 {
        return Err(CustomError::NotAsciiCharacter);
    }

    if ch.is_ascii_digit() {
        //ma cearta cargo clippy daca nu folosesc functia ch.is_ascii_digit()
        return Ok((ch as u8) - (b'0'));
    }

    Err(CustomError::NotADigit)
}

fn char_to_number_hex(ch: char) -> Result<u8, CustomError> {
    if (ch as u8) > 127 {
        return Err(CustomError::NotAsciiCharacter);
    }

    if ch.is_ascii_digit() {
        //ma cearta cargo clippy daca nu folosesc functia ch.is_ascii_digit()
        return Ok((ch as u8) - (b'0'));
    }

    if ('A'..='F').contains(&ch) {
        return Ok((ch as u8) - (b'A' + 10_u8));
    }

    Err(CustomError::NotABase16Digit)
}

fn print_error(err: CustomError) {
    match err {
        CustomError::NotAsciiCharacter => println!("Error: Not an ASCII character"),
        CustomError::NotADigit => println!("Error: Not a digit"),
        CustomError::NotABase16Digit => println!("Error: Not a base16 digit"),
        CustomError::NotALetter => println!("Error: Not a letter"),
        CustomError::NotPrintable => println!("Error: Not printable"),

        CustomError::MissingScore => println!("Person score not registered"),
    }
}

///////////////////////////// VV EX5 VV /////////////////////////////

fn get_score(name: &str) -> Option<i32> {
    match name {
        "Alin" => Some(95),
        "Matei" => Some(120),
        "Miruna" => Some(140),
        _ => None,
    }
}

fn print_player_score(player: &str) {
    match get_score(player) {
        Some(x) => println!("{player}'s score is {x}"),
        None => print_error(CustomError::MissingScore), // nu mergea cu println!
    }
}
