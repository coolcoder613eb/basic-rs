mod tokens;
use tokens::tokenise;
fn main() {
    let testcode = "10 PRINT \"Hello, World!\"\n20 GOTO 10";
    println!("{}", testcode);
    let lines = testcode.lines();
    for line in lines {
        let tokens = tokenise(line);
    }
}
