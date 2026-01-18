use crate::math::angle::Angle;
use crate::pipeline::{Operation3D, Pipeline3D};
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;
use nalgebra::Matrix4;

#[derive(Debug, Default)]
pub struct EuclideanRotation3D {
    pub is_enabled: bool,

    pub angle_x: f64,
    pub angle_y: f64,
    pub angle_z: f64,
}

impl EuclideanRotation3D {
    pub fn handle(&mut self, operators: Vec<&mut Pipeline3D>) {
        if !self.is_enabled {
            return;
        }

        for pipeline in operators {
            pipeline.add_operation(self.create_operation())
        }

        self.reset();
    }

    fn create_operation(&mut self) -> Operation3D {
        Operation3D::Rotation(Rotation3DOperation {
            angle_x: Angle::from_degree(self.angle_x),
            angle_y: Angle::from_degree(self.angle_y),
            angle_z: Angle::from_degree(self.angle_z),
        })
    }

    pub fn run(&mut self) {
        self.is_enabled = true;
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[derive(Debug)]
pub struct Rotation3DOperation {
    pub angle_x: Angle,
    pub angle_y: Angle,
    pub angle_z: Angle,
}

impl Default for Rotation3DOperation {
    fn default() -> Self {
        Self {
            angle_x: Angle::from_degree(0.0),
            angle_y: Angle::from_degree(0.0),
            angle_z: Angle::from_degree(0.0),
        }
    }
}

impl Rotation3DOperation {
    pub fn go(&self, point: &mut Point3D, pivot: &mut Point3D) {
        let matrix_to_origin = self.matrix_offset_to_origin(pivot);
        let matrix_from_origin = self.matrix_offset_to_point(pivot);
        let matrix_ox = self.matrix_around_ox();
        let matrix_oy = self.matrix_around_oy();
        let matrix_oz = self.matrix_around_oz();

        let result_matrix: Matrix4<f64> =
            matrix_to_origin * matrix_ox * matrix_oy * matrix_oz * matrix_from_origin;
        self.update_point(point, &result_matrix);
    }

    fn update_point(&self, point: &mut Point3D, result_matrix: &Matrix4<f64>) {
        let vector = point.to_vector();

        let result = vector * result_matrix;

        *point = Point3D {
            x: Centimeter(result.x),
            y: Centimeter(result.y),
            z: Centimeter(result.z),
        };
    }

    fn matrix_around_ox(&self) -> Matrix4<f64> {
        let angle = self.angle_x.radian();

        Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f64::cos(angle),
            f64::sin(angle),
            0.0,
            0.0,
            -f64::sin(angle),
            f64::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_around_oy(&self) -> Matrix4<f64> {
        let angle = self.angle_y.radian();

        Matrix4::new(
            f64::cos(angle),
            0.0,
            -f64::sin(angle),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            f64::sin(angle),
            0.0,
            f64::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_around_oz(&self) -> Matrix4<f64> {
        let angle = self.angle_z.radian();

        Matrix4::new(
            f64::cos(angle),
            f64::sin(angle),
            0.0,
            0.0,
            -f64::sin(angle),
            f64::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_offset_to_origin(&self, pivot: &Point3D) -> Matrix4<f64> {
        Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            -pivot.x.value(),
            -pivot.y.value(),
            -pivot.z.value(),
            1.0,
        )
    }

    fn matrix_offset_to_point(&self, pivot: &Point3D) -> Matrix4<f64> {
        Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            pivot.x.value(),
            pivot.y.value(),
            pivot.z.value(),
            1.0,
        )
    }
}
