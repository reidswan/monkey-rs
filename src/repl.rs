use crate::lexer;

const PROMPT: &'static str = ">> ";

pub fn start(reader: &mut dyn std::io::BufRead, writer: &mut dyn std::io::Write) {
    loop {
        write!(writer, "{}", PROMPT).expect("failed to write");
        writer.flush().expect("failed to flush");
        let mut input: String = String::new();
        reader.read_line(&mut input).expect("failed to read");
        let l = lexer::Lexer::new(&input);
        for i in l {
            write!(writer, "{:?}\n", i).expect("failed to write");
        }
    }
}
