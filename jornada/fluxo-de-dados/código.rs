use std::io;
use std::io::Write;

fn main() {
    let idade: i32 = 18;
    if idade >= 18 {
        println!("Maior de idade.");
    } else {
        println!("Menor de idade.");
    }
    io::stdout().flush().unwrap();

    let tipo = if idade % 2 == 0 { "par" } else { "ímpar" };
    println!("{}", tipo);
    io::stdout().flush().unwrap();

    let resposta_usuário = "sair";
    if resposta_usuário == "sair" {
        println!("Saindo...");
    }
    io::stdout().flush().unwrap();

    let letra: char = 'r';
    if letra.is_lowercase() {
        println!("Letra Minúscula.")
    } else {
        println!("Letra Maiúscula.");
    }
    io::stdout().flush().unwrap();
}
