

fn main() {
    let full_name: String = String::from("Adam Elfsborg");
    let role: String = String::from("Maintainer");
    let joined_at: String = String::from("Mon 13 Nov 19:08");
    let u = User::new(full_name, role, joined_at, dbg!(150 * 80));

    u.logger();
    println!("Hello, world!");
}

#[derive(Debug)]
struct User {
    full_name: String,
    role: String,
    joined_at: String,
    salary: u32,
}

impl User {
    fn new(full_name: String, role: String, joined_at: String, salary: u32) -> User {
        let u = User {
            full_name,
            role, 
            joined_at,
            salary,
        };
        dbg!(&u);
        println!("{:#?}", &u);
        
        u
    }

    fn logger(&self) {
        println!("'{}' '{}' '{}' '{}'", self.full_name, self.role, self.joined_at, self.salary)
    }
}