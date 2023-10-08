use crate::garden::{test, vagetables::{test as vTest, Asparagus}};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    
    test();
    vTest();
}
