use std::io;

fn main(){

    println!("\nIf you have an experience of 5 years, input 'y'.
    \nIf you dont have an experience of 5 years, input 'n'.");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let xp = input1.trim();
    if xp == "n"  {
        println!("\nYour Incentive is #100,000");
    }
    else {
    println!("\nInput your age");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid age");
    
    let age:i64 = input2.trim().parse().expect("Not a valid age");

    if xp == "y" && age >= 40 {
        println!("\nYour Incentive is #1,580,000");   
    }
    else if xp == "y" && (age >=30 && age <40)  {
        println!("\nYour Incentive is #1,480,000");
    }
    else if xp == "y" && age <28 {
        println!("\nYour Incentive is #1,300,000 per month");
    }
    }
}        