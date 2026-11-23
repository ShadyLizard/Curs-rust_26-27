trait my_cmd
{
    fn get_name(&self) -> &str;
    fn exec(arguments : &str);
}

struct ping
{
    name : String
}

impl my_cmd for ping
{
    fn get_name(&self) -> &str
    {
        &self.name
    }

    fn exec(arguments : &str)
    {
        println!("pong");
    }
}

struct count
{
    name : String
}

impl my_cmd for count
{
    fn get_name(&self) -> &str
    {
        &self.name
    }

    fn exec(arguments : &str)
    {
        let mut x=0;

        for argument in arguments.split_whitespace()
        {
            x +=1;
        }   
        println!("counted {x} arguments");
    }
}

struct times
{
    name : String ,
    times_called : u32 = 0,
}

impl my_cmd for times
{
    fn get_name(&self) -> &str
    {
        &self.name
    }

    fn exec(arguments : &str)
    {
        times_called += 1;
        println!("i have been called {times_called}");
    }
}

fn main() 
{
    let p = ping
    {
        name:"ping".to_string(),
    };
    println!("{x}",x = p.get_name() );

    ping::exec("it feels like this should have no arguments");


    let c = count
    {
        name:"count".to_string(),
    };
    println!("{x}",x = c.get_name() );
    count::exec("a b c d");
}
//de terminat