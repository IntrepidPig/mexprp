use std::fmt;
use rug::Rational;
use opers::Calculation;
use errors::MathError;
use answer::Answer;
use num::Num;

#[derive(Debug, Clone)]
pub struct ComplexRugRat {
	pub r: Rational,
	pub i: Rational,
}

impl Num for ComplexRugRat {
	fn from_f64(t: f64) -> Calculation<Self> {
		Ok(Answer::Single(ComplexRugRat {
			r: {
				if let Some(r) = Rational::from_f64(t) {
					r
				} else {
					return Err(MathError::Other) // TODO make descriptive
				}
			},
			i: Rational::from(0),
		}))
	}
	
	fn from_f64_complex((r, i): (f64, f64)) -> Calculation<Self> {
		Ok(Answer::Single(ComplexRugRat {
			r: if let Some(r) = Rational::from_f64(r) {
				r
			} else {
				return Err(MathError::Other) // TODO make descriptive
			},
			i: if let Some(i) = Rational::from_f64(i) {
				i
			} else {
				return Err(MathError::Other) // TODO make descriptive
			},
		}))
	}
	
	fn add(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(&self.r + &other.r);
		let i = Rational::from(&self.i + &other.i);
		
		Ok(Answer::Single(ComplexRugRat { r, i }))
	}
	
	fn sub(&self, other: &Self) -> Calculation<Self> {
		let r = Rational::from(&self.r - &other.r);
		let i = Rational::from(&self.i - &other.i);
		
		Ok(Answer::Single(ComplexRugRat { r, i }))
	}
	
	fn mul(&self, other: &Self) -> Calculation<Self> {
		let r1 = Rational::from(&self.r * &other.r);
		let i1 = Rational::from(&self.r * &other.i);
		let i2 = Rational::from(&self.i * &other.r);
		let r2 = Rational::from(&self.i * &other.i);
		let r = r1 - r2;
		let i = i1 + i2;
		
		Ok(Answer::Single(ComplexRugRat { r, i }))
	}
	
	fn div(&self, other: &Self) -> Calculation<Self> {
		let conj = other.conjugate();
		let num = match self.mul(&conj)? {
			Answer::Single(n) => n,
			Answer::Multiple(_) => unreachable!(),
		};
		let den = match other.mul(&conj)? {
			Answer::Single(n) => n,
			Answer::Multiple(_) => unreachable!(),
		};
		let r = Rational::from(&num.r / &den.r);
		let i = Rational::from(&num.i / &den.r);
		
		Ok(Answer::Single(ComplexRugRat { r, i }))
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

impl ComplexRugRat {
	pub fn conjugate(&self) -> Self {
		ComplexRugRat {
			r: self.r.clone(),
			i: -self.i.clone(),
		}
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