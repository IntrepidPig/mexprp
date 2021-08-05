use std::cmp::Ordering;


use rug::Complex;
use rug::ops::Pow;
use crate::opers::Calculation;
use crate::errors::MathError;
use crate::answer::Answer;
use crate::num::Num;
use crate::context::Context;

impl Num for Complex {
	fn from_f64(t: f64, ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(Complex::with_val(ctx.cfg.precision, t)))
	}

	fn from_f64_complex(val: (f64, f64), ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(Complex::with_val(ctx.cfg.precision, val)))
	}

	fn typename() -> String {
		String::from("Complex")
	}

	fn tryord(&self, other: &Self, _ctx: &Context<Self>) -> Result<Ordering, MathError> {
		if let Some(ord) = self.real().partial_cmp(other.real()) {
			Ok(ord)
		} else {
			Err(MathError::CmpError)
		}
	}

	fn add(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, self + other);

		Ok(Answer::Single(r))
	}

	fn sub(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, self - other);

		Ok(Answer::Single(r))
	}

	fn mul(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, self * other);

		Ok(Answer::Single(r))
	}

	fn div(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, self / other);

		Ok(Answer::Single(r))
	}

	fn pow(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Pow::pow(self, other));

		Ok(Answer::Single(r))
	}

	fn sqrt(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::sqrt_ref(self));

		Ok(if ctx.cfg.sqrt_both {
			Answer::Multiple(vec![r.clone(), -r])
		} else {
			Answer::Single(r)
		})
	}

	fn abs(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::abs_ref(self));

		Ok(Answer::Single(r))
	}

	fn sin(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::sin_ref(self));

		Ok(Answer::Single(r))
	}

	fn cos(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::cos_ref(self));

		Ok(Answer::Single(r))
	}

	fn tan(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::tan_ref(self));

		Ok(Answer::Single(r))
	}

	fn asin(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::asin_ref(self));

		Ok(Answer::Single(r))
	}

	fn acos(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::acos_ref(self));

		Ok(Answer::Single(r))
	}

	fn atan(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::with_val(ctx.cfg.precision, Complex::atan_ref(self));

		Ok(Answer::Single(r))
	}

	fn floor(&self, ctx: &Context<Self>) -> Calculation<Self> {
		// Floor definition for complex numbers as defined by WolframAlpha https://mathworld.wolfram.com/FloorFunction.html
		let r = Complex::real(self).floor_ref();
		let i = Complex::imag(self).floor_ref();
		let a = Complex::with_val(ctx.cfg.precision, (r, i));

		Ok(Answer::Single(a))
	}
	
	fn ceil(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::real(self).ceil_ref();
		let i = Complex::imag(self).ceil_ref();
		let a = Complex::with_val(ctx.cfg.precision, (r, i));
		
		Ok(Answer::Single(a))
	}
	
	fn round(&self, ctx: &Context<Self>) -> Calculation<Self> {
		let r = Complex::real(self).round_ref();
		let i = Complex::imag(self).round_ref();
		let a = Complex::with_val(ctx.cfg.precision, (r, i));
		
		Ok(Answer::Single(a))
	}

	fn log(&self, other: &Self, ctx: &Context<Self>) -> Calculation<Self> {
		let n = Complex::with_val(ctx.cfg.precision, Complex::log10_ref(self));
		let d = Complex::with_val(ctx.cfg.precision, Complex::log10_ref(other));
		let r = Complex::with_val(ctx.cfg.precision, n / d);

		Ok(Answer::Single(r))
	}
}
