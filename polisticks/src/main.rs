use std::io;

struct Player{
    name: String,
    symbol: char,
}

fn main() {
    let mut player_1: Player = Player("Gracz1".to_string(), X);
    let mut player_2: Player;

    init_game(&mut player_1, &mut player_2);

    
    println!("co tam ?")

}

fn init_game(p1: &mut Player, p2: &mut Player) {

}