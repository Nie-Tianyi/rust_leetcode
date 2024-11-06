use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    // please define the Rust input here.
    let (_functions, _relations, _max_capacity) = get_inputs();
    // please finish the function body here.

    // please define the Rust output here.
}

fn iter_relations(re: Vec<String>) {
    let mut call_relationship: HashMap<String, Vec<String>> = HashMap::new();
    let relation = re.clone();
    for r in relation {
        let mut r = r.split_ascii_whitespace();
        let caller = r.next().unwrap().to_string();
        let mut callees = Vec::new();
        while let Some(callee) = r.next() {
            callees.push(callee.to_string())
        }
        call_relationship.insert(caller, callees);
        // {caller: [callee1, callee2 ..] .. }
    }

    let entry_point = re.first().unwrap();
    for c in entry_point.split_ascii_whitespace() {
        if c == "A" {
            unimplemented!();
        }
    }
}

// returns
fn get_inputs() -> (HashMap<String, usize>, Vec<String>, usize) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut functions: HashMap<String, usize> = HashMap::new();
    let mut relations: Vec<String> = Vec::new();

    // 拿到有多少行对应关系
    let number_of_function = lines.next().unwrap().unwrap();
    let number_of_function: usize = number_of_function
        .parse()
        .expect("unexpected number of mapping relation");
    // 输入关系
    for _ in 0..number_of_function {
        let relation = lines.next().unwrap().unwrap();
        let mut relation = relation.split_ascii_whitespace();
        let func_name = relation.next().unwrap().to_string();
        let space = relation
            .next()
            .expect("expected a number")
            .parse::<usize>()
            .expect("parse number error");

        functions.insert(func_name, space);
    }
    // println!("{}", number_of_function);
    // println!("{:?}", functions);

    // 输入有多少行调用关系
    let number_of_relation = lines.next().unwrap().unwrap();
    let number_of_relation = number_of_relation
        .parse::<usize>()
        .expect("expect a number");
    // println!("{number_of_relation}");
    for _ in 0..number_of_relation {
        let relation = lines.next().unwrap().unwrap();
        relations.push(relation);
    }
    // println!("{:?}", relations);
    // 输入最大的栈容量
    let max_capacity = lines.next().unwrap().unwrap();
    let max_capacity = max_capacity.parse::<usize>().expect("expect a number");

    (functions, relations, max_capacity)
}
