mod convert;
fn main() {
    let num = convert::less_than_10k(1);
    println!("1  =>  {}  =>  {}  =>  {}",num.0,num.1,num.2);

}
