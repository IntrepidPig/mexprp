use thiserror::Error;

/// An error that can occur during parsing
#[derive(Debug, Error)]
pub enum ParseError {
	/// Got an unexpected token
	#[error("Got unexpected token: '{token}'")]
	UnexpectedToken {
		/// The token
		token: String,
	},
	/// Parentheses didn't match
	#[error("Parentheses didn't match")]
	MismatchedParentheses,
	/// Expected something but it wasn't found
	#[error("Expected something that wasn't found: {expected}")]
	Expected {
		/// The thing that was expected
		expected: Expected,
	},
}

/// An error that can occur while evaluating an expression
#[derive(Debug, Error)]
pub enum MathError {
	/// A variable that was not defined in the context was referenced
	#[error("Variable '{name}' is not defined")]
	UndefinedVariable {
		/// The name of the variable
		name: String,
	},
	/// A function that was not defined in the context was referenced
	#[error("Function '{name}' is not defined")]
	UndefinedFunction {
		/// The name of the function
		name: String,
	},
	/// A function was given arguments in an incorrect form
	#[error("A function was passed incorrect arguments")]
	IncorrectArguments,
	/// Attempted to divide by zero
	#[error("Attempted to divide by zero")]
	DivideByZero,
	/// A NaN value was used in a way that is not possible
	#[error("A NaN value was attempted to be used as an operand")]
	NaN,
	/// Tried to compare a value that can't be compared (eg NaN, Infinity, etc.)
	#[error("Tried to compare a value that can't be compared (eg NaN, Infinity, etc.)")]
	CmpError,
	/// Attempted an operation on a Number that wasn't implemented for that type
	#[error("The operation '{op}' is not supported for the type {num_type}")]
	Unimplemented {
		/// The name of the operation that was attempted
		op: String,
		/// The type of number it was attempted for
		num_type: String,
	},
	/// Another type of Error occurred.
	#[error("An unknown error occurred during evaluation")]
	Other,
}

/// An error that occurs when evaluating a string
#[derive(Debug, Error)]
pub enum EvalError {
	/// An error occurred during parsing
	#[error("Failed to parse the expression: {error}")]
	ParseError {
		/// The error
		error: ParseError,
	},
	/// An error occurred during evaluation
	#[error("Failed to evaluate the expression: {error}")]
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
#[derive(Debug, Error)]
pub enum Expected {
	/// Expected an operator
	#[error("Expected another operator")]
	Operator,
	/// Expected an expression
	#[error("Expected another expression")]
	Expression,
	/// Expected a parenthesis
	#[error("Expected a parenthesis")]
	Paren,
	/// Expected a function
	#[error("Expected a function")]
	Function,
}
