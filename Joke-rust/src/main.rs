use std::ops::Index;

fn main() {
    dose();
}
fn dose() {
    let x = 0;
    loop {
       if x<10 {
        x+=1;
        println!("I LOVE RUST");
       }else {
         println!("finished")  
       } 
    } 
}