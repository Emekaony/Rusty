#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

fn display_person_attributes(p: &Person) -> String {
    let message: String = format!("hello there {} {}", p.first_name, p.last_name);
    return message;
}

fn main() {
    let emeka = Person {
        first_name: "Nnaemeka".to_string(),
        last_name: "Onyeokoro".to_string(),
    };
    println!("{}", display_person_attributes(&emeka));
    println!("emeka's first name is {}.", emeka.first_name);
    println!("emeka's last name is {}.", emeka.last_name);
}
