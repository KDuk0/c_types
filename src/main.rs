struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age,
        }
    }

    fn age_group(&self) -> String {
        if self.age > 150 {
            panic!("age is out of range");
        }

        if self.age < 10 {
            return "child".to_string();
        }

        if self.age >= 18 {
            return "teenager".to_string();
        }

        format!("Teenager of {} years old.", self.age)
    }
}

fn main() {
    let person = Person::new("Kliment".to_string(), 38);
    let person1 = Person::new("Aurelia".to_string(), 4);
    let person2 = Person::new("Nekoj".to_string(), 15);
    let person3 = Person::new("Drug".to_string(), 156);

    println!("{} is a {}", person.name, person.age_group());
    println!("{} is a {}", person1.name, person1.age_group());
    println!("{} is a {}", person2.name, person2.age_group());
    println!("{} is a {}", person3.name, person3.age_group());
}

