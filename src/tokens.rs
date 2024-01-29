pub enum Tokens {
    Linenum,
    Print,
    Goto,
    String,
}
pub struct Token {
    tokentype: Tokens,
    text: str,
}
struct Line {
    index: usize,
    line: String,
}
pub fn tokenise(linetext: &str) {
    println!("{}", linetext);
    let mut line = Line {
        index: 0,
        line: String::from(linetext),
    };
    println!("Line number: {}", getlinenum(&mut line));
    println!("'{}'", &line.line[line.index..])
}
fn getlinenum(line: &mut Line) -> &str {
    readuntil(line, ' ', false)
}
fn readuntil(line: &mut Line, character: char, esc: bool) -> &str {
    let index = line.line.find(character).unwrap();
    let text = &line.line[line.index..index];
    line.index = index + 1;
    text
}
