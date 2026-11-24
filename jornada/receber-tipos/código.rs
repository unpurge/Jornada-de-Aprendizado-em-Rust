use std::io;
use std::io::Write;
fn main() {
    let mut nome: String = String::new();
    let mut letra: String = String::new();

    print!("Informe seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut nome)
        .expect("Informe seu nome corretamente!");
    let nome = nome.trim();

    print!("Informe sua letra favorita: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut letra).expect("Tente novamente!");
    let letra: char = letra.trim().parse().unwrap();

    println!("Nome: {}.", nome);
    println!("Letra: {}.", letra);

    let mut idade: String = String::new();
    let mut altura: String = String::new();

    print!("Informe sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut idade)
        .expect("Informe sua idade corretamente!");
    let idade: i32 = idade.trim().parse().unwrap();

    print!("Informe sua altura em 'metros': ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut altura)
        .expect("Informe apenas uma letra, corretamente!");
    let altura: f32 = altura.trim().parse().unwrap();

    println!("Idade: {}.\nAltura: {:.2}.", idade, altura);
}
