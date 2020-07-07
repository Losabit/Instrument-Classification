use rand::Rng;
use nalgebra::{DMatrix};
use rand::seq::index::sample;

struct RBF { w: DMatrix<f64> , x: DMatrix<f64> , gamma : f64 , }

pub extern "C" fn randvalue(size: usize, it : usize) -> f64 {
    let mut rng = rand::thread_rng();
    let myrand = (rng.gen_range(0,it) % size as f64 as usize) as f64;
    return myrand;
}
pub  extern  "C" fn init_rbf(w : DMatrix<f64> , x :DMatrix<f64> , gamma: f64) -> RBF{
    let r = RBF{
        w,
        x,
        gamma
    };
    return r;
}
pub extern "C" fn rbf_naive_predict_classification() -> f64{
    if(){
        1.0
    }
    else{
        -1.0
    }

}

pub  extern "C" fn train_native_rbf (x : * f64, y : * f64, input_per_sample: f64, nbSample : f64, gamma : f64 ) -> RBF{
    let xm =  DMatrix::from_row_slice(input_per_sample, nbSample, &x);
    let ym = DMatrix::from_row_slice(input_per_sample, 1, &y);
    let phi_mat = DMatrix::from_row_slice(input_per_sample, nbSample, 0);
    unsafe {
        for i in nbSample {
            for j in input_per_sample {
                let alpha = xm.row(i) - xm.row(j);
                let toexp = -gamma * (alpha.pow(2));
                phi_mat(i, j) = toexp.exp();
            }
        }
    }
    let wm = phi_mat.try_inverse() * ym;
   let rbf =  init_rbf(wm, xm, gamma);
    return rbf;
}
