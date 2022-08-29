use std::{ops::Index, io::stdin};

fn main() {
    dose();
    input_str();
    Sec_str();
}
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
fn input_str() {
  let mut line = String::new();
  println!("Enter your name :");
  std::io::stdin().read_line(&mut line).unwrap();
  println!("Hello , {}", line);
}
fn Sec_str() {
    let mut fine =String::new();
    println!("1=Fine : 2=good : 3=bad");
    println!("How are you Today?");
    std::io::stdin().read_line(&mut fine).unwrap();
    
    //  match fine {
    //      1 => println!("This world is good for u"),
    //      2 => println!("So far so good Guy"),
    //      3 => println!("this world is crazy"),
    //      _ => println!("WTF!! man"),
    //  }
}