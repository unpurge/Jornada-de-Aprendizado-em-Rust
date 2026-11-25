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
--->> Se a variável idade, tiver um valor inteiro maior ou igual que 18, imprimirá "Maior de idade", mas se for menor que 18, imprimirá: "Menor de idade". 

    let tipo = if idade % 2 == 0 { "par" } else { "ímpar" }; 
    println!("{}", tipo);
    io::stdout().flush().unwrap(); 
--->> Criação de variável, sem explicitar seu tipo, mas vindo automaticamente como String, por conta de: estar atribuindo caracteres entre as aspas. Esse if, serve de atribuição direta de valor para uma variável. Lembrando que isso não existe no C, mas deveria... ;)

    let resposta_usuário = "sair";
    if resposta_usuário == "sair" {
        println!("Saindo...");
        // podendo usar: exit(0);
    }
    io::stdout().flush().unwrap();
--->> variável criada, com o valor de "sair", sendo que: a condição do if, é de se que tal variável
ter o valor de "sair", a condição verdadeira executará uma impressão, como: "Saindo...".
Coloquei o "exit(0);" para fazer uma saida/parada forçada do programa, mas precisará da biblioteca: "std::process::exit". 

    let letra: char = 'r';
    if letra.is_lowercase() {
        println!("Letra Minúscula.")
    } else {
        println!("Letra Maiúscula.");
    }
    io::stdout().flush().unwrap();
--->> Caso a condição do if for verdadeira, sendo que é preciso a letra precisa estar Minúscula, executará uma impressão, onde
tal impressão pode mudar, por conta de um/1 if, e um/1 else. Logo, duas condições.
    
}
