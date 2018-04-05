use num::Num;
use opers::Calculation;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Answer<N: Num> {
	Single(N),
	Multiple(Vec<N>),
}

impl<N: Num> Answer<N> {
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
}

impl<N: Num> fmt::Display for Answer<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Answer::Single(ref n) => write!(f, "{}", n),
			Answer::Multiple(ref ns) => {
				let mut buf = String::from("S = {");
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