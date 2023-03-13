use std::io;

fn main(){
    
    println!("\n Menu               -               Price");
    println!("\n1.Poundo Yam/Edinkaiko Soup - P           #3,200");
    println!("\n2.Fried Rice & Chicken - F                #3,000");
    println!("\n3.Amala & Ewedu Soup - A                  #2,500");
    println!("\n4.Eba & Egusi Soup - E                    #2,000");
    println!("\n5.White Rice & Stew - W                   #2,500");

    
    let p:f64 = 3_200.0;
    let f:f64 = 3_000.0;
    let a:f64 = 2_500.0;
    let e:f64 = 2_000.0;
    let w:f64 = 2_500.0;

    println!("\nWhat food do you want:");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid Food");
    let order:&str = input1.trim();

    if order == "p" {
        println!("\nHow many portions do you want:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let portion:f64 = input2.trim().parse().expect("Not a valid Number");

    let price = p*(portion);
    if price > 10_000.0 {
        let discount = price * (5.0/100.0);
        println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
    }
    else {
        println!("\nYour order is White Rice & stew, your bill is {}",price);
    }
    }
    else if order == "f" {
        println!("\nHow many portions do you want:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let portion:f64 = input2.trim().parse().expect("Not a valid Number");

    let price = f*(portion);
    if price > 10_000.0 {
        let discount = price * (5.0/100.0);
        println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
    }
    else {
        println!("\nYour order is White Rice & stew, your bill is {}",price);
    }
    }
    else if order == "a" {
        println!("\nHow many portions do you want:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let portion:f64 = input2.trim().parse().expect("Not a valid Number");

    let price = a*(portion);
    if price > 10_000.0 {
        let discount = price * (5.0/100.0);
        println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
    }
    else {
        println!("\nYour order is White Rice & stew, your bill is {}",price);
    }
    }
    else if order == "e" {
        println!("\nHow many portions do you want:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let portion:f64 = input2.trim().parse().expect("Not a valid Number");

    let price = e*(portion);
    if price > 10_000.0 {
        let discount = price * (5.0/100.0);
        println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
    }
    else {
        println!("\nYour order is White Rice & stew, your bill is {}",price);
    }
    }
    else if order == "w" {
        println!("\nHow many portions do you want:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let portion:f64 = input2.trim().parse().expect("Not a valid Number");

    let price = w*(portion);
    if price > 10_000.0 {
        let discount = price * (5.0/100.0);
        println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
    }
    else {
        println!("\nYour order is White Rice & stew, your bill is {}",price);
    }
    }
    else {
        println!("\n Not on the Menu");
    }
    loop {
        println!("\n Do you want to make another order?, (Y/N)");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid order");
    
        let new_order:&str = input3.trim();
    if new_order == "y" {
        println!("\n What is your order");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Not a valid Food");
        let order:&str = input4.trim();
    
        if order == "p" {
            println!("\nHow many portions do you want:");
    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid String");
        let portion:f64 = input2.trim().parse().expect("Not a valid Number");
    
        let price = p*(portion);
        if price > 10_000.0 {
            let discount = price * (5.0/100.0);
            println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
        }
        else {
            println!("\nYour order is White Rice & stew, your bill is {}",price);
        }
        }
        else if order == "f" {
            println!("\nHow many portions do you want:");
    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid String");
        let portion:f64 = input2.trim().parse().expect("Not a valid Number");
    
        let price = f*(portion);
        if price > 10_000.0 {
            let discount = price * (5.0/100.0);
            println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
        }
        else {
            println!("\nYour order is White Rice & stew, your bill is {}",price);
        }
        }
        else if order == "a" {
            println!("\nHow many portions do you want:");
    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid String");
        let portion:f64 = input2.trim().parse().expect("Not a valid Number");
    
        let price = a*(portion);
        if price > 10_000.0 {
            let discount = price * (5.0/100.0);
            println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
        }
        else {
            println!("\nYour order is White Rice & stew, your bill is {}",price);
        }
        }
        else if order == "e" {
            println!("\nHow many portions do you want:");
    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid String");
        let portion:f64 = input2.trim().parse().expect("Not a valid Number");
    
        let price = e*(portion);
        if price > 10_000.0 {
            let discount = price * (5.0/100.0);
            println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
        }
        else {
            println!("\nYour order is White Rice & stew, your bill is {}",price);
        }
        }
        else if order == "w" {
            println!("\nHow many portions do you want:");
    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid String");
        let portion:f64 = input2.trim().parse().expect("Not a valid Number");
    
        let price = w*(portion);
        if price > 10_000.0 {
            let discount = price * (5.0/100.0);
            println!("\nYour order is White Rice & stew, your bill is {} with a 5% discount",discount);
        }
        else {
            println!("\nYour order is White Rice & stew, your bill is {}",price);
        }
        }
        else {
            println!("\n Not on the Menu");
        }
        continue;
       }
       else if new_order == "n"{
        println!("\n Thank you for ordering");
        break;   
       }
       else  {
           println!("\nNot an answer");
           break;
       }
    }

}    