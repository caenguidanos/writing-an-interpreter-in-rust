fn main() {
    let mut input = String::new();

    loop {
        println!("\nWaiting for input:");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                for token in lexer::Lexer::new(input.as_bytes()) {
                    println!("{:?}", token);
                }

                input = String::new();
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
