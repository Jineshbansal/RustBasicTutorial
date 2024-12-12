 
fn main() {
    /*generics */
    let ans=largest([1,3], [2,3]);
    println!("The largest string is: {:?}", ans);
}

fn largest<T: std::cmp::PartialOrd>(a:T,b:T)->T{
    if a>b{
        a
    }else{
        b
    }
}

   