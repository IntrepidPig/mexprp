
use std::fmt;
use std::marker::Sized;
use std::cmp::Ordering;

#[cfg(feature = "rug")]
mod complexrugrat;
#[cfg(feature = "rug")]
mod rugrat;
mod complexfloat;
mod float64;

#[cfg(feature = "rug")]
pub use self::complexrugrat::ComplexRugRat;
pub use self::complexfloat::ComplexFloat;

use opers::Calculation;
use errors::MathError;

/// A `Num` represents any type that can be used in an expression. It requires lots of operations to
/// be implemented for it, any of which can fail, as well as the traits: Debug, Clone, Display, PartialOrd,
/// and PartialEq.
#[allow(missing_docs)]
pub trait Num: fmt::Debug + fmt::Display + Clone + PartialOrd + PartialEq where
	Self: Sized
{
	/// Attempts to create an instance of the number from an f64
	fn from_f64(t: f64) -> Calculation<Self>;
	/// Attempts to create an instance of the number from complex parts. It's possible the imaginary
	/// part will be ignored for Numbers that don't support it.
	fn from_f64_complex(t: (f64, f64)) -> Calculation<Self>;
	
	/// Attemps partial_cmp and returns MathError::CmpError if it fails. Useful for `?` operator
	/// in calculations. It's possibly a good idea to re-implement it.
	fn tryord(&self, other: &Self) -> Result<Ordering, MathError> {
		if let Some(ord) = self.partial_cmp(other) {
			Ok(ord)
		} else {
			Err(MathError::CmpError)
		}
	}
	
	fn add(&self, other: &Self) -> Calculation<Self>;
	fn sub(&self, other: &Self) -> Calculation<Self>;
	fn mul(&self, other: &Self) -> Calculation<Self>;
	fn div(&self, other: &Self) -> Calculation<Self>;
	fn pow(&self, other: &Self) -> Calculation<Self>;
	fn sqrt(&self) -> Calculation<Self>;
	fn nrt(&self, other: &Self) -> Calculation<Self>;
	fn abs(&self) -> Calculation<Self>;
	fn sin(&self) -> Calculation<Self>;
	fn cos(&self) -> Calculation<Self>;
	fn tan(&self) -> Calculation<Self>;
	fn asin(&self) -> Calculation<Self>;
	fn acos(&self) -> Calculation<Self>;
	fn atan(&self) -> Calculation<Self>;
	fn atan2(&self, other: &Self) -> Calculation<Self>;
	fn floor(&self) -> Calculation<Self>;
	fn ceil(&self) -> Calculation<Self>;
	fn round(&self) -> Calculation<Self>;
	fn log(&self, other: &Self) -> Calculation<Self>;
}