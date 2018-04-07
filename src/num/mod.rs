//! This module contains the `Num` trait and its implementations.
//!
//! The `Num` trait defines the inner workings of this library. Any type that implements the `Num` trait
//! can be used to represent a number in an equation. There are currently five predefined implementors
//! of the `Num` trait, but that number is subject to change (with additions and removals). You can also
//! define your own `Num`, but hopefully a fitting one already exists for you here.
//!
//! The five nums are:
//! - `f64`
//! - `ComplexFloat`
//! - `ComplexRugRat`
//! - `rug::Complex`
//! - `rug::Rational`
//!
//! Each have different strengths and weaknesses.
//!
//! `f64` implements all functions, but suffers the limitations `f64`s usually suffer from (low precision,
//!  NaN/infinity errors, etc).
//!
//! `ComplexFloat` is just two `f64`s representing a real part and an imaginary part, but doesn't
//! support nearly as many operations as `f64`.
//!
//! `ComplexRugRat` is two `rug::Rationals` representing a real and an imaginary part. This supports
//! even fewer operations than `ComplexFloat`.
//!
//! `rug::Complex` is the next best after `f64`. It's a complex multiple precision floating point
//! number. It's precision can be defined in the `Context` and equation is parsed an evaluated with.
//!
//! `rug::Rational` is just a rational number, and also supports very few operations.
//!
//! To see the progress on implementations of `Num` types, see the the [`issues on GitHub`](https://github.com/IntrepidPig/mexprp/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3Anumber)
//! with the label "number"

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