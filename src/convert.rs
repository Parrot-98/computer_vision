use ndarray::Array3;

pub fn convert(img: &Vec<f32>) -> Array3<f32> {
    Array3::from_shape_vec((224, 224, 3), img.clone())
        .expect("Shape dimensions do not match the total vector length")
}