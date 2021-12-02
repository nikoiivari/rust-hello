use std::string::String;

enum UserInput {
    Exit = 0,
    Kylla,
    Ei,
    Virhe
}

fn main() {
    let mut userwallet: f32 = 20.00;
    let coffee_price: f32 = 4.99;

    loop {
        println!("Olet kahvilassa. Kahvi maksaa {}€", coffee_price);
        println!("Lompakossasi on rahaa {}€", userwallet);
        println!("Ostatko kahvin? (KYLLÄ/EI)");

        let userstring = get_user_string();
        let userinput = parse_user_input(userstring);
        println!(""); // Makes output prettier
        if let UserInput::Exit = userinput {
            println!("Poistut kahvilasta asianmukaisesti.");
            break;}
        else if let UserInput::Virhe = userinput {
            println!("Valitsit väärin. Sinut heitetään ulos kahvilasta.");
            break;
        }
        else if let UserInput::Kylla = userinput {
            if userwallet >= coffee_price {
                userwallet = userwallet - coffee_price;
                println!("Ostit kahvin. Lompakkoon jäi rahaa: {}€", userwallet);
                println!("");                
            } else {
                println!("Lompakossa ei ole tarpeeksi rahaa kahvin ostamiseen.");
                println!("Ostotapahtuma keskeytyy.");
                println!("");
            }
        }
        else if let UserInput::Ei = userinput {
            println!("Et ostanut kahvia. Myyjä katselee sinua harmistuneena. Kulutat kahvilan");
            println!("hengitysilmaa, ja viet yhden istumapaikan tuottamatta tuloja kahvilalle.");
            println!("");
        }
    }

}

fn get_user_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s;
}

fn parse_user_input(parseable: String) -> UserInput {
    match &parseable as &str {
        "EXIT\n" => return UserInput::Exit,
        "KYLLÄ\n" => return UserInput::Kylla,
        "EI\n" => return UserInput::Ei,
        _ => return UserInput::Virhe
    }
}