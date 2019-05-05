// see chapter2/mutables.rs

fn main(){
    let mut fuel = 34;
    println!("Fuel: {}", fuel);
    fuel = 60;
    println!("Fuel: {}", fuel);

    let x = 42u8;
    println!("x: {}", x);
    let magic_number = 3.14f64;
    println!("Mgic Number: {}", magic_number);
}