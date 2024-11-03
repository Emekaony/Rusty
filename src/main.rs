#[derive(Debug)]
struct Employee {
    firstname: String,
    company: String,
    age: u32,
}

impl Employee {
    fn details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            self.firstname, self.age, self.company
        )
    }

    fn static_details() -> String {
        String::from("Details of a person")
    }
}

fn main() {
    let emp: Employee = Employee {
        firstname: "Nnaemeka".to_string(),
        company: "Rhombus".to_string(),
        age: 23,
    };
    println!("{}", emp.details());
    println!("{:?}", emp);
    // when using static functions, use the :: thing not the dot notation
    println!("static employee details: {}", Employee::static_details());
}
