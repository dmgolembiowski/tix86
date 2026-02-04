use chumsky::prelude::*;
use reedline::{DefaultPrompt, Reedline, Signal};

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Fact,
    Num(f64),
    Par(Vec<Token>),
}

fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<Token>> {
    recursive(|lexer| {
        let par = lexer.delimited_by(just('('), just(')')).map(Token::Par);
        let num = text::int(10)
            .then(just('.').then(text::digits(10)).or_not())
            .to_slice()
            .map(|s: &str| Token::Num(s.parse().unwrap()));

        let add = just('+').to(Token::Add);
        let sub = just('-').to(Token::Sub);
        let mul = just('*').to(Token::Mul);
        let div = just('/').to(Token::Div);
        let modulo = just('%').to(Token::Mod);
        let pow = just('^').to(Token::Pow);
        let fact = just('!').to(Token::Fact);

        let token = num
            .or(add)
            .or(sub)
            .or(mul)
            .or(div)
            .or(modulo)
            .or(pow)
            .or(fact)
            .or(par);

        token.padded().repeated().collect()
    })
}

fn factorial(n: f64) -> f64 {
    if n < 0.0 || n.fract() != 0.0 {
        f64::NAN
    } else if n == 0.0 || n == 1.0 {
        1.0
    } else {
        (1..=n as u64).map(|x| x as f64).product()
    }
}

fn pemdas<'a>() -> impl Parser<'a, &'a [Token], f64> {
    recursive(|calc| {
        let num = select_ref! { Token::Num(x) => *x };
        let par = calc.nested_in(select_ref! { Token::Par(inner) => inner.as_slice() });
        let atom = num.or(par);
        let factorial_expr = atom
            .clone()
            .foldl(just(&Token::Fact).repeated(), |acc, _| factorial(acc));

        let power = factorial_expr
            .clone()
            .then(
                just(&Token::Pow)
                    .ignore_then(factorial_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest): (f64, Vec<f64>)| {
                if rest.is_empty() {
                    first
                } else {
                    let mut it = rest.into_iter().rev();
                    let rightmost = it.next().unwrap();
                    let exponent = it.fold(rightmost, |acc, base| base.powf(acc));
                    first.powf(exponent)
                }
            });

        let prod = power.clone().foldl(
            just(&Token::Mul)
                .to(Token::Mul)
                .or(just(&Token::Div).to(Token::Div))
                .or(just(&Token::Mod).to(Token::Mod))
                .then(power)
                .repeated(),
            |a, (op, b)| match op {
                Token::Mul => a * b,
                Token::Div => a / b,
                Token::Mod => a % b,
                _ => panic!("The parser/combinator should have already intercepted nested parentheses, factorial expressions, and exponentiation"),
            },
        );
        let sum = prod.clone().foldl(
            just(&Token::Add)
                .to(Token::Add)
                .or(just(&Token::Sub).to(Token::Sub))
                .then(prod)
                .repeated(),
            |a, (op, b)| match op {
                Token::Add => a + b,
                Token::Sub => a - b,
                _ => panic!("same, but now also now true for multiplication"),
            },
        );

        sum
    })
}
fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();
    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let tokens = match lexer().parse(&buffer).into_result() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let result = match pemdas().parse(&tokens).into_result() {
                    Ok(x) => x,
                    Err(_) => continue,
                };
                println!("{}", result);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => break,
            _ => {}
        }
    }
}
