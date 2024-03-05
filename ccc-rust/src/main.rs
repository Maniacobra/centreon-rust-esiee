mod proto {
    pub mod tutorial;
}

fn main() {
    println!("Hello, world!");
    let mut test_person = proto::tutorial::Person::default();
    test_person.name = String::from("Denis Bureau");
    println!("Person: {:?}", test_person);
}
