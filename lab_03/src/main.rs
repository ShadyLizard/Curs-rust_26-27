use crate::CustomError::Not_Ascii_character;

fn main() {
    
    let mut number = 0;

    while let Some(prime) = next_prime(number)
    {
        println!("{}",prime);
        number = prime;
    }

}


fn is_prime (n:u16) -> bool 
{

    if n < 2 {return false};
    let mut prime = true;

    for i in 2..n
    {
        if i*i > n{break;}
        if n%i == 0 
        {
            prime = false;
            break;
        }
    }

    prime

}

fn next_prime (mut n:u16) -> Option<u16> 
{
    if n == u16::MAX {return None;}
    if n<=1 {return Some(2);}

    if n%2==0 
    {
        n+=1;
    }
    else
    {
        n+=2;
    }
    
    loop
    {
        if n == u16::MAX {return None;}
        if is_prime(n) {return Some(n);}
        n+=2;  
    }

}

///////////////////////////// VV EX2 VV /////////////////////////////

fn ch_add(a:u32, b:u32) -> Option<u32>
{

    if a > u32::MAX - b
    {
        panic!("u32({a}) + u32({b}) outgrows u32 type");
    }
    else
    {
        Some(a+b)
    }

}

fn ch_mull( a:u32,b:u32 ) -> Option<u32>
{
    let mut mul = 0;

    for _ in 1..=b
    {
        mul = ch_add(mul, a)?
    }
    
    Some(mul)
}

///////////////////////////// VV EX3 VV /////////////////////////////

fn ch_add_R(a:u32, b:u32) -> Result<u32, &'static str> // nu inteleg exact sintaxa de la &'static str
{

    if a > u32::MAX - b
    {
        return Err("addition overflow");
    }
    else
    {
        return Ok(a+b)
    }

}

fn ch_mull_R( a:u32,b:u32 ) -> Result<u32, &'static str>
{
    let mut mul = 0;

    for _ in 1..=b
    {
        mul = ch_add_R(mul, a)?; // cum pot sa schimb eroarea ?
    }
    
    Ok(mul)
}

///////////////////////////// VV EX4 VV /////////////////////////////

pub enum CustomError
{
    Not_Ascii_character,
    Not_a_Digit,
    Not_a_Bse16_digit,
    Not_a_Letter,
    Not_Printable,
}

fn to_uppercase(ch:char) -> Result<char, CustomError>
{
    if ch <= 'Z' && ch >= 'A'
    {
        return Ok(ch)
    }

    if ch <= 'z' && ch >= 'a'
    {
        let dif = 'a' as u8 - 'A' as u8;

        return Ok( (ch as u8 - dif ) as char )
    }

    return Err(CustomError::Not_a_Letter)
}

fn to_lowercase(ch: char) -> Result<char, CustomError>
{
    if ch >= 'a' && ch <= 'z'
    {
        return Ok(ch);
    }

    if ch >= 'A' && ch <= 'Z'
    {
        let dif = 'a' as u8 - 'A' as u8;
        return Ok((ch as u8 + dif) as char);
    }

    return Err(CustomError::Not_a_Letter);
}

fn print_char(ch: char) -> Result<char, CustomError>
{
    let val = ch as u8;

    if val >= 32 && val <= 126
    {
        println!("{ch}");
        return Ok(ch);
    }

    return Err(CustomError::Not_Printable);
}

fn char_to_number(ch: char) -> Result<u8, CustomError>
{
    if (ch as u8) > 127
    {
        return Err(CustomError::Not_Ascii_character);
    }

    if ch >= '0' && ch <= '9'
    {
        return Ok((ch as u8) - ('0' as u8));
    }

    return Err(CustomError::Not_a_Digit);
}

fn char_to_number_hex(ch: char) -> Result<u32, CustomError>
{
    if (ch as u32) > 127
    {
        return Err(CustomError::Not_Ascii_character);
    }

    if ch >= '0' && ch <= '9'
    {
        return Ok((ch as u32) - ('0' as u32));
    }
    
    if ch >= 'A' && ch <= 'F'
    {
        return Ok((ch as u32) - ('A' as u32) + 10);
    }

    return Err(CustomError::Not_a_Bse16_digit);
}

fn print_error(err: CustomError)
{
    match err
    {
        CustomError::Not_Ascii_character => println!("Err
    }
}