use iris::{
    function::FunctionBuilder, lexer::TokenIterator, module::ModuleBuilder, program::Program,
    state::State,
};

fn main() {
    let code = "2 + 2 * 2";
    let mut token_iterator = TokenIterator::new(code).peekable();
    let expression = match iris::parser::parse(&mut token_iterator) {
        Ok(expression) => expression,
        Err(error) => {
            println!("Parser error: {}", error.message);
            return;
        }
    };

    let mut program = Program::new();
    let mut module_builder = ModuleBuilder::new(code.as_bytes());
    let mut function_builder = FunctionBuilder::new();

    match iris::expression::build(&expression, &mut module_builder, &mut function_builder) {
        Ok(_) => (),
        Err(error) => {
            println!("Build error: {}", error.message);
            return;
        }
    }

    module_builder.push_function(function_builder.build());
    program.push(module_builder.build());

    let mut state = State::new();
    match program.run(&mut state) {
        Ok(value) => println!("{:?}", value),
        Err(error) => {
            println!("Runtime error: {}", error.message);
        }
    }
}
