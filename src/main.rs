mod convert;
fn main() {
    let num = convert::less_than_1000(999);
    println!("1  =>  {}  =>  {}  =>  {}",num.0,num.1,num.2);

}
