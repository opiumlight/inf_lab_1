mod number_systems;

use std::io::Write;
use crate::number_systems::*;
use std::str::FromStr;


fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


fn main() {
    println!("Внимание! Для записи в 9С используется сдвиг +4 (т.е. 4=8, -4=0)");
    println!("Допустимые СС: 9C, Fact, Fib, Neg10, 10");

    let num = input("Введите исходное число: ");

    let dec: i32;
    match input("Укажите его систему: ").to_ascii_lowercase().as_str() {
        "9c" => dec = Symmetric9{value: num }.to_dec(),
        "fact" => dec = Factorial{value: num }.to_dec(),
        "fib" => dec = Fibonacci{value: num }.to_dec(),
        "neg10" => dec = Neg10{value: num }.to_dec(),
        "10" => dec = i32::from_str(num.as_str()).unwrap(),
        _ => panic!("Unexpected base")
    }

    let res: Box<dyn NumeralSystem>;
    match input("Аналогично, выберите конечную СС: ").to_ascii_lowercase().as_str() {
        "9c" => res = Box::new(Symmetric9::from_dec(dec)),
        "fact" => res = Box::new(Factorial::from_dec(dec)),
        "fib" => res = Box::new(Fibonacci::from_dec(dec)),
        "neg10" => res = Box::new(Neg10::from_dec(dec)),
        "10" => res = Box::new(Decimal(dec)),
        _ => panic!("Unexpected base"),
    }
    println!("{}", res);
}