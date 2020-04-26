use std::io::stdin;
use std::cmp::Ordering::Equal;

enum Variants {
    Integer(i32),
    Float(f64),
    String(String),
}

fn main() {

    let mut input_list: Vec<Variants> = Vec::new();

    for _i in 0..10 {

        println!("정수 또는 실수 또는 문자열 입력.");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.contains(char::is_numeric) {
            let input: f64 = input.parse().unwrap();

            if input % 2.0 == 0.5 || input % 2.0 == 0.0 {
                let input: i32 = input as i32;
                input_list.push(Variants::Integer(input));

                continue;
            }

            input_list.push(Variants::Float(input));
            continue;
        }

        input_list.push(Variants::String(input.to_string()));
    }

    let mut integer_list: Vec<i32> = Vec::new();
    let mut float_list:Vec<f64> = Vec::new();
    let mut string_list:Vec<String> = Vec::new();

    for variant in input_list {
        match variant {
            Variants::Integer(value) => integer_list.push(value),
            Variants::Float(value)  => float_list.push(value),
            Variants::String(value) => string_list.push(value),
        }
    }

    integer_list.sort();
    float_list.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    string_list.sort();

    println!("integer list values are {:?}", integer_list);
    println!("float list values are {:?}", float_list);
    println!("string list values are {:?}", string_list);
}
