fn main() {
    let mut a = vec![10, 20, 30, 40, 50];

    // increase all elements by 1 using map
    a = a.iter().map(|x| x + 1).collect();

    let mut b = Vec::new();

    b.push(10);
    b.push(20);
    b.push(1);
    b.push(8);
    println!("{:?}", b);

    b = a;

    println!("{:?}", b);

    let person1: Person = Person::new("Dhikshith", 21);

    let mut person2: Person = Person::new("Dhikshith", 22);

    person2.set_address("Bangalore");

    println!("Person 1: {:?}", person1.to_string());
    println!("Person 2: {:?}", person2.to_string());
}    


//Make Person implement the Debug trait

struct Person{
    name: String,
    age: u8,
    address: Option<String>,
}


impl Person {
    /// Creates a new [`Person`].
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
            address: None,
        }
    }

    fn set_address(&mut self, address: &str) {
        self.address = Some(address.to_string());
    }


    fn to_string(self) -> String {
        // Stringify this object
        let mut s = String::new();
        s.push_str("Name: ");
        s.push_str(&self.name);
        s.push_str("    Age: ");
        s.push_str(&self.age.to_string());
        s.push_str("    Address: ");
        s.push_str(&self.address.unwrap_or("None".to_string()));
        s 
    }
}