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
	
	fn nrt(&self, other: &Self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn abs(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::abs(*self)))
	}
	
	fn sin(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::sin(*self)))
	}
	
	fn cos(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::cos(*self)))
	}
	
	fn tan(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::tan(*self)))
	}
	
	fn asin(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::asin(*self)))
	}
	
	fn acos(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::acos(*self)))
	}
	
	fn atan(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::atan(*self)))
	}
	
	fn atan2(&self, other: &Self) -> Calculation<Self> {
		Ok(Answer::Single(f64::atan2(*self, *other)))
	}
	
	fn floor(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::floor(*self)))
	}
	
	fn ceil(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::ceil(*self)))
	}
	
	fn round(&self) -> Calculation<Self> {
		Ok(Answer::Single(f64::round(*self)))
	}
}