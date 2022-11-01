fn main() {
    let r = 10;
    let plane = vec![vec![0; r]; r];

    for line in plane.iter() {
        for num in line.iter() {
            print!("{} ", num);
        }
        
        println!();
    }
}
