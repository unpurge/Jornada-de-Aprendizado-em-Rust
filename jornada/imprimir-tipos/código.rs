fn main() {
    let nome: String = String::from("Raul Scardua Fioroti");

    let idade_string: String = String::from("18");
    let idade_inteiro = idade_string.parse::<i32>().unwrap();

    let altura_string: String = String::from("1.75");
    let altura_float = altura_string.parse::<f32>().unwrap();

    let letra: char = 'R';

    println!("Nome: {nome}.");
    println!("Tem {idade_inteiro} anos.");
    println!("Altura em metros: {altura_float}.");
    println!("Primeira letra do nome: {letra}");
}
