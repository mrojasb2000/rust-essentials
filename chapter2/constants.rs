// see chapter2/constants.rs

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";
const MYPI: f32 = 3.14;

fn main(){
    println!("The Game you are playing is called {}.", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);
    println!("In the Game {0} you start with {1} % health, yes you rea", MAX_HEALTH, GAME_NAME);
}