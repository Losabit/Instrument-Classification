use rand::Rng;
use nalgebra::{DMatrix, DMatrixSlice};
use rand::seq::index;

<<<<<<< Updated upstream
pub struct RBF { w: DMatrix<f64> , x: DMatrix<f64> , gamma : f64 , }

pub extern "C" fn randvalue(size: usize, it : usize) -> f64 {
    let mut rng = rand::thread_rng();
    let myrand = (rng.gen_range(0,it) % size as f64 as usize) as f64;
    return myrand;
}
pub  extern  "C" fn init_rbf(w : DMatrix<f64> , x :DMatrix<f64> , gamma: f64) -> RBF{
=======
pub struct RBF { pub(crate) w: DMatrix<f64> , pub(crate) x: DMatrix<f64> ,  }
pub extern  "C" fn init_rbf(w : DMatrix<f64> , x :DMatrix<f64> ) -> RBF{
    //println!("{:?}",w);
>>>>>>> Stashed changes
    let r = RBF{
        w,
        x
    };
    return r;
}
<<<<<<< Updated upstream
pub extern "C" fn rbf_naive_predict_classification() -> f64{
    if(true){
        1.0
    }
    else{
        -1.0
    }

}
/*
pub  extern "C" fn train_native_rbf (x : * f64, y : * f64, input_per_sample: f64, nbSample : f64, gamma : f64 ) -> RBF{
    let xm =  DMatrix::from_row_slice(input_per_sample, nbSample, &x);
    let ym = DMatrix::from_row_slice(input_per_sample, 1, &y);
    let phi_mat = DMatrix::from_row_slice(input_per_sample, nbSample, 0);
=======

pub  extern "C" fn train_native_rbf (x :  &[f64], y :  &[f64], input_per_sample: usize, nbSample : usize, gamma : f64 ) -> RBF{
    let xm =  DMatrix::from_row_slice(input_per_sample , nbSample ,x);
    let ym = DMatrix::from_row_slice(input_per_sample , 1 as usize, y);
    println!("{:?}", xm.row(0)[1]);
    let mut phi = Vec::new();
>>>>>>> Stashed changes
    unsafe {
        for i in 0.. input_per_sample  {
            for j in  0.. input_per_sample  {
               let mut sum = Vec::new();
                for k in 0..nbSample{
                    println!("{:?}",xm.row(i)[k] - xm.row(j)[k]);
                    sum.push(xm.row(i)[k] - xm.row(j)[k])
                }
                let alpha = DMatrix::from_row_slice(nbSample,1,&sum);
                let toexp = (-gamma * (alpha.norm() * alpha.norm()) ) ;
                phi.push( toexp.exp());
            }
        }
    }
    let phi_mat = DMatrix::from_row_slice(input_per_sample,input_per_sample,&phi);
    println!("{:?}", phi_mat);
    println!("{:?}", phi_mat.determinant());
    let wm = phi_mat.try_inverse().unwrap() * ym;
   let rbf =  init_rbf(wm, xm);
    return rbf;
}
*/
