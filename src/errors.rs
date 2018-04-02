/// An error that can occur during parsing
#[derive(Debug, Fail)]
pub enum ParseError {
	/// Got an unexpected token
	#[fail(display = "Got unexpected token: '{}'", token)]
	UnexpectedToken {
		/// The token
		token: String,
	},
	/// Parentheses didn't match
	#[fail(display = "Parentheses didn't match")]
	MismatchedParentheses,
	/// Expected something but it wasn't found
	#[fail(display = "Expected something that wasn't found: {}", expected)]
	Expected {
		/// The thing that was expected
		expected: Expected,
	}
}

/// An error that can occur while evaluating an expression
#[derive(Debug, Fail)]
pub enum MathError {
	/// A variable that was not defined in the context was referenced
	#[fail(display = "Variable '{}' is not defined", name)]
	UndefinedVariable {
		/// The name of the variable
		name: String,
	},
	/// A function that was not defined in the context was referenced
	#[fail(display = "Function '{}' is not defined", name)]
	UndefinedFunction {
		/// The name of the function
		name: String,
	},
	/// A function was given arguments in an incorrect form
	#[fail(display = "A function was passed incorrect arguments")]
	IncorrectArguments,
	/// Attempted to divide by zero
	#[fail(display = "Attempted to divide by zero")]
	DivideByZero,
	/// A NaN value was used in a way that is not possible
	#[fail(display = "A NaN value was attempted to be used as an operand")]
	NaN,
}

/// An error that occurs when evaluating a string
#[derive(Debug, Fail)]
pub enum EvalError {
	/// An error occurred during parsing
	#[fail(display = "Failed to parse the expression: {}", error)]
	ParseError {
		/// The error
		error: ParseError,
	},
	/// An error occurred during evaluation
	#[fail(display = "Failed to evaluate the expression: {}", error)]
	MathError {
		/// The error
		error: MathError,
	},
}

impl From<ParseError> for EvalError {
	fn from(t: ParseError) -> EvalError {
		EvalError::ParseError { error: t }
	}
}

impl From<MathError> for EvalError {
	fn from(t: MathError) -> EvalError {
		EvalError::MathError { error: t }
	}
}

/// Expected a token but was not met
#[derive(Debug, Fail)]
pub enum Expected {
	/// Expected an operator
	#[fail(display = "Expected another operator")]
	Operator,
	/// Expected an expression
	#[fail(display = "Expected another expression")]
	Expression,
	/// Expected a parenthesis
	#[fail(display = "Expected a parenthesis")]
	Paren,
	/// Expected a function
	#[fail(display = "Expected a function")]
	Function,
}
