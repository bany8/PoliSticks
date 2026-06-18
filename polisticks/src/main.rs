struct Player{
    name: String,
    symbol: char,
}

fn main() {
    let mut player_1: Player = Player {name: "Player1".to_string(), symbol: 'X'};
    let mut player_2: Player = Player {name: "Player2".to_string(), symbol: 'O'};

    println!("co tam ?")
}

fn menu(){
    println!("Witamy w Polisticks !!");
    println!("Wybierz opcję");
}
