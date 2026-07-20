use ndarray::{ArrayView3, Array2};

pub fn convolution(img: &ArrayView3<f32>, dim: i32) -> Array2<f32> {
    let dim = dim as usize;
    
    let kernel_sum = (dim * dim) as f32;
    let kernel = Array2::<f32>::ones((dim, dim)) / kernel_sum; 
    
    let output_size = (224 - dim) + 1;
    let mut result = Array2::<f32>::zeros((output_size, output_size));

    for row in 0..output_size {
        for col in 0..output_size { // these to loops to move the kernel
            let mut sum = 0.0;

            for r in 0..dim {
                for c in 0..dim { // thise to to get the valuse the the kernel covers

                    let pixel_rgb_avg = (img[[row + r, col + c, 0]] + img[[row + r, col + c, 1]] + img[[row + r, col + c, 2]]) / 3.0;

                    sum += pixel_rgb_avg * kernel[[r, c]];
                }
            }
            
            result[[row, col]] = sum;
        }
    }

    result
}