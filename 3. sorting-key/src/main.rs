use std::io::stdin;

pub trait ShowValue {
    fn show_value(&self) -> String;
}

struct AStruct<'a, T> {
    key: &'a str,
    value: &'a Value<T>,
}

//data wrapper
struct Value<T>(T);

enum DataType {
    Integer(i32),
    Float(f64),
    String(String)
}

impl<'a> ShowValue for AStruct<'a, DataType> {
    fn show_value(&self) -> String {
        match self.value.0 {
            DataType::Integer(ref v) => format!("key is {}, value is {}", self.key, v),
            DataType::Float(ref v) => format!("key is {}, value is {}", self.key, v),
            DataType::String(ref v) => format!("key is {}, value is {}", self.key, v),
        }
    }
}

fn split_key_value(str: String) -> (String, String) {

    let mut i = 0;
    for c in str.chars() {
        if c == ',' {
            break;
        }
        i += 1;
    }

    let key = str[0..i].trim().to_string();
    let value = str[i + 1..str.len()].trim().to_string();

    (key, value)
}

fn is_integer(v: &f64) -> bool {
    if v % 2.0 == 0.0 || v % 2.0 == 0.5 || v % 2.0 == 1.0 {
        return true;
    }
    false
}


fn main() {

    let mut input_list: Vec<String> = Vec::new();
    for _i in 0..3 {
        println!("key와 value를 입력하세요. 예) aaa,1");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input_list.push(input);
    }

    let mut keys:Vec<String> = Vec::new();
    let mut variants: Vec<Value<DataType>> = Vec::new();

    for input in input_list {
        let key_and_value: (String, String) = split_key_value(input);
        let (key, value) = key_and_value;
        let f_value = value.parse::<f64>();

        let value = match f_value {
            Ok(v) => {
                if is_integer(&v) {
                    Value::<DataType>(DataType::Integer(v as i32))
                } else {
                    Value::<DataType>(DataType::Float(v))
                }
            },
            Err(_) => {
                Value::<DataType>(DataType::String(value))
            }
        };

        keys.push(key);
        variants.push(value);
    }


    let mut data_list: Vec<AStruct<DataType>> = Vec::new();
    for (idx, item) in variants.iter().enumerate() {
        data_list.push(AStruct {
            key: &keys[idx],
            value: &variants[idx]
        })
    }

    data_list.sort_by_key(|d| d.key);

    for data in data_list {
        println!("{}", data.show_value());
    }

}
