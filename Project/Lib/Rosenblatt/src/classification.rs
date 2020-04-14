fn main(){
    println!("test");

}
/**
* Function Train_rosenblatt
*
*
*
**/
fn train_rosenblatt(mut w: Vec<Vec<f32>>, x: Vec<Vec<f32>>, y:&mut[f32], nb_iter:&mut i32, alpha : &mut i32){
   let i : i16 = 0;
    for it in 0 .. nb_iter  {
       let k:i16 = 0; // Random à mettre
        let gxk: i32 = 0;
        //gxk = predict_linear_classification(w,x[k]);
        for i  in 0.. x[1].len()  {
            w[i+1] += alpha * (y[k] - gxk ) * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk);
    }
}