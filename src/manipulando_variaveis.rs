//Manipulando variáveis

/*
    ---------------------------------------------
    let = variavel
    ---------------------------------------------
    mut = transforma a variavel em mutável
    ---------------------------------------------
    let mut *nome da variavel* = variavel mutável
    ---------------------------------------------
*/

fn main(){
    let mut x = 10;
    println!("Valor inicial da variavel 'x': {}", x);

    x = 20;
    println!("Valor final da variavel 'x': {}", x);
}