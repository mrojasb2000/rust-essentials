// see chapter2/constants.rs

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";
const MYPI: f32 = 3.14;

fn main(){
    println!("The Game you are playing is called {}.", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);
    println!("In the Game {0} you start with {1} % health, yes you rea", MAX_HEALTH, GAME_NAME);
    println!("You have {points} % health", points = 70);
    println!("MAX_HEALTH is {:x} in hexadecimal", MAX_HEALTH);
    println!("MAX_HEALTH is {:b} in binary", MAX_HEALTH);
    println!("Two written in binary {0:b}", 2);
}