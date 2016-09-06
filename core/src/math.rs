extern crate cgmath;

pub use cgmath::*;

pub type DefaultFloat = f64;

pub type Vector2f = Vector2<DefaultFloat>;
// pub type Vector2i = Vector2<DefaultInt>;
// pub type Vector2u = Vector2<DefaultUnsigned>;

pub type Vector3f = Vector3<DefaultFloat>;
// pub type Vector3i = Vector3<DefaultInt>;
// pub type Vector3u = Vector3<DefaultUnsigned>;

pub type Vector4f = Vector4<DefaultFloat>;
// pub type Vector4i = Vector4<DefaultInt>;
// pub type Vector4u = Vector4<DefaultUnsigned>;

pub type Point2f = Point2<DefaultFloat>;
// pub type Point2i = Point2<DefaultInt>;
// pub type Point2u = Point2<DefaultUnsigned>;

pub type Point3f = Point3<DefaultFloat>;
// pub type Point3i = Point3<DefaultInt>;
// pub type Point3u = Point3<DefaultUnsigned>;

pub type Matrix2f = Matrix2<DefaultFloat>;
pub type Matrix3f = Matrix3<DefaultFloat>;
pub type Matrix4f = Matrix4<DefaultFloat>;
