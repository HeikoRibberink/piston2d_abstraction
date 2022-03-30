use lerp::Lerp;
use piston_window::image;
use piston_window::Context;
use piston_window::G2dTexture;
use piston_window::Graphics;
use piston_window::ImageSize;
use self::tex::*;

pub trait Renderable {
    fn get_transform(&self) -> &Matrix2d;
    fn get_texture(&self) -> &piston_window::G2dTexture;
    fn get_settings(&self) -> &Settings;
}

pub fn render(
    c: Context,
    g: &mut impl Graphics<Texture = G2dTexture>,
    settings: &RenderSettings,
    rs: &[&dyn Renderable],
) {
    let [width, height] = c.get_view_size();
    let mut m = Matrix2d::identity();
    m *= match settings.scaling {
        ScalingStrategy::Default => Matrix2d::scale(2.0, 2.0),
        ScalingStrategy::Pixel => Matrix2d::scale(2.0 / width, 2.0 / height),
        ScalingStrategy::SquareSidesLerpWidthHeight(l) => {
            Matrix2d::scale(2.0 / width, 2.0 / height) * width.lerp(height, l)
        }
        ScalingStrategy::Custom(ref f) => f([width, height]),
    };
    m *= match settings.origin {
        OriginStrategy::TopLeft => Matrix2d::translate(-1.0, -1.0),
        OriginStrategy::Middle => Matrix2d::translate(-0.0, -0.0),
        OriginStrategy::Custom(tx, ty) => Matrix2d::translate(tx, ty),
    };
    m *= match settings.flip {
        FlipStrategy::None => Matrix2d::identity(),
        FlipStrategy::Horizontal => Matrix2d::scale(1.0, -1.0),
        FlipStrategy::Vertical => Matrix2d::scale(-1.0, 1.0),
        FlipStrategy::HorizontalAndVertical => Matrix2d::scale(-1.0, -1.0),
    };
    for r in rs {
        let t = r.get_texture();
        let tr = r.get_settings().m * *r.get_transform() * m;
        image(t, *tr, g);
    }
}

pub struct RenderSettings {
    pub scaling: ScalingStrategy,
    pub flip: FlipStrategy,
    pub origin: OriginStrategy,
}

impl RenderSettings {}

pub enum ScalingStrategy {
    Default,
    Pixel,
    SquareSidesLerpWidthHeight(f64),
    Custom(Box<dyn Fn([f64; 2]) -> Matrix2d>),
}

pub enum FlipStrategy {
    None,
    Horizontal,
    Vertical,
    HorizontalAndVertical,
}

pub enum OriginStrategy {
    TopLeft,
    Middle,
    Custom(f64, f64),
}

pub mod tex {
    use super::*;
    #[derive(Copy, Clone)]
    pub struct Settings {
        pub(super) m: Matrix2d,
    }

    impl Settings {
        pub fn new(
            center: OriginStrategy,
            scaling: ScalingStrategy,
            texture: &piston_window::G2dTexture,
        ) -> Self {
            let (w, h) = texture.get_size();
            let w = w as f64;
            let h = h as f64;
            let c = match center {
                OriginStrategy::TopLeft => (0.0, 0.0),
                OriginStrategy::Middle => (-0.5, -0.5),
                OriginStrategy::Custom(tx, ty) => (-tx, -ty),
            };
            let m: Matrix2d = Matrix2d::translate(w * c.0, h * c.1)
                * match scaling {
                    ScalingStrategy::Pixel => Matrix2d::identity(),
                    ScalingStrategy::Square => Matrix2d::scale(1.0 / w, 1.0 / h),
                    ScalingStrategy::LerpWidthHeight(l) => {
                        Matrix2d::scale(1.0 / w.lerp(h, l), 1.0 / h.lerp(w, 1.0 - l))
                    }
                };
            Self { m }
        }
        pub fn pixel(texture: &piston_window::G2dTexture) -> Self {
            Self::new(
                OriginStrategy::TopLeft,
                ScalingStrategy::Pixel,
                texture,
            )
        }
        pub fn square(texture: &piston_window::G2dTexture) -> Self {
            Self::new(
                OriginStrategy::Middle,
                ScalingStrategy::Square,
                texture,
            )
        }
    }

    #[derive(Copy, Clone)]
    pub enum OriginStrategy {
        TopLeft,
        Middle,
        Custom(f64, f64),
    }

    #[derive(Copy, Clone)]
    pub enum ScalingStrategy {
        Pixel,
        Square,
        LerpWidthHeight(f64),
    }
}

//Matrix2d
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2d {
    raw: [[f64; 3]; 2],
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
    ///Matrix multiplication
    impl ops::Mul<Matrix2d> for Matrix2d {
        type Output = Self;
        fn mul(mut self, m: Self) -> Self {
            self.raw = multiply(m.raw, self.raw);
            self
        }
    }
    ///Matrix multiplication
    impl ops::MulAssign<Matrix2d> for Matrix2d {
        fn mul_assign(&mut self, m: Self) {
            self.raw = multiply(m.raw, self.raw);
        }
    }
    ///Matrix scaling
    impl ops::Mul<[f64; 2]> for Matrix2d {
        type Output = Self;
        fn mul(self, m: [f64; 2]) -> <Self as std::ops::Mul<[f64; 2]>>::Output {
            self * Matrix2d::scale(m[0], m[1])
        }
    }
    ///Matrix scaling
    impl ops::MulAssign<[f64; 2]> for Matrix2d {
        fn mul_assign(&mut self, m: [f64; 2]) {
            *self *= Matrix2d::scale(m[0], m[1]);
        }
    }
    ///Matrix translation
    impl ops::Add<[f64; 2]> for Matrix2d {
        type Output = Self;
        fn add(self, m: [f64; 2]) -> <Self as std::ops::Add<[f64; 2]>>::Output {
            self * Matrix2d::translate(m[0], m[1])
        }
    }
    ///Matrix translation
    impl ops::AddAssign<[f64; 2]> for Matrix2d {
        fn add_assign(&mut self, m: [f64; 2]) {
            *self *= Matrix2d::translate(m[0], m[1]);
        }
    }
    ///Matrix rotation
    impl ops::Rem<f64> for Matrix2d {
        type Output = Self;
        fn rem(self, m: f64) -> <Self as std::ops::Rem<f64>>::Output {
            self * Matrix2d::rotate(m)
        }
    }
    ///Matrix rotation
    impl ops::RemAssign<f64> for Matrix2d {
        fn rem_assign(&mut self, m: f64) {
            *self *= Matrix2d::rotate(m);
        }
    }
    ///Component-wise matrix scaling
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
    ///Component-wise matrix scaling
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

    // impl Into<[[f64; 3]; 2]> for Matrix2d {
    //     fn into(self) -> [[f64; 3]; 2] {
    //         self.raw
    //     }
    // }
}

mod test {
    #[test]
    fn assign_ops() {
        use super::Matrix2d;
        let mut m = Matrix2d::identity();
        let c = &mut m;
        *c *= Matrix2d::rotate(0.5 * std::f64::consts::PI);
        assert_eq!(m, Matrix2d::identity());
    }
}
