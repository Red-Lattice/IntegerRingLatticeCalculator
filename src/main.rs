use std::io;

fn main()
{
    manage_first_input();
}

fn manage_first_input()
{
    println!("Welcome! Type 'in' (without the quotes) to calculate the number of lattice points in a circle.");
    println!("Alternatively, type 'circumference' to calculate the number of points on a circle's circumference");
    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");
        
    println!("{:?}", first_input);
    if first_input == ("in\r\n")
    {
        println!("Do you want the circle centered at x = 0 or x = 1/2? (y = 0 for both)");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Failed to read line");
            if second_input == ("0\r\n")
            {
                println!("WARNING: Not recommended to go above 1,000,000 due to processing time (unless you're patient)");
                println!("Origin it is");
                println!("What radius circle?");
                let val = input_value();
                println!("Total lattice points in the circle: {:?}", origin_sum_below(val.clone()));
            }
            else
            {
                println!("WARNING: Not recommended to go above 1,000,000 due to processing time (unless you're patient)");
                println!("1/2 then!");
                println!("What radius circle?");
                let val = input_value() * 2 + 1;
                println!("Total lattice points on the circumference: {:?}", half_sum_below(val.clone()));
            }
    }
    if first_input == ("circumference\r\n")
    {
        println!("Do you want the circle centered at x = 0 or x = 1/2? (y = 0 for both)");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Failed to read line");
            if second_input == ("0\r\n")
            {
                println!("WARNING: Not recommended to go above 1,000,000 due to processing time (unless you're patient)");
                println!("Origin it is");
                println!("What radius circle?");
                let val = input_value();
                let fac_vec = factors(val.clone());
                println!("Total lattice points on the circumference: {:?}", circum_lattice_points_origin(fac_vec.clone()));
            }
            else
            {
                println!("WARNING: Not recommended to go above 1,000,000 due to processing time (unless you're patient)");
                println!("1/2 then!");
                println!("What radius circle?");
                let val = input_value() * 2 + 1;
                let fac_vec = factors(val.clone());
                println!("Total lattice points on the circumference: {:?}", circum_lattice_points_halfx(fac_vec.clone()));
            }
    }
}

fn input_value() -> u64
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) =>  return i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    return 1;
}

fn origin_sum_below(num: u64) -> u64
{
    let mut total_pts = 0;
    let mut i = num;
    while i > 0
    {
        total_pts += circum_lattice_points_origin(factors(i));
        i -= 1;
    }
    return total_pts;
}

fn half_sum_below(num: u64) -> u64
{
    let mut total_pts = 0;
    let mut i = num;
    while i > 0
    {
        total_pts += circum_lattice_points_halfx(factors(i));
        i -= 1;
    }
    return total_pts;
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
    let mut x = 2;

    let mut i = 0;

    // This makes it uglier, but we can skip all odd numbers now
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
    x += 1;
    
    // If it has exceeded the sqrt of n then n is a prime
    while n > 1 && x < ((n as f64).sqrt().round() as u64) + 1
    {
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
        x += 2;
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
            total *= vec[i + 1] + 1;
        }
        i += 2;
    }
    return (total>>1).try_into().unwrap();
}

fn circum_lattice_points_halfx(vec: Vec<u64>) -> u64
{
    return 4 * (int_sided_rtriangles(vec)) + 2;
}