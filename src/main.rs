mod tokens;
use tokens::tokenise;
fn main() {
    let testcode = "10 PRINT \"Hello,\\\"\\n\\\" World!\"\n20 GOTO 10";
    println!("Program: \n---\n{}\n---", testcode);
    let lines = testcode.lines();
    for line in lines {
        let tokens = tokenise(line);
        println!("Tokens: {:#?}", tokens)
    }
}
