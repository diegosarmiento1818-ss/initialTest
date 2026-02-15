use hello_rust::Animal;
use hello_rust::AnimalSpecie;
use hello_rust::Person;
use libraries::core::functions::Greet;

fn main() {
    println!("Hello, world!");

    let person1 = Person::new("Alice".to_string(), 30);
    let animal1 = Animal::new("Buddy".to_string(), AnimalSpecie::Dog);
    println!("{}", person1.greet());
    println!("{}", animal1.greet());
    loop {
        // Infinite loop to keep the program running
        
    }
}

