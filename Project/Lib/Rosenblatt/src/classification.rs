extern crate rand;
extern crate ndarray;
use rand::Rng;

fn generate_rand_f32vector(size: i16, start: f32, end: f32) -> Vec<f32>{
    let mut vector: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    for _it in 0..size{
        vector.push(rng.gen_range(start, end));
    }
    return vector;
}

fn predict_linear_classification(w:&Vec<f32>, xk:&Vec<f32>)-> i8{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return if sum >= 0.0 { 1 } else { -1 }
}

fn train_rosenblatt(w:&mut Vec<f32>, x:&Vec<Vec<f32>>, y:&Vec<i8>, nb_iter:i32, alpha:f32) {
    for _it in 0..nb_iter {
        let k = 0;
        let gxk = predict_linear_classification(w,&x[k]);
        for i in 0..x[1].len() {
            w[i + 1] += alpha * (y[k] - gxk as i8) as f32 * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk as i8) as f32;
    }
}

fn main(){
    //classification
    let mut w = generate_rand_f32vector(3, 0.0, 1.0);
    let x = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    let y = vec![-1,1];
    
    println!("{:?}",w);
    train_rosenblatt(&mut w, &x, &y, 15, 0.1);
    println!("{:?}",w);

    //regression
    /*
    let mut x: Vec<f32> = vec![];
    let mut y: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..20{
        x.push(i as f32);
        y.push(1.5 * (i as f32) + 0.75 + ((rng.gen_range(0.0,1.0) - 0.35) * 2.0));
    }
    println!("{:?}",x);
    println!("{:?}",y);
*/
}

