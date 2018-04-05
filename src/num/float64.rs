use opers::Calculation;
use errors::MathError;
use num::Num;

impl Num for f64 {
	fn from_f64(t: f64) -> Calculation<Self> {
		Ok(t)
	}
	
	fn from_f64_complex((r, _i): (f64, f64)) -> Calculation<Self> {
		Ok(r)
	}
	
	fn add(&self, other: &Self) -> Calculation<Self> {
		Ok(*self + *other)
	}
	
	fn sub(&self, other: &Self) -> Calculation<Self> {
		Ok(*self - *other)
	}
	
	fn mul(&self, other: &Self) -> Calculation<Self> {
		Ok(*self * *other)
	}
	
	fn div(&self, other: &Self) -> Calculation<Self> {
		if *other == 0.0 {
			return Err(MathError::DivideByZero)
		}
		
		Ok(*self / *other)
	}
	
	fn pow(&self, other: &Self) -> Calculation<Self> {
		Ok(self.powf(*other))
	}
}