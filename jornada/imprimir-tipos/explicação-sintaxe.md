fn main() {
    let nome: String = String::from("Raul Scardua Fioroti");
    let idade_string: String = String::from("18");
    let altura_string: String = String::from("1.75");
      -> A variável nome, recebe o tipo como: String. E faz o texto dentro dos parenteses, ir direto na memória da variável.

    let idade_inteiro = idade_string.parse::<i32>().unwrap();
    let altura_float = altura_string.parse::<f32>().unwrap();
      -> É criado variáveis, que são alocados em suas memórias: os textos das variáveis_string porém em formato "traduzido" para inteiro ou float.
      Sendo, i32 para inteiro com sinal.
      E, f32 para float.

    let letra: char = 'R';
      -> Apenas é criado uma variável com o tipo: char, que é alocado na memória a letra: R.
      
    println!("Nome: {nome}.");
    println!("Tem {idade_inteiro} anos.");
    println!("Altura em metros: {altura_float}.");
    println!("Primeira letra do nome: {letra}");
      -> Ambas funções de imprimir valores.
      println!: para imprimir COM quebra de linha.
      print!: para imprimir SEM quebra de linha.
      As variáveis dentro do parentêses, para este caso, funciona da mesma forma se estivesse fora, como: ...}", variável); Bem semelhante ao C.
}
