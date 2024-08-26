
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}


fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}


fn main() {
    
    println!("İlk sayıyı girin:");
    let mut first_num = String::new();
    std::io::stdin().read_line(&mut first_num).unwrap();
    let first_num: f64 = first_num.trim().parse().unwrap();

    println!("Yapmak istediğiniz işlemi girin (+, -, *, /):");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    println!("İkinci sayıyı girin:");
    let mut second_num = String::new();
    std::io::stdin().read_line(&mut second_num).unwrap();
    let second_num: f64 = second_num.trim().parse().unwrap();

    
    let op = match operation {
        "+" => Operation::Add(first_num, second_num),
        "-" => Operation::Subtract(first_num, second_num),
        "*" => Operation::Multiply(first_num, second_num),
        "/" => Operation::Divide(first_num, second_num),
        _ => panic!("Geçersiz işlem!"),
    };

   
    let result = calculate(op);
    println!("Sonuç: {}", result);
}
