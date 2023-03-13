fn main() {
    println!("\nIdentify yourself,\nAre you a Patient or Physician");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Invalid User");
    let user:String = input1.trim().parse().expect("Invalid user");
    
    if user == "physician" {
        println!("\n Please provide your email and specialization");
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Invalid User");
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Invalid User");
        let email = input2.trim();
        let spec = input3.trim();

        let e = ("g_audu@patron.org","a_seidu@patron.org","m_gbenga@patron.org");
        let s = ("orthopedic","surgery","pediatrics");
        if  email == e.0 && spec == s.0{
            hospital_db();
        }
        else if email == e.1 && spec == s.1 {
            hospital_db();
        }
        else if email == e.2 && spec == s.2 {
            hospital_db();
        }
        else {
            println!("\n Invalid User");
        }
    }
    else if user == "patient" {
        println!("\nPlease provide your email");
        let mut input4 = String::new();
        std::io::stdin().read_line(&mut input4).expect("Invalid User");
        let email = input4.trim();
        let e = ("a_simon@gmail.com","f_tina@gmail.com","d_valerie@gmail.com","s_samuel@gmail.com","o_feji@gmail.com","m_kabir@gmail.com","a_jane@gmail.com","m_ali@gmail.com","o_chisom@gmail.com","e_agatha@gmail.com");
        
        if email == e.0 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.1 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.2 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.3 {
            allergy_less_3();
            allergy_greater_3();
        }else if email == e.4 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.5 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.6 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.7 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.8 {
            allergy_less_3();
            allergy_greater_3();
        }
        else if email == e.9 {
            allergy_less_3();
            allergy_greater_3();
        }
        
    }

}
use std::io::Read;
fn hospital_db(){
    let mut file = std::fs::File::open("ikennanicholasikwuka_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
use std::io::Write;
fn allergy_less_3(){
    let patient_records = "Week 9 - Rust File Input & Output\n";
    let physician_details = "Department of Computer Science";

    let mut file = std::fs::File::create("allergy_less_3.txt").expect("create failed");
    let mut file = std::fs::File::open("patients_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
    let mut file = std::fs::File::open("allergies_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
    
    println!("\nData written to file.");
}
fn allergy_greater_3(){


    let mut file = std::fs::File::create("allergy_greater_3.txt").expect("create failed");
    let mut file = std::fs::File::open("patients_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
    let mut file = std::fs::File::open("allergies_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
    println!("\nData written to file.");
}
