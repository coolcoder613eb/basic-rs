#[derive(Debug)]
pub enum Tokens {
    Linenum,
    Print,
    Goto,
    String,
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
    tokens
}
fn getlinenum(line: &mut Line) -> &str {
    readuntil(line, ' ', false)
}
fn readuntil(line: &mut Line, character: char, esc: bool) -> &str {
    let left = &line.line[line.index..];
    let index = left.find(character).unwrap();
    let text = &line.line[line.index..index + line.index];
    line.index = index + 1;
    text
}
