use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nInsert value for a:");
    io::stdin().read_line(&mut input1).expect("Not a valid Number");

    let a:f64 = input1.trim().parse().expect("Not a valid string");

    println!("\nInsert value for b:");
    io::stdin().read_line(&mut input2).expect("Not a valid Number");

    let b:f64 = input2.trim().parse().expect("Not a valid string");
    let b = f64::powf(b,2.0);

    println!("\nInsert value for c:");
    io::stdin().read_line(&mut input3).expect("Not a valid Number");

    let c:f64 = input3.trim().parse().expect("Not a valid string");

    let mut d = b - (4.0*(a*c));
    println!("\nThe discriminant is {}",d);
    
    if d>0.0
    {
        d =d.sqrt();
        let x1 = (-b + d)/2.0*a;

        let x2 = (-b - d)/2.0*a;
         
        println!("\nThere are two distinct roots");

        println!("\nThey are {} {}",x1,x2);
    }
    else if d==0.0 
    {
        d =d.sqrt();
        let x1 = (-b + d)/2.0*a;

        let x2 = (-b - d)/2.0*a;
         
        println!("\nThere is exactly on real root");

        println!("\nThey are {} {}",x1,x2);  
    }
    else if d<=0.0
    {
         println!("\nThere are no real roots");
    }
}