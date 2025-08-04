use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Gender {
    Male,
    Female,
}

impl Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
        }
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    gender: Gender,
}

impl Student {
    fn new(name: &str, age: u8, gender: Gender) -> Self {
        Self {
            name: name.to_string(),
            age,
            gender,
        }
    }
}

fn group_by<F>(data: &Vec<Student>, key: F) -> HashMap<String, usize>
where
    F: Fn(&Student) -> String,
{
    let mut map = HashMap::new();
    for student in data {
        *map.entry(key(student)).or_insert(0) += 1;
    }
    map
}

fn main() {
    let students = vec![
        Student::new("张三", 16, Gender::Male),
        Student::new("李四", 45, Gender::Male),
        Student::new("王五", 17, Gender::Male),
        Student::new("翠花", 18, Gender::Female),
    ];

    let map = group_by(&students, |student| {
        if student.age > 18 {
            "成年人".to_string()
        } else {
            "未成年人".to_string()
        }
    });
    println!("{:?}", map);

    let map = group_by(&students, |student| student.gender.to_string());
    println!("{:?}", map);

    let map = group_by(&students, |student| student.name.clone());
    println!("{:?}", map);
}
