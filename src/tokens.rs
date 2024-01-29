#[derive(Copy, Clone, Debug)]
pub enum Tokens {
    Linenum,
    Print,
    Goto,
    String,
    Number,
    Comment,
    Unknown,
}
#[derive(Debug)]
pub struct Token {
    tokentype: Tokens,
    text: String,
}
#[derive(Debug)]
struct Line {
    index: usize,
    line: String,
}
pub fn tokenise(linetext: &str) -> Vec<Token> {
    println!("Line: {}", linetext);
    let mut line = Line {
        index: 0,
        line: String::from(linetext),
    };
    let linenum = getlinenum(&mut line);
    let mut tokens = vec![Token {
        tokentype: Tokens::Linenum,
        text: String::from(linenum),
    }];
    let keyword = readuntil(&mut line, ' ', false);
    let kwtype = match keyword.to_uppercase().as_str() {
        "PRINT" => Tokens::Print,
        "GOTO" => Tokens::Goto,
        _ => {
            println!("Unknown keyword!");
            Tokens::Unknown
        }
    };
    tokens.push(Token {
        tokentype: kwtype,
        text: String::from(keyword),
    });
    // Keyword-specific code
    match kwtype {
        Tokens::Print => {
            //expect an expression
            let exprs = expression(&mut line);
            for expr in exprs {
                //println!("{:#?}", expr);
                tokens.push(expr)
            }
        }
        Tokens::Goto => {
            //expect an expression
            let exprs = expression(&mut line);
            for expr in exprs {
                println!("{:#?}", expr);
                tokens.push(expr)
            }
        }
        _ => {}
    }
    return tokens;
}
fn expression(line: &mut Line) -> Vec<Token> {
    // lex an expression
    // for now, string and number literals only
    let mut left = line.line[line.index..].trim();
    println!(
        "Left: '{}', left.chars().nth(0): {}",
        left,
        left.chars().nth(0).unwrap()
    );
    if left.starts_with('"') {
        line.index += 1;
        let string = readuntil(line, '"', true);
        println!("string: {string}");
        vec![Token {
            tokentype: Tokens::String,
            text: string,
        }]
    } else if left.chars().nth(0).unwrap().is_digit(10) {
        let mut digits = String::new();
        digits.push(left.chars().nth(0).unwrap());
        line.index += 1;
        left = &line.line[line.index..];
        loop {
            if !left.is_empty() && left.chars().nth(0).unwrap().is_digit(10) {
                digits.push(left.chars().nth(0).unwrap());
                line.index += 1;
                left = &line.line[line.index..];
            } else {
                break;
            }
        }
        vec![Token {
            tokentype: Tokens::Number,
            text: digits,
        }]
    } else {
        vec![]
    }
}
fn getlinenum(line: &mut Line) -> String {
    readuntil(line, ' ', false)
}
fn readuntil(line: &mut Line, character: char, esc: bool) -> String {
    let mut text = String::new();
    let mut pretext = String::new();
    loop {
        let mut drop = false;
        let left = &line.line[line.index..];
        let index = left.find(character).unwrap();
        let minus_one = (index + line.index) - 1;
        if esc && (&line.line[minus_one..minus_one + 1] == "\\") {
            pretext.push_str(&line.line[line.index..index + line.index - 1]);
            pretext.push_str(&line.line[index + line.index..index + line.index + 1]);
            println!("ESCAPED {pretext}")
        } else {
            drop = true;
        }
        text = pretext.clone() + &line.line[line.index..index + line.index];
        println!(
            "left: {left}, index: {index}, line.index: {}, text: {text}, -1: '{}'",
            line.index,
            &line.line[minus_one..minus_one + 1]
        );
        line.index = index + line.index + 1;
        if drop {
            break;
        }
    }
    text
}
