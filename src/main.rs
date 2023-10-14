use std::io;

fn main()
{
    let facVec = fact();
    println!("{:?}", circum_lattice_points_origin(facVec.clone()));
    println!("{:?}", circum_lattice_points_halfx(facVec));
}

fn fact() -> Vec<u64>
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) =>  return factors(i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    return vec![];
}
//***(0, 0) Code***
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
//***(1/2, 0) Code***
fn int_sided_rtriangles(vec: Vec<u64>) -> u64
{
    let mut total = 1;
    let mut i = 0;
    while i < vec.len()
    {
        if vec[i] & 3 == 1
        {
            total *= (i + 1) + 1;
        }
        i += 2;
    }
    return (total>>1).try_into().unwrap();
}

fn circum_lattice_points_halfx(vec: Vec<u64>) -> u64
{
    return 4 * (int_sided_rtriangles(vec)) + 2;
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