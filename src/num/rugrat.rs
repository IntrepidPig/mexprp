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
	
	fn from_f64_complex((r, _i): (f64, f64)) -> Calculation<Self> {
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
	
	fn nrt(&self, other: &Self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn abs(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn sin(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn cos(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn tan(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn asin(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn acos(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn atan(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn atan2(&self, other: &Self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn floor(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn ceil(&self) -> Calculation<Self> {
		unimplemented!()
	}
	
	fn round(&self) -> Calculation<Self> {
		unimplemented!()
	}
}