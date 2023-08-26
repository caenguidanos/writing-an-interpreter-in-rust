use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = r#"
    let five = 5;
    let ten = 10;
    
    let add = fn(x, y) {
        x + y;
    };
    
    let result_of_add = add(five, ten);
    !-/*5;
    
    5 < 10 > 5;
    
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    
    10 == 10;
    10 != 9;
    
    10 <= 10;
    10 >= 10;
    10 ** 10;
"#;

fn lexer_process(input: &[u8]) -> Vec<token::Token> {
    lexer::Lexer::new(input).collect::<Vec<token::Token>>()
}

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("lexer", |b| {
        b.iter(|| lexer_process(black_box(INPUT.as_bytes())))
    });
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
