use std::cmp::Ordering;

use rug::Rational;
use crate::opers::Calculation;
use crate::errors::MathError;
use crate::answer::Answer;
use crate::num::Num;
use crate::context::Context;

impl Num for Rational {
	fn from_f64(t: f64, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(if let Some(r) = Rational::from_f64(t) {
			r
		} else {
			return Err(MathError::Other); // TODO make descriptive
		}))
	}

	fn from_f64_complex((r, _i): (f64, f64), _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(if let Some(r) = Rational::from_f64(r) {
			r
		} else {
			return Err(MathError::Other); // TODO make descriptive
		}))
	}

	fn typename() -> String {
		String::from("Rational")
	}

	fn tryord(&self, other: &Self, _ctx: &Context<Self>) -> Result<Ordering, MathError> {
		if let Some(ord) = self.partial_cmp(other) {
			Ok(ord)
		} else {
			Err(MathError::CmpError)
		}
	}

	fn add(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self + other);

		Ok(Answer::Single(r))
	}

	fn sub(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self - other);

		Ok(Answer::Single(r))
	}

	fn mul(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self * other);

		Ok(Answer::Single(r))
	}

	fn div(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self / other);

		Ok(Answer::Single(r))
	}
	
	fn abs(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self.abs_ref());
		
		Ok(Answer::Single(r))
	}
	
	fn floor(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self.floor_ref());
		
		Ok(Answer::Single(r))
	}
	
	fn ceil(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self.ceil_ref());
		
		Ok(Answer::Single(r))
	}
	
	fn round(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		let r = Rational::from(self.round_ref());
		
		Ok(Answer::Single(r))
	}
}
