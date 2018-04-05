use std::fmt;
use rug::Rational;
use opers::Calculation;
use errors::MathError;
use answer::Answer;
use num::Num;

impl Num for Rational {
	fn from_f64(t: f64) -> Calculation<Self> {
		Ok(Answer::Single(if let Some(r) = Rational::from_f64(t) {
			r
		} else {
			return Err(MathError::Other) // TODO make descriptive
		}))
	}
	
	fn from_f64_complex((r, i): (f64, f64)) -> Calculation<Self> {
		Ok(Answer::Single(if let Some(r) = Rational::from_f64(r) {
			r
		} else {
			return Err(MathError::Other) // TODO make descriptive
		}))
	}
	
	fn add(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(self + other);
		
		Ok(Answer::Single(r))
	}
	
	fn sub(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(self - other);
		
		Ok(Answer::Single(r))
	}
	
	fn mul(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(self * other);
		
		Ok(Answer::Single(r))
	}
	
	fn div(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(self / other);
		
		Ok(Answer::Single(r))
	}
	
	fn pow(&self, other: &Self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn sqrt(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn sin(&self) -> Calculation<Self> {
		unimplemented!()
	}
}