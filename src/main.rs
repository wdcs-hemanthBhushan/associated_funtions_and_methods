#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}

impl Student {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn check(&self) -> (&String, &u32) {
        (&self.name, &self.age)
    }

    fn change_name(&mut self, name: &str, age: u32) {
        self.name = name.to_string();
        self.age = age;
    }
}
#[derive(Debug)]
struct School {
    s1: Student,
    s2: Student,
}

impl School {
    fn new() -> Self {
        Self {
            s1: Student::new("hemanth".to_string(), 100),
            s2: Student::new("efrgewfwg".to_string(), 24),
        }
    }

    fn change_student(&mut self) {
        self.s1.age = 25555
    }
    fn check(&self) -> &Student {
        &self.s1
    }
}

fn main() {
    let mut s1: Student = Student {
        name: String::from("hemanth"),
        age: 23,
    };
    println!("{:?}", s1.check());
    println!("{:?}", s1.change_name("he4365y5manth", 244));
    println!("{:?}", s1.check());

    let mut school = School {
        s1: Student::new("hemanth".to_string(), 100),
        s2: Student {
            name: "radom".to_string(),
            age: 23,
        },
    };

    println!("{:?}",school);

    school.change_student();
    println!("{:?}",school);
    println!("{:?}",school.check());
}
