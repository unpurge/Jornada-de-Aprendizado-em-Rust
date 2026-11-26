use std::io;
use std::io::Write;

fn main() {
    let a = "3".parse::<i32>().expect("Erro. \n");
    let b = "22".parse::<i32>().unwrap();
    let c = "x".parse::<i32>().unwrap_or(0);
    let d = "x".parse::<i32>().unwrap_or_default();
    println!("{a}\n{b}\n{c}\n{d}");
    io::stdout().flush().unwrap();
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
        Err(e) => eprintln!("erro: {e}"), // invalid digit found in string
    }
    let o: Option<i32> = "3".parse::<i32>().ok(); // Result -> Option (a, b, c, d já são i32, não dá match com Some/None)
    match o {
        Some(n) => println!("achou {n}"),
        None => println!("nada"),
    }
    let e: Result<i32, _> = "42".parse::<i32>(); // Dará certo, pois "42" como String, é possivel se transformar como número.
    let f: Result<i32, _> = "abc".parse::<i32>(); // Dará errado, pois "abc" não é possível se transformar em número.
    match e {
        Ok(n) => println!("número {n}"),
        Err(erro) => println!("erro: {erro}"),
    }
    match f {
        Ok(n) => println!("número {n}"),
        Err(erro) => println!("erro: {erro}"),
    }

    panic!("Erro. Saindo forçadamente...\n");
}
