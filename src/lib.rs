pub struct Person {
    pub name: &'static str,
    pub age: i16
}

impl Person {
    pub fn new(name: &'static str, age: i16) -> Self {
        Person {
            name: name,
            age: age
        }
    }
}

#[cfg(test)]
mod tests {
    use Person;

    #[test]
    fn create_person() {
        let luc: Person = Person::new("Luc", 35);
        assert_eq!(luc.name, "Luc");
        assert_eq!(luc.age, 35);
    }
}
