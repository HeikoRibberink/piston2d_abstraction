use piston_window::image;
use piston_window::Context;
use piston_window::G2dTexture;
use piston_window::Graphics;

pub trait Renderable {
	fn get_render_transform(&self) -> &Matrix2d;
	fn get_render_texture(&self) -> &piston_window::G2dTexture;
}

pub fn render(
	c: Context,
	g: &mut impl Graphics<Texture = G2dTexture>,
	rs: &[&dyn Renderable],
) {
	let m = Matrix2d::scale(2.0, -2.0) * Matrix2d::translate(-1.0, 1.0);

	let [width, height] = c.get_view_size();
	let m = m * Matrix2d::scale(height as f64 / width as f64, 1.0);
	for r in rs {
		let tr = *r.get_render_transform() * m;
		let t = r.get_render_texture();
		image(t, *tr, g);
	}
}

//Matrix2d
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2d {
	pub raw: [[f64; 3]; 2],
}

impl Matrix2d {
	pub fn raw(m: [[f64; 3]; 2]) -> Self {
		Matrix2d { raw: m }
	}
	pub fn identity() -> Self {
		Matrix2d {
			raw: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
		}
	}

	pub fn scale(sx: f64, sy: f64) -> Self {
		Matrix2d {
			raw: [[sx, 0.0, 0.0], [0.0, sy, 0.0]],
		}
	}

	pub fn translate(tx: f64, ty: f64) -> Self {
		Matrix2d {
			raw: [[1.0, 0.0, tx], [0.0, 1.0, ty]],
		}
	}

	pub fn rotate(a: f64) -> Self {
		let c = a.cos();
		let s = a.sin();
		Matrix2d {
			raw: [[c, -s, 0.0], [s, c, 0.0]],
		}
	}
}

pub mod matrix_2d {
	use super::Matrix2d;
	use piston_window::math::multiply;
	use std::ops;
	//MatrixMul
	impl ops::Mul<Matrix2d> for Matrix2d {
		type Output = Self;
		fn mul(mut self, m: Self) -> Self {
			self.raw = multiply(m.raw, self.raw);
			self
		}
	}
	impl ops::MulAssign<Matrix2d> for Matrix2d {
		fn mul_assign(&mut self, m: Self) {
			self.raw = multiply(m.raw, self.raw);
		}
	}
	//Scale
	impl ops::Mul<[f64; 2]> for Matrix2d {
		type Output = Self;
		fn mul(self, m: [f64; 2]) -> <Self as std::ops::Mul<[f64; 2]>>::Output {
			self * Matrix2d::scale(m[0], m[1])
		}
	}
	impl ops::MulAssign<[f64; 2]> for Matrix2d {
		fn mul_assign(&mut self, m: [f64; 2]) {
			*self *= Matrix2d::scale(m[0], m[1]);
		}
	}
	//Move
	impl ops::Add<[f64; 2]> for Matrix2d {
		type Output = Self;
		fn add(self, m: [f64; 2]) -> <Self as std::ops::Add<[f64; 2]>>::Output {
			self * Matrix2d::translate(m[0], m[1])
		}
	}
	impl ops::AddAssign<[f64; 2]> for Matrix2d {
		fn add_assign(&mut self, m: [f64; 2]) {
			*self *= Matrix2d::translate(m[0], m[1]);
		}
	}
	//Rotate
	impl ops::Rem<f64> for Matrix2d {
		type Output = Self;
		fn rem(self, m: f64) -> <Self as std::ops::Rem<f64>>::Output {
			self * Matrix2d::rotate(m)
		}
	}
	impl ops::RemAssign<f64> for Matrix2d {
		fn rem_assign(&mut self, m: f64) {
			*self *= Matrix2d::rotate(m);
		}
	}
	impl ops::Mul<f64> for Matrix2d {
		type Output = Self;
		fn mul(self, m: f64) -> Self {
			Matrix2d {
				raw: {
					let mut out = [[0.0; 3]; 2];
					for i in 0..2 {
						for j in 0..3 {
							out[i][j] = self.raw[i][j] * m;
						}
					}
					out
				},
			}
		}
	}
	impl ops::MulAssign<f64> for Matrix2d {
		fn mul_assign(&mut self, m: f64) {
			for i in 0..2 {
				for j in 0..3 {
					self.raw[i][j] *= m;
				}
			}
		}
	}
	impl ops::Deref for Matrix2d {
		type Target = [[f64; 3]; 2];
		fn deref(&self) -> &<Self as ops::Deref>::Target {
			&self.raw
		}
	}
	impl ops::DerefMut for Matrix2d {
		fn deref_mut(&mut self) -> &mut <Self as ops::Deref>::Target {
			&mut self.raw
		}
	}

	impl From<[[f64; 3]; 2]> for Matrix2d {
		fn from(raw: [[f64; 3]; 2]) -> Self {
			Self { raw }
		}
	}

	impl Into<[[f64; 3]; 2]> for Matrix2d {
		fn into(self) -> [[f64; 3]; 2] {
			self.raw
		}
	}
}

mod test {
	#[test]
	fn assign_ops() {
		use super::{Matrix2d};
		let mut m = Matrix2d::identity();
		let c = &mut m;
		*c *= Matrix2d::rotate(0.5 * std::f64::consts::PI);
		assert_eq!(m, Matrix2d::identity());
	}
}