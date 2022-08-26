use crate::{
    common::Error, expression::Expression, function::FunctionBuilder, module::ModuleBuilder,
    program::Program,
};

pub struct Builder<'a> {
    pub module_builder: ModuleBuilder<'a>,
    pub function_builder: FunctionBuilder,
}

impl<'a> Builder<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            module_builder: ModuleBuilder::new(source),
            function_builder: FunctionBuilder::new(),
        }
    }

    pub fn build(mut self) -> Program {
        let function = self.function_builder.build();
        self.module_builder.push_function(function);
        let mut program = Program::new();
        program.push(self.module_builder.build());
        program
    }
}

pub fn build(expression: &Expression, builder: &mut Builder) -> Result<(), Error> {
    match expression {
        Expression::Literal(literal) => literal.build(builder),
        Expression::Binary(binary) => binary.build(builder),
        Expression::BinaryLogic(binary_logic) => binary_logic.build(builder),
        Expression::Variable(variable) => variable.build(builder),
        Expression::Assignment(assignment) => assignment.build(builder),
        Expression::ExprList(list) => list.build(builder),
    }
}
