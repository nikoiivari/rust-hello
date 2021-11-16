use std::string::String;

enum UserInput {
    Exit = 0,
    Kylla,
    Ei,
    Virhe
}

fn main() {
    //let mut userwallet = 20.00;
    
    get_user_yesno();
    
    loop {
        let userstring = get_user_string();
        let userinput = parse_user_input(userstring);
        if let UserInput::Virhe = userinput {println!("VIRHEEE!");break;}
        else if let UserInput::Kylla = userinput {println!("KYLLÄÄÄ!");}
        else if let UserInput::Ei = userinput {println!("EIII!");}
    }

}


fn get_user_yesno() -> UserInput {
    println!("KYLLÄ/EI");
        
    return UserInput::Virhe;
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