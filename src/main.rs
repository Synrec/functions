use std::io;
use std::cmp::Ordering;

fn main () {
    const FIXED_VALUE: u32 = 9;
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");
    let value: u32 = match value.trim().replace(" ", "").parse(){
        Ok(num) => num,
        Err(_) => return()
    };
    let value = sent_value(value);
    println!("The value now is: {value}");
    match value.cmp(&FIXED_VALUE){
        Ordering::Less => lesser_func(value),
        Ordering::Greater => greater_func(value),
        Ordering::Equal => equal_func(value)
    }
}

fn sent_value (x: u32) -> u32 {
    println!("Value sent over is {x}");
    return x * x;
}

fn lesser_func(x: u32){
    println!("{x} is too small.");
    match x.cmp(&4){
        Ordering::Less => println!("Seriously, that number is waaaaay too small!"),
        Ordering::Greater => println!("Just a bit higher next time!"),
        Ordering::Equal => println!("Just a bit higher next time!")
    }
}

fn greater_func(x: u32){
    println!("{x} is too big.");
    match x.cmp(&25){
        Ordering::Less => println!("Just a bit lower next time!"),
        Ordering::Greater => println!("Seriously, that number is waaaaay too big!!"),
        Ordering::Equal => println!("Just a bit lower next time!")
    }
}

fn equal_func(x: u32){
    println!("{x} is just right.");
}