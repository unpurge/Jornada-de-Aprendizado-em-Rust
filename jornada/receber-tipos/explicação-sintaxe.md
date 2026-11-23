use std::io;
use std::io::Write;

-> Biblioteca:
      io: usada para input.
      Write: usada para forçar output enquanto o programa roda.

    // criação de váriaveis.
    let mut nome: String = String::new();
    let mut letra: String = String::new();
    let mut idade: String = String::new();
    let mut altura: String = String::new();

    print!("") e println!(""); // imprimir informações.
    
    io::stdout().flush().unwrap(); // força este print enquando o programa roda.
    
    io::stdin() // função para receber tipos.
        .read_line(&mut nome) // aloca a string no endereço da variável.
        .expect("Informe seu nome corretamente!"); // Função caso der erro.
        
    let nome = nome.trim(); // retirando a quebra de linha no começo e fim.
    
    let idade: i32 = idade.trim().parse().unwrap();
    let altura: f32 = altura.trim().parse().unwrap();

    -> Esses dois let's, são usados novamente depois de já termos criado essas variáveis,
    funcionando como um Sombreamento/Shadowing, onde apenas "atualiza" o valor anterior, porém a variável origem
    precisa pode ser mútavel (let mut).
    Com isso, na parte: "let idade: i32", informa que a variável vai atualizar e se tornar tipo: inteiro de 32bytes.

    -> Na função: "parse()", funciona como a "permissão" para poder modificar o tipo da variável.
    2 exemplos:
        1 - Caso ter exatamente "parse()" depois da atribuição da variavel, a variavel deve ser informada o tipo apos o nome,
        como: "let idade:i32".
        2 - Caso ter "parse<"novo tipo da varíavel">()", não pode informar o novo tipo antes da atribuição.
        como: "let altura = altura.trim().parse<f32>().unwrap();"
