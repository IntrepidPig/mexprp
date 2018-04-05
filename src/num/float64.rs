use std::f64;

use opers::Calculation;
use errors::MathError;
use num::Num;
use answer::Answer;

impl Num for f64 {
	fn from_f64(t: f64) -> Calculation<Self> {
		Ok(Answer::Single(t))
	}
	
	fn from_f64_complex((r, _i): (f64, f64)) -> Calculation<Self> {
		Ok(Answer::Single(r))
	}
	
	fn add(&self, other: &Self) -> Calculation<Self> {
		Ok(Answer::Single(*self + *other))
	}
	
	fn sub(&self, other: &Self) -> Calculation<Self> {
		Ok(Answer::Single(*self - *other))
	}
	
	fn mul(&self, other: &Self) -> Calculation<Self> {
		Ok(Answer::Single(*self * *other))
	}
	
	fn div(&self, other: &Self) -> Calculation<Self> {
		if *other == 0.0 {
			return Err(MathError::DivideByZero)
		}
		
		Ok(Answer::Single(*self / *other))
	}
	
	fn pow(&self, other: &Self) -> Calculation<Self> {
		Ok(Answer::Single(self.powf(*other)))
	}
	
	fn sqrt(&self) -> Calculation<Self> {
		let sqrt = f64::sqrt(*self);
		Ok(Answer::Multiple(vec![sqrt, -sqrt]))
	}
	
	fn sin(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::sin(*self)))
	}
}