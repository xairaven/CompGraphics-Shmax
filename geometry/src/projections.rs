use nalgebra::Matrix4;

pub mod twopoint;

pub trait Projection {
    fn matrix(&self) -> Matrix4<f64>;
}
