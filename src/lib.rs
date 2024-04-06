use std::io;

fn main(){
    println!("Enter N which represents first natural numbers");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid Number");

    let difference = difference(n);

    println!("{}", difference)
}

pub fn difference(n:u32) ->u32{
    let squares  = sum_of_squares(n);
    let square_of_sum = square_of_sum(n);
    square_of_sum - squares
}

pub fn square_of_sum(n:u32) -> u32{
    let sum = (1..=n).sum::<u32>();
    sum * sum
}
pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x * x).sum::<u32>()
}