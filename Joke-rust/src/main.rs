use std::{ops::Index, io::stdin};
// import Standard libray

//mainFunction
fn main() {
    dose();
    input_str();
    Sec_str();
}

//Love rust function
fn dose() {
    let mut x = 0;
    loop {
      println!("I love Rust {} Time",x);
      x+=1;
      if x > 30 {
          break;
      }
    } 
}

//what is your name
fn input_str() {
  let mut line = String::new();
  println!("Enter your name :");
  std::io::stdin().read_line(&mut line).unwrap();
  println!("Hello , {}", line);
}

//how are you
fn Sec_str() {
    let mut fine =String::new();
    println!("1=Fine : 2=good : 3=bad");
    println!("How are you Today?");
    std::io::stdin().read_line(&mut fine).unwrap();
    println!("Hello , {}", fine);
  }


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //\\      ||
   //  \\     ||
  //    \\    ||    
 //======\\   ||
//        \\  ||