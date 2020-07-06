use rand::Rng;
use nalgebra::{DMatrix};
use rand::seq::index::sample;

struct RBF { w: DMatrix<f64> , x: DMatrix<f64> , gamma : f64 , }

pub extern fn init_linear_model(size: usize, it : usize) -> f64 {
    let mut vector = Vec::new();
    let mut rng = rand::thread_rng();
    (rng.gen_range(it) % size as f64 as usize) as f64
}
pub  extern  fn init_rbf(w : DMatrix<f64> , x :DMatrix<f64> , gamma: f64) -> RBF{
    let r = RBF{
        w,
        x,
        gamma
    };
    return r;
}
pub extern  fn rbf_naive_predict_classification() -> f64{
    if(){
        1.0
    }
    else{
        -1.0
    }

}

pub  extern fn train_native_rbf (x : *mut f64, y : *mut f64, input_per_sample: f64, nbSample : f64, gamma : f64 ) -> RBF{
    let  Xm =  DMatrix::from_row_slice(input_per_sample,nbSample,&x);
    let Ym = DMatrix::from_row_slice(input_per_sample,1,&y);
    let phiMat = DMatrix::from_row_slice(input_per_sample,nbSample,0);
    unsafe {
        for i in nbSample {
            for j in input_per_sample {
                let alpha = Xm.row(i) - Xm.row(j);
                let toexp = -gamma * (alpha.pow(2));
                phiMat(i, j) = toexp.exp();
            }
        }
    }
    let Wm = phiMat.try_inverse() * Ym;
   let rbf =  init_rbf(Wm,Xm,gamma);
    return rbf;
}
