//Trabalhando funções com Rust

//o programa sempre vai rodar na função 'main'

fn main() {
    another_function(79);
    sum_function(5, 5);
}

fn another_function(x: i32) {
    println!("o valor de 'x' é: {}", x);
}

fn sum_function(number1: i32, number2: i32) {
    println!("a soma dos numeros é: {}", number1 + number2);
}