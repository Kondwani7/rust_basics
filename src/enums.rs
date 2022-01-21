//enums are data types with definite values
enum GameMovement{
    Up,
    Down,
    Left,
    Right,
}

fn move_player(m: GameMovement){
    //perform an action depending on the instruction
    match m {
        GameMovement::Up => println!("player moving up"),
        GameMovement::Down => println!("player moving down"),
        GameMovement:: Left => println!("player moving left"),
        GameMovement:: Right => println!("player moving right"),
    }
}

pub fn run(){
    let player1 = GameMovement::Left;
    let player2 = GameMovement::Up;
    let player3 = GameMovement::Right;
    let player4 = GameMovement::Down;

    move_player(player1);
    move_player(player2);
    move_player(player3);
    move_player(player4);

}