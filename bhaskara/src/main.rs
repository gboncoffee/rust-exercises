fn bhaskara(a: f64, b: f64, c:f64) -> Vec<f64> {

    let delta = b.powi(2) - (4.0 * a * c);

    let x = (-b + delta.sqrt()) / (2.0 * a);
    let y = (-b - delta.sqrt()) / (2.0 * a);

    vec![x, y]
}

fn main() {

    for root in &bhaskara(1.0, -1.0, -12.0) {
        print!("{} ", root);
    }
    println!();
}
