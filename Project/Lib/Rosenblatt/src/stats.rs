extern crate rand;
extern crate gnuplot;
use rand::Rng;

fn average(x: &[f32]) -> f32{
    let mut average = 0.0;
    for i in 0..x.len(){
        average += x[i];
    }
    return average / x.len() as f32;
}

fn covariance(x: &[f32], y: &[f32]) -> f32{
    assert_eq!(x.len(), y.len());
    let mut value = 0.0;
    let average_x = average(x);
    let average_y = average(y);
    for i in 0..x.len(){
        value += (x[i] - average_x) * (y[i] - average_y);
    }
    value = value / x.len() as f32;
    return value;
}

fn variance(x: &[f32]) -> f32{
    return order_center_moment(x,2);
}

fn variation_coeff(x: &[f32]) -> f32{
    return (variance(x).sqrt() * 100.0) / average(x);
}

fn order_center_moment(x: &[f32], ordre: i32) -> f32{
    let mut value = 0.0;
    let average_x = average(x);
    for i in 0..x.len(){
        value +=  (x[i] - average_x).powi(ordre);
    }
    value = value / x.len() as f32;
    return value;
}

fn fisher_coeff(x: &[f32]) -> f32{
    return order_center_moment(x,3) / variance(x).sqrt().powi(3);
}

fn fisher_aplatissement(x: &[f32]) -> f32{
    return (order_center_moment(x,4) / variance(x).sqrt().powi(4)) - 3.0;
}

fn correlation_coeff(x: &[f32], y: &[f32]) -> f32{
    return covariance(x, y) / (variance(x).sqrt() * variance(y).sqrt());
}

fn main(){
    let mut x = [0f32;20];
    let mut y = [0f32;20]; 
    let mut rng = rand::thread_rng();
    for i in 0..20{
        x[i]  = (i + 1) as f32;  
        y[i] = 1.5 * ((i + 1) as f32) + 0.75 + ((rng.gen_range(0.0,1.0) - 0.35) * 2.0);
    }

    let a = covariance(&x, &y) / variance(&x);
    let b =  average(&y) - a * average(&x);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("CV = {:?}", correlation_coeff(&x,&y));

    let mut figure = gnuplot::Figure::new();
    figure.axes2d()
	.set_legend(gnuplot::Graph(0.5), gnuplot::Graph(0.9), &[], &[])
	.points(
		&x,
		&y,
		&[gnuplot::Caption("Points")],
    )
    .lines(
		&[x[0], x[19]],
		&[a * x[0] + b, a * x[19] + b],
		&[gnuplot::Caption("Regression")],
	);
    figure.show().unwrap();
}