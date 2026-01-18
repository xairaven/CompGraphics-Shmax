pub mod animations;
pub mod conversion;
pub mod fractals;
pub mod pipeline;
pub mod projections;
pub mod smooth;
pub mod units;
pub mod viewport;
pub mod figures {
    pub mod contour;
    pub mod detail;
    pub mod epicycloid;
    pub mod grid;
    pub mod grid3d;
    pub mod star3d;
    pub mod surface;
    pub mod texture;
}
pub mod math {
    pub mod angle;
}
pub mod primitives {
    pub mod circle;
    pub mod line2d;
    pub mod line3d;
    pub mod point2d;
    pub mod point3d;
    pub mod vector2d;
}
pub mod shapes {
    pub mod dot;
    pub mod shape;
    pub mod square;
}
pub mod transformations {
    pub mod euclidean {
        pub mod offset;
        pub mod offset3d;
        pub mod rotation;
        pub mod rotation3d;
    }
    pub mod affine;
    pub mod projective;
}
