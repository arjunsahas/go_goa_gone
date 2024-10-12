use std::string::String;

struct Employee {
    name: String,
    address: String,
    age: u8,
}

fn print(emp: &Employee) {
    println!("Employee: {}, {}, {}", emp.name, emp.address, emp.age);
}

fn main() {
    let name = String::from("arjun");
    let address = String::from("malleswaram");

    let emp = Employee { name: name, address: address, age: 19 };
    print(&emp);

    let emp2 = Employee { name: "arjun".to_string(), address: String::from("Rajaji"), age: 19 };
    print(&emp2);
}
