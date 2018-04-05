use std::fmt;
use std::marker::Sized;
use std::cmp::PartialOrd;

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

pub trait Num: fmt::Debug + fmt::Display + Clone where
	Self: Sized
{
	fn from_f64(t: f64) -> Calculation<Self>;
	fn from_f64_complex(t: (f64, f64)) -> Calculation<Self>;
	
	fn add(&self, other: &Self) -> Calculation<Self>;
	fn sub(&self, other: &Self) -> Calculation<Self>;
	fn mul(&self, other: &Self) -> Calculation<Self>;
	fn div(&self, other: &Self) -> Calculation<Self>;
	fn pow(&self, other: &Self) -> Calculation<Self>;
	fn sqrt(&self) -> Calculation<Self>;
	fn sin(&self) -> Calculation<Self>;
}