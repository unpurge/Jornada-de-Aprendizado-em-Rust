• Option<T> -> Some(T), None
• Result<T, E> -> Ok(T), Err(E)
• x.unwrap()
• x.expect("msg")
• x.unwrap_or(valor)
• x.unwrap_or_default()
match x { Ok(v) => ..., Err(e) => ... }
panic!("erro fatal");
