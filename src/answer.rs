use num::Num;
use opers::Calculation;
use std::fmt;

/// An answer of an evalutation. Can be either a single answer or multiple
#[derive(Debug, Clone, PartialEq)]
pub enum Answer<N: Num> {
	/// A single answer
	Single(N),
	/// Multiple answers. Will always be at least two (probably)
	Multiple(Vec<N>),
}

impl<N: Num> Answer<N> {
	/// Perform an operation on all the values of an answer with all the values of another answer
	pub fn op<F: Fn(&N, &N) -> Calculation<N>>(&self, other: &Self, oper: F) -> Calculation<N> {
		fn push_answers<N: Num>(answer: Answer<N>, list: &mut Vec<N>) {
			match answer {
				Answer::Single(n) => list.push(n),
				Answer::Multiple(ns) => {
					for n in ns {
						list.push(n)
					}
				}
			}
		}
		
		match *self {
			Answer::Single(ref n) => {
				match *other {
					Answer::Single(ref n2) => {
						oper(n, n2)
					},
					Answer::Multiple(ref n2s) => {
						let mut answers = Vec::new();
						for n2 in n2s {
							push_answers(oper(n, n2)?, &mut answers);
						}
						Ok(Answer::Multiple(answers))
					}
				}
			},
			Answer::Multiple(ref ns) => {
				match *other {
					Answer::Single(ref n2) => {
						let mut answers = Vec::new();
						for n in ns {
							push_answers(oper(n, n2)?, &mut answers);
						}
						Ok(Answer::Multiple(answers))
					},
					Answer::Multiple(ref n2s) => {
						let mut answers = Vec::new();
						for n in ns {
							for n2 in n2s {
								push_answers(oper(n, n2)?, &mut answers);
							}
						}
						Ok(Answer::Multiple(answers))
					}
				}
			},
		}
	}
	
	/// Perform an operation on all the values of an answer
	pub fn unop<F: Fn(&N) -> Calculation<N>>(&self, oper: F) -> Calculation<N> {
		fn push_answers<N: Num>(answer: Answer<N>, list: &mut Vec<N>) {
			match answer {
				Answer::Single(n) => list.push(n),
				Answer::Multiple(ns) => {
					for n in ns {
						list.push(n)
					}
				}
			}
		}
		
		match *self {
			Answer::Single(ref n) => {
				oper(n)
			},
			Answer::Multiple(ref ns) => {
				let mut answers = Vec::new();
				for n in ns {
					push_answers(oper(n)?, &mut answers);
				}
				Ok(Answer::Multiple(answers))
			},
		}
	}
	
	/// Unwrap the single variant of an answer
	pub fn unwrap_single(self) -> N {
		match self {
			Answer::Single(n) => n,
			Answer::Multiple(_) => panic!("Attempted to unwrap multiple answers as one")
		}
	}
	
	/// Convert this answer into a vector
	pub fn to_vec(self) -> Vec<N> {
		match self {
			Answer::Single(n) => vec![n],
			Answer::Multiple(ns) => ns,
		}
	}
}

impl<N: Num> fmt::Display for Answer<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Answer::Single(ref n) => write!(f, "{}", n),
			Answer::Multiple(ref ns) => {
				let mut buf = String::from("{");
				for (i, n) in ns.iter().enumerate() {
					buf.push_str(&format!("{}", n));
					if i + 1 < ns.len() {
						buf.push_str(", ");
					}
				}
				buf.push_str("}");
				write!(f, "{}", &buf)
			}
		}
	}
}