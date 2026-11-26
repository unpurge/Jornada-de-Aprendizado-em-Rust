use std::io;
use std::io::Write;

fn main() {
    // ---------- expect ----------
    let a = "3".parse::<i32>().expect("Erro. \n");
    /* programa paralisa em processo, caso der erro.
       Precisa ter apenas um argumento em &str (texto), dentro dos parenteses desta função.*/

    // ---------- unwrap ----------
    let b = "22".parse::<i32>().unwrap();
    /*  programa paralisa em processo, caso der erro.
        *NÃO* pode ter argumento/valor dentro dos parenteses desta função.*/

    // ---------- unwrap_or ----------
    let c = "x".parse::<i32>().unwrap_or(0); // 0 é o valor padrão
    /* *NÃO* paralisa em processo, caso der erro. Funciona do mesmo jeito.
        Precisa ter um valor dentro dos parentêses desta função.
        O valor recebido, serve como um "salva-vidas" caso der erro, onde ele atribuirá a uma variável.*/

    // ---------- unwrap_or_default ----------
    let d = "x".parse::<i32>().unwrap_or_default();
    /* Valor padrão para cada tipo de variáveis, em caso de erro:
          String: "", uma String vazia.
          Char: '\0'.
          Int e Float: recebe o valor 0.
          Bool: false.
     *NÃO* paralisa em processo, caso der erro.
     *NÃO* pode ter argumento/valor dentro dos parenteses desta função.*/

    println!("{a}\n{b}\n{c}\n{d}");
    io::stdout().flush().unwrap();

    // ---------- match ----------
    // o match, é como "if" e "switch", em C.
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
        Err(e) => eprintln!("erro: {e}"), 
    }
    /* O valor da condição, caso der Ok, igual como: válido, o valor entra na "variável" temporária deste match, e podendo realizar comandos.
    Caso der Err (erro), a "variável" temporária dentro do parentêses, guardará o erro (ParseIntError) dentro de si, e o {e} o mostra como texto. */

    // ---------- Option ----------
    // enum Option<T> { Some(T), None }
    let o: Option<i32> = "3".parse::<i32>().ok(); 
    match o {
        Some(n) => println!("achou {n}"),
        None => println!("nada"),
    }
    /*  Some(T): existe um valor dentro da variável.
        None: não tem valor na variável. (sem motivo).*/

    // ---------- Result ----------
    // enum Result<T, E> { Ok(T), Err(E) }
    let e: Result<i32, _> = "42".parse::<i32>(); // Dará certo, pois "42" como String, é possivel se transformar como número.
    let f: Result<i32, _> = "abc".parse::<i32>(); // Dará errado, pois "abc" não é possível se transformar em número.
    /* Caso a String, possa se transfromar como inteiro,
    o Result, continua sendo Result, e o Ok() guardará o valor i32 dentro de si.
    Caso der erro, o Err() guardará o motivo do erro (ParseIntError) dentro de si. */

    match e {
        Ok(n) => println!("número {n}"),
        Err(erro) => println!("erro: {erro}"),
    }
    match f {
        Ok(n) => println!("número {n}"),
        Err(erro) => println!("erro: {erro}"),
    }
    /* Para certificar se deu erro ou não, nessas 2 criações de variável,
       faz o "match variável", Ok() caso ter dado certo, e Err() se deu erro.
    */
    // ---------- ? ----------
    "?", no final da sintaxe, faz abrir a "caixa" gerada pelo Result ou Option.
    Se der "Ok" ou "Some", ? retorna algum tipo ou valor para a variável.
    Caso der "Err" ou "None", o ? da return Err(e) ou None, e quebra a função na hora.
    

    panic!("Erro. Saindo forçadamente...\n");

}
