// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;
fn ler()->i32{
    let mut input =String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    input.trim().parse().unwrap()
}
fn ler_float()->f64{
    let mut input =String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
}

fn main() {

    println!("digite o ano do seu carro:");
    let mut ano=ler();
    println!("digite o preço:");
    let mut preco=ler_float();

    if ano < 1990 {
        println!("este é o valor do carro {:.2}, de ano {} valor de inposto a pagar {:.2} de 1,5%", preco, ano, preco*0.015);

    }else {
        println!("este é o valor do carro {:.2}, de ano {} valor de inposto a pagar {:.2} de 1%", preco, ano, preco*0.01);

    }
}
