use std::io;
use std::collections::HashMap;

fn get_arg(v:& Vec<&str>, i: usize) -> String {
    v.get(i).unwrap().to_string()
} 

fn add_hash(name: String, dept: String, h: &mut HashMap<String, String>) {
    let d = match dept.as_str() {
        "Engineering" | "E" => "Engineering".to_string(),
        "Finance" | "F" => "Finance".to_string(),
        "Sales" | "S" => "Sales".to_string(),
        _ => "?".to_string(),
    };
    h.insert(name, d);
}

fn show_hash(h:& HashMap<String, String>, dept:& String) {
    let mut names: Vec<String> = Vec::new();

    for (key, value) in h.iter() {
        if value == dept {names.push(key.to_string())}
    }
    names.sort();
    println!("{}: {:?}", dept, names);
}

fn show_all(h:& HashMap<String, String>) {
    let mut names_engineering: Vec<String> = Vec::new();
    let mut names_finance: Vec<String> = Vec::new();
    let mut names_sales: Vec<String> = Vec::new();
    let mut names_q: Vec<String> = Vec::new();

    for (key, value) in h.iter() {
        match value.as_str() {
            "Engineering" => names_engineering.push(key.to_string()),
            "Finance" => names_finance.push(key.to_string()),
            "Sales" => names_sales.push(key.to_string()),
            _ => names_q.push(key.to_string()),
        }
    }
    names_engineering.sort();
    names_finance.sort();
    names_sales.sort();
    names_q.sort();
    println!("Engineering: {:?}", names_engineering);
    println!("Finance: {:?}", names_finance);
    println!("Sales: {:?}", names_sales);
    println!("?: {:?}", names_q);

}

fn main() {
    let mut map = HashMap::new();

    
    loop{
        let mut input_text = String::new();

        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read lline");
        let input = input_text.trim_end().to_string();
        let vec: Vec<&str> = input.split_whitespace().collect();

        match vec.get(0){
            Some(t) => match t {
                &"add" => add_hash(get_arg(&vec, 1), get_arg(&vec, 3), &mut map),
                &"show" => show_hash(& map, & get_arg(&vec, 1)),
                &"all" => show_all(& map),
                &"exit" => break,
                _ => println!("invalied command"),
            }
            None => println!("please input command"),
        }
    }
}
