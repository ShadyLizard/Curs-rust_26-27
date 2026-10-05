
fn main() {
    let rabbits = 5;
    let cap = 100;
    println!("hello world i have {} rabbits", rabbits);

    for i in 1..=cap  
    {
        print_prime(i);
    }

    for i in 0..=cap  
    {
        for j in i+1..=cap  
        {
            print_coprime(i,j);
        }
    }

    bottles_song(10);
}

fn print_prime(x:i32)
{

    if x == 1
    {
        return
    }

    if x == 2 
    {
        println!("2");
        return
    }

    let mut prime = true;

    let mut i=2;
    while i*i <= x
    {
        if x%i == 0
        {
            prime = false;
            break;
        }
        i = i+1;
    }

    if prime
    {
        println!("{}",x);
    }
}

fn print_coprime(x:i32, y:i32)
{
    let mut temp : i32;
    let mut a : i32 =x;
    let mut b : i32 =y;
    while b != 0
    {
        temp = b;
        b = a % b;
        a = temp;
    }
    if a == 1
    {
        println!("{} and {} are coprime",x,y);
    }
}

fn bottles_song(mut bottles:i32)
{

    //am inteles ca nu este recomandata recursivitatea in cazuri de genul

    if bottles != 0
    {
        println!("{} bottles of beer on the wall",bottles);
        println!("{} bottles of beer",bottles);
        println!("take one down, pass it arround");
        bottles = bottles-1;
        println!("{} bottles of beer on the wall \n",bottles);
        bottles_song(bottles);
    }
    else
    {return}
}
