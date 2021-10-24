mod lexer;
mod parser;
mod repl;

fn main() {
    let stdin_raw = std::io::stdin();
    let mut stdin = stdin_raw.lock();
    let mut stdout = std::io::stdout();
    repl::start(&mut stdin, &mut stdout)
}
