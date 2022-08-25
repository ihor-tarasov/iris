use std::io::Write;

use crate::{
    builder::Builder, expression::Expression, lexer::{PeekableTokenIterator}, program::Program, state::State,
    value::Value,
};

enum ReplError {
    SomeError,
    UnexpectedEnd,
}

fn repl_parse(code: &[u8]) -> Result<Expression, ReplError> {
    let mut token_iterator = PeekableTokenIterator::new(code);
    crate::parser::parse(&mut token_iterator).map_err(|error| {
        if error.location.eq(0..0) {
            ReplError::UnexpectedEnd
        } else {
            println!("Parser error: {}", error.message);
            ReplError::SomeError
        }
    })
}

fn repl_build(code: &[u8], expression: Expression) -> Result<Program, ReplError> {
    let mut builder = Builder::new(code);

    crate::builder::build(&expression, &mut builder).map_err(|error| {
        println!("Build error: {}", error.message);
        ReplError::SomeError
    })?;

    Ok(builder.build())
}

fn repl_run(mut program: Program) -> Result<Value, ReplError> {
    let mut state = State::new();
    program.run(&mut state).map_err(|error| {
        println!("Runtime error: {}", error.message);
        ReplError::SomeError
    })
}

fn repl_read_one_more_line(code: &mut String) {
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    code.push_str(line.as_str());
}

fn repl_run_one_more_line(code: &mut String) -> Result<(), ReplError> {
    repl_read_one_more_line(code);

    let expression = repl_parse(code.as_bytes())?;

    let program = repl_build(code.as_bytes(), expression)?;
    //println!("{:#?}", program);
    println!("{}", repl_run(program)?);
    Ok(())
}

fn repl_iteration() {
    let mut code = String::new();

    print!("-> ");
    loop {
        match repl_run_one_more_line(&mut code) {
            Ok(_) => break,
            Err(error) => match error {
                ReplError::SomeError => break,
                ReplError::UnexpectedEnd => print!("-| "),
            },
        }
    }
}

pub fn run() {
    loop {
        repl_iteration()
    }
}
