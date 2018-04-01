#[derive(Debug, Fail)]
#[fail(display = "Got unexpected token")]
pub struct UnexpectedToken(pub String);

#[derive(Debug, Fail)]
#[fail(display = "Parenthesis didn't match")]
pub struct MismatchedParenthesis;

#[derive(Debug, Fail)]
#[fail(display = "Variable {} wasn't set to a value", name)]
pub struct UninitializedVar {
	pub name: String,
}

#[derive(Debug, Clone, Fail)]
#[fail(display = "Name was already in use: {}", name)]
pub struct NameInUse {
	pub name: String,
}

#[derive(Debug, Fail)]
pub enum EvalError {
	#[fail(display = "{}", error)]
	UninitializedVar {
		error: UninitializedVar
	},
}

#[derive(Debug, Fail)]
pub enum MathError {
	#[fail(display = "Unknown error occurred in evaluation")]
	Unknown,
	#[fail(display = "Variable '{}' is not defined", name)]
	UndefinedVariable {
		name: String,
	},
	#[fail(display = "Function '{}' is not defined", name)]
	UndefinedFunction {
		name: String,
	},
	#[fail(display = "A function was passed incorrect arguments")]
	IncorrectArguments,
	#[fail(display = "Attempted to divide by zero")]
	DivideByZero,
	#[fail(display = "A NaN value was attempted to be used as an operand")]
	NaN,
}

#[derive(Debug, Fail)]
pub enum Expected {
	#[fail(display = "Expected another operator")]
	Operator,
	#[fail(display = "Expected another expression")]
	Expression,
	#[fail(display = "Expected a parenthesis")]
	Paren,
	#[fail(display = "Expected a function")]
	Function,
}