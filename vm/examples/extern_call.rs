use vm::instruction::compiler::parser::Parser;

use vm::runtime::VM;

fn main() {
    let tmp = std::time::Instant::now();
    if let Ok(content) = std::fs::read_to_string("./vm/examples/extern_call.txt") {
        let mut lexer = vm::instruction::compiler::lexer::AtlasLexer::default();
        lexer.set_path("examples/extern_call.txt");
        lexer.set_source(content);
        lexer.add_system(vm::instruction::compiler::lexer::identifier_system);
        lexer.add_system(vm::instruction::compiler::lexer::comment_system);
        let res = lexer.tokenize();
        match res {
            Ok(t) => {
                println!("Ok Lexer: {:?}", tmp.elapsed());
                let tmp = std::time::Instant::now();
                let parser = Parser::parse(t);
                match parser {
                    Ok(code) => {
                        println!("Ok Parser: {:?}", tmp.elapsed());
                        let tmp = std::time::Instant::now();
                        let mut vm = VM::new(0, code.constants);
                        vm.execute(code.ins.as_slice());
                        println!("Ok Excution: {:?}", tmp.elapsed())
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
            }
            Err(_e) => {
                println!("Error1");
            }
        }
    } else {
        println!("Error2")
    }
}