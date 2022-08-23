use std::io::Write;

use iris::{
    function::FunctionBuilder, lexer::TokenIterator, module::ModuleBuilder, program::Program,
    state::State,
};

fn main() {
    loop {
        let mut code = String::new();

        print!("-> ");
        loop {
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            code.push_str(line.as_str());

            let mut token_iterator = TokenIterator::new(code.as_str()).peekable();

            let expression = match iris::parser::parse(&mut token_iterator) {
                Ok(expression) => expression,
                Err(error) => {
                    if error.location.eq(0..0) {
                        print!("-| ");
                        continue;
                    } else {
                        println!("Parser error: {}", error.message);
                        break;
                    }
                }
            };

            let mut program = Program::new();
            let mut module_builder = ModuleBuilder::new(code.as_bytes());
            let mut function_builder = FunctionBuilder::new();

            match iris::expression::build(&expression, &mut module_builder, &mut function_builder) {
                Ok(_) => (),
                Err(error) => {
                    println!("Build error: {}", error.message);
                    break;
                }
            }

            module_builder.push_function(function_builder.build());
            program.push(module_builder.build());

            let mut state = State::new();
            match program.run(&mut state) {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("Runtime error: {}", error.message),
            }

            break;
        }
    }
}
