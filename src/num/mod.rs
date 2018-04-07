
use std::fmt;
use std::marker::Sized;
use std::cmp::Ordering;

#[cfg(feature = "rug")]
mod complexrugrat;
#[cfg(feature = "rug")]
mod rugrat;
#[cfg(feature = "rug")]
mod rugcomplex;
mod complexfloat;
mod float64;

#[cfg(feature = "rug")]
pub use self::complexrugrat::ComplexRugRat;
#[cfg(feature = "rug")]
pub use self::complexfloat::ComplexFloat;

use opers::Calculation;
use errors::MathError;
use context::Context;

/// A `Num` represents any type that can be used in an expression. It requires lots of operations to
/// be implemented for it, any of which can fail, as well as the traits: Debug, Clone, Display, PartialOrd,
/// and PartialEq.
#[allow(missing_docs)]
pub trait Num: fmt::Debug + fmt::Display + Clone + PartialEq where
	Self: Sized
{
	/// Attempts to create an instance of the number from an f64
	fn from_f64(t: f64, ctx: &Context<Self>) -> Calculation<Self>;
	/// Attempts to create an instance of the number from complex parts. It's possible the imaginary
	/// part will be ignored for Numbers that don't support it.
	fn from_f64_complex(t: (f64, f64), ctx: &Context<Self>) -> Calculation<Self>;
	
	fn tryord(&self, other: &Self, ctx: &Context<Self>) -> Result<Ordering, MathError>;
	fn add(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn sub(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn mul(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn div(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn pow(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn sqrt(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn nrt(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn abs(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn sin(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn cos(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn tan(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn asin(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn acos(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn atan(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn atan2(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
	fn floor(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn ceil(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn round(&self, ctx: &Context<Self>) -> Calculation<Self>;
	fn log(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self>;
}