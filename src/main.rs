mod convert;
fn main() {
    let num = convert::less_than_10k(9999);
    println!("1  =>  {}  =>  {}  =>  {}",num.0,num.1,num.2);
}
