
pub use libraries::core::functions::Greet;

pub struct Person {
    name: String,
    age: u32,
}
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, am a person. My name is {} and I am {} years old!", self.name, self.age)
    }
}

#[derive(Debug)]
pub enum AnimalSpecie {
    Cat,
    Dog,
    Bird,
}
pub struct Animal {
    name: String,
    species: AnimalSpecie,
}
impl Animal {
    pub fn new(name: String, species: AnimalSpecie) -> Self {
        Animal { name, species }
    }
}
impl Greet for Animal {
    fn greet(&self) -> String {
        format!("Hello, am an animal, specifically a {:?}! My name is {}!", self.species, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_person_greet() {
        let person = Person::new("Alice".to_string(), 30);
        assert_eq!(person.greet(), "Hello, am a person. My name is Alice and I am 30 years old!");
    }
    #[test]
    fn test_animal_greet() {
        let animal = Animal::new("Buddy".to_string(), AnimalSpecie::Dog);
        assert_eq!(animal.greet(), "Hello, am an animal, specifically a Dog! My name is Buddy!");
    }
}