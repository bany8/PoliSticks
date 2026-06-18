fn main() {
    struct Player{
        name: String,
        symbol: char,
    }

    impl Player {
        fn new(name: String, symbol: char) -> Player{
            Self {name: name, 
                symbol: symbol};
        }

        fn change_name(&mut self, name: String){
            self.name = name;
        }
    
    }
    let mut player_1: Player = Player.new("Player1".to_string(), 'X');
    let mut player_2: Player = Player.new("Player2".to_string(), 'O');

    println!("co tam ?")
}

fn menu(){
    println!("Witamy w Polisticks !!");
    println!("Wybierz opcję");
}