use std::fmt;
use opers::Calculation;
use num::Num;
use answer::Answer;

#[derive(Debug, Clone)]
pub struct ComplexFloat {
	pub r: f64,
	pub i: f64,
}

impl Num for ComplexFloat {
	fn from_f64(t: f64) -> Calculation<Self> {
		Ok(Answer::Single(ComplexFloat {
			r: t,
			i: 0.0,
		}))
	}
	
	fn from_f64_complex((r, i): (f64, f64)) -> Calculation<Self> {
		Ok(Answer::Single(ComplexFloat { r, i }))
	}
	
	fn add(&self, other: &Self) -> Calculation<Self> {
		let r = self.r + other.r;
		let i = self.i + other.i;
		
		Ok(Answer::Single(ComplexFloat { r, i }))
	}
	
	fn sub(&self, other: &Self) -> Calculation<Self> {
		let r = self.r - other.r;
		let i = self.i - other.i;
		
		Ok(Answer::Single(ComplexFloat { r, i }))
	}
	
	fn mul(&self, other: &Self) -> Calculation<Self> {
		let r1 = self.r * other.r;
		let i1 = self.r * other.i;
		let i2 = self.i * other.r;
		let r2 = self.i * other.i;
		let r = r1 - r2;
		let i = i1 + i2;
		
		Ok(Answer::Single(ComplexFloat { r, i }))
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
		let r = num.r / den.r;
		let i = num.i / den.r;
		
		Ok(Answer::Single(ComplexFloat { r, i }))
	}
	
	fn pow(&self, other: &Self) -> Calculation<Self> {
		unimplemented!()
	}
}

impl ComplexFloat {
	pub fn conjugate(&self) -> Self {
		ComplexFloat {
			r: self.r,
			i: -self.i,
		}
	}
}

impl From<(f64, f64)> for ComplexFloat {
	fn from((r, i): (f64, f64)) -> Self {
		ComplexFloat {
			r,
			i,
		}
	}
}

impl From<f64> for ComplexFloat {
	fn from(t: f64) -> Self {
		ComplexFloat {
			r: t,
			i: 0.0,
		}
	}
}

impl fmt::Display for ComplexFloat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.i == 0.0 {
			write!(f, "{}", self.r)
		} else if self.r == 0.0 {
			write!(f, "{}i", self.i)
		} else {
			write!(f, "({} + {}i)", self.r, self.i)
		}
	}
}