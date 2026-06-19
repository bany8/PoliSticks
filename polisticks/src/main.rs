use std::{fmt::Write, io};

struct Player{
    name: String,
    symbol: char,
}

impl Player{
    fn new(name: String, symbol: char) -> Self {
        Player {name: name,
                symbol: symbol}
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut player_1: Player = Player::new("Gracz1".to_string(), 'X');
    let mut player_2: Player = Player::new("Gracz2".to_string(), 'O');
    let mut size: u64 = 5;
    let mut win_coundition: u64 = 3;

    menu(&mut player_1, &mut player_2, &mut size, &mut win_coundition);
    println!("co tam ?");
}

fn menu(player_1: &mut Player, player_2: &mut Player, size: &mut u64, win_coundition: &mut u64){
    println!("Witamy w Polisticks !!");
    println!("Obecne ustawienia:");
    println!("Gracz1 imie: {}, symbol: {} - (aby zmienić wpisz 1)", player_1.name, player_1.symbol);
    println!("Gracz2 imie: {}, symbol: {} - (aby zmienić wpisz 2)", player_2.name, player_2.symbol);
    println!("Rozmiar pola: {size}x{size} - (aby zmienić wpisz 3)");
    println!("Liczba z rzędu do wygranej: {win_coundition} - (aby zmienić wpisz 4)");
    println!("Wyjdź - (wpisz 0)\n");
    println!("Wybór: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess {
        0 => {},
        _ => {},
        1 => {println!("Wybier imie: ");
            io::stdin().read_line(&mut player_1.name).expect("Failed to read line");
            //println!("Wybier symbol: ");
            //let mut x: String = "X".to_string();
            //io::stdin().read_line(&mut x).expect("Failed to read line");
            //player_2.symbol.
            },
        2 => {println!("Wybier imie: ");
            io::stdin().read_line(&mut player_1.name).expect("Failed to read line");
            },
    }
}
