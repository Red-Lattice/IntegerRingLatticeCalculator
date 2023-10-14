use std::io;

fn main()
{
    fact();
}

fn fact()
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) => println!("{:?}", circum_lattice_points_origin(factors(i))),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn circum_lattice_points_origin(vec: Vec<u64>) -> u64
{
    let mut r = 1;
    let mut i = 0;
    while i < vec.len()
    {
        if vec[i] % 4 == 1
        {
            r *= 2*(vec[i + 1]) + 1;
        }
        i += 2;
    }
    return 4 * r;
}

pub fn factors(mut n: u64) -> Vec<u64> 
{
    let mut factors = vec![];
    let mut x = 1;

    while n > 1 
    {
        x += 1;
        let mut i = 0;

        while n % x == 0 
        {
            n /= x;
            i += 1;
        }
        if i > 0
        {
            factors.push(x);
            factors.push(i);
        }
    }
    
    factors
}

//***This code is no longer in use***
pub fn factors2_d(mut n: u64) -> Vec<Vec<u64>> 
{
    let mut factors = vec![];
    let mut x = 1;

    while n > 1 
    {
        x += 1;
        let mut i = 0;

        while n % x == 0 
        {
            n /= x;
            i += 1;
        }
        let vec = vec![x, i];
        if i > 0
        {
            factors.push(vec);
        }
    }
    
    factors
}