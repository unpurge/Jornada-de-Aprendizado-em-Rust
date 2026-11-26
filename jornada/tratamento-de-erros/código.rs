fn main() {
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
        Err(e) => eprintln!("erro: {e}"), // invalid digit found in string
    }
    let c = "x".parse::<i32>().unwrap_or(0); // valor padrão
    let e = "x".parse::<i32>().unwrap_or_default(); // 0 (Default do tipo)
    println!("{a} {b} {c} {d} {e}");
}
