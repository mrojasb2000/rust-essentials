// see chapter2/checking.rs

fn main(){
    let score : i32 = 100;
    println!("Score: {}", score);
    let score = "YOU WON!"; // error: mismatched types: expected i32, found reference
    println!("Score: {}", score);

    let player1 = "Rob";
    let player2 = "Jane";
    //let player3 = player1.to_owned() + player2;
    let player3 = player1.to_string() + player2;
    println!("Players: {}", player3);
}