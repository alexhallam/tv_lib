fn main() {
    // define path
    let in_file_path: &str = "examples/data/uspop.csv";
    // one liner
    let vec_vec_str = tv_lib::print::read_csv(in_file_path);
    //print
    println!("{:?}", vec_vec_str);
}
