use std::io;
use std::io::Write;
fn main() {
    // criação de váriaveis: String e Char.
    let mut nome: String = String::new();
    let mut letra: String = String::new();

    // Perguntando um nome, e exigindo uma string como resposta.
    print!("Informe seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut nome)
        .expect("Informe seu nome corretamente!");
    let nome = nome.trim();

    // Perguntando a letra favorita, e exigindo apenas um caracter como resposta.
    print!("Informe sua letra favorita: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut letra).expect("Tente novamente!");
    let letra: char = letra.trim().parse().unwrap();
    // output de informações.
    println!("Nome: {}.", nome);
    println!("Letra: {}.", letra);

    // criação de váriaveis: Int e Float.
    let mut idade: String = String::new();
    let mut altura: String = String::new();

    // Perguntando uma idade, e exigindo um valor inteiro como resposta.
    print!("Informe sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut idade)
        .expect("Informe sua idade corretamente!");
    let idade: i32 = idade.trim().parse().unwrap();

    // Perguntando a altura, e exigindo um valor NÃO inteiro como resposta.
    print!("Informe sua altura em 'metros': ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut altura)
        .expect("Informe apenas uma letra, corretamente!");
    let altura: f32 = altura.trim().parse().unwrap();

    // output de informações.
    println!("Idade: {}.\nAltura: {:.2}.", idade, altura);
}
