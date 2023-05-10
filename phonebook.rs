fn get_phone_number(name: String, phone_book: &[(String, i32)]) -> Result<i32, String>{
    for e in phone_book{
        if e.0 == name{
            return Ok(e.1);
        }    
    }
    return Err(String::from("Phone number not found"));
}

fn main(){
    let phonebook = [(String::from("Francesco"), 123), (String::from("Fabrizio"), 456), (String::from("Filippo"), 789)];
    let name1 = String::from("Francesco");
    let name2 = String::from("Andrea");
    let number1 = get_phone_number(name1, &phonebook);
    let number2 = get_phone_number(name2, &phonebook);


    if number1.is_ok(){
        println!("Number 1: {}", number1.unwrap());
    }

    if number2.is_ok(){
        println!("Number 2: {}", number2.unwrap());
    }
}