use num_bigint::BigInt;

fn main() {
    // Создаем два больших числа
    let a = BigInt::from(2u32).pow(21);
    let b = BigInt::from(3u32).pow(22);

    // Выполняем арифметические операции
    let sum = &a + &b;
    let difference = &a - &b;
    let product = &a * &b;
    let quotient = &a / &b;

    // Выводим результаты
    println!("a = {}", a);
    println!("b = {}", b);
    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
}