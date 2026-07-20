use ndarray::{Array2, Array};

pub fn max_pooling(img: &ndarray::ArrayView2<f32>, dim: i32) -> Array2<f32> {
    let dim = dim as usize;
    let out_rows = img.shape()[0] / dim;
    let out_cols = img.shape()[1] / dim;

    let flat_data: Vec<f32> = img.exact_chunks((dim, dim))
        .into_iter()
        .map(|window| *window.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
        .collect();

    Array::from_shape_vec((out_rows, out_cols), flat_data)
        .expect("Error resizing max pooling output")
}