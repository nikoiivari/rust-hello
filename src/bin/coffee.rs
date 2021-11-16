use std::string::String;

fn main() {

    get_user_yesno();
    println!("{}",get_user_string());

}


fn get_user_yesno() -> bool {
    //println!("Y/N");
        
    return true;
}

fn get_user_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s;
}