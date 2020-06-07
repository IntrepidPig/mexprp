use std::fmt;
use std::cmp::Ordering;

use rug::Rational;
use opers::Calculation;
use errors::MathError;
use answer::Answer;
use num::Num;
use context::Context;

/// A complex number made of a real part and an imaginary part, both of which are `rug::Rationals`.
/// Requires the `rug` feature.
#[derive(Debug, Clone)]
pub struct ComplexRugRat {
	/// The real part
	pub r: Rational,
	/// The imaginary part
	pub i: Rational,
}

impl Num for ComplexRugRat {
	fn from_f64(t: f64, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(ComplexRugRat {
			r: {
				if let Some(r) = Rational::from_f64(t) {
					r
				} else {
					return Err(MathError::Other); // TODO make descriptive
				}
			},
			i: Rational::from(0),
		}))
	}

	fn from_f64_complex((r, i): (f64, f64), _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(ComplexRugRat {
			r: if let Some(r) = Rational::from_f64(r) {
				r
			} else {
				return Err(MathError::Other); // TODO make descriptive
			},
			i: if let Some(i) = Rational::from_f64(i) {
				i
			} else {
				return Err(MathError::Other); // TODO make descriptive
			},
		}))
	}

	fn typename() -> String {
		String::from("ComplexRugRat")
	}

	fn tryord(&self, other: &Self, _ctx: &Context<Self>) -> Result<Ordering, MathError> {
		if let Some(ord) = self.partial_cmp(other) {
			Ok(ord)
		} else {
			Err(MathError::CmpError)
		}
	}

	fn add(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(&self.r + &other.r);
		let i = Rational::from(&self.i + &other.i);

		Ok(Answer::Single(ComplexRugRat { r, i }))
	}

	fn sub(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(&self.r - &other.r);
		let i = Rational::from(&self.i - &other.i);

		Ok(Answer::Single(ComplexRugRat { r, i }))
	}

	fn mul(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r1 = Rational::from(&self.r * &other.r);
		let i1 = Rational::from(&self.r * &other.i);
		let i2 = Rational::from(&self.i * &other.r);
		let r2 = Rational::from(&self.i * &other.i);
		let r = r1 - r2;
		let i = i1 + i2;

		Ok(Answer::Single(ComplexRugRat { r, i }))
	}

	fn div(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let conj = other.conjugate();
		let num = self.mul(&conj, ctx)?.unwrap_single();
		let den = other.mul(&conj, ctx)?.unwrap_single();
		let r = Rational::from(&num.r / &den.r);
		let i = Rational::from(&num.i / &den.r);

		Ok(Answer::Single(ComplexRugRat { r, i }))
	}
}

impl ComplexRugRat {
	/// Returns the complex conjugate of this number
	pub fn conjugate(&self) -> Self {
		ComplexRugRat {
			r: self.r.clone(),
			i: -self.i.clone(),
		}
	}
}

impl PartialOrd for ComplexRugRat {
	fn partial_cmp(&self, other: &ComplexRugRat) -> Option<Ordering> {
		Some(self.r.cmp(&other.r))
	}
}

impl PartialEq for ComplexRugRat {
	fn eq(&self, other: &ComplexRugRat) -> bool {
		self.r.eq(&other.r)
	}
}

impl fmt::Display for ComplexRugRat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.i == 0 {
			write!(f, "{}", self.r)
		} else if self.r == 0 {
			write!(f, "{}i", self.i)
		} else {
			write!(f, "({} + {}i)", self.r, self.i)
		}
	}
}
