 
fn main() {
    /*
    Consuming adapters and iterator adaptors
    let v1_iter=v1.iter();
    let total=v1_iter.sum();
    you can't use v1_iter after this as it has been consumed
    
    Iterator adaptors  don't consume iterator.Instead they produce different iterators by changing some aspect of the original  iterator.
     */  

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2_iter = v1_iter.map(|x| x + 1);
    let v3_iter = v2_iter.filter(|x| * x==2);
    for i in v3_iter {
        println!("{}", i);
    } 
    // converting iterator to vector
    let v: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v);

}

   