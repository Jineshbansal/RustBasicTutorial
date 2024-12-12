 
fn main() {
    let v1=vec![1,2,3,4,5];
    // to borrow it mutably used iter_mut 
    let v1_iter=v1.iter();
    let total:i32=v1_iter.sum();
    println!("Sum: {}",total);
    println!("v1: {:?}",v1);
    /*  It is same as 
    for val in v1{
        println!("Got: {}",val);
    }   
    In iter methods work by borrowing
    We can't mutate variables as refernce passed is immutable 
    */
     
    // For loop uses into_iter by default
    // for val in v1_iter{
    //     println!("Got: {}",val);
    // }
    /*
    Another way to iterate is to use into_iter
    let mut iter=v1.iter();
    while let Some(val)=iter.next(){
        println!("Got: {}",val);
    }
    
     */
    
    /*
    into_iter take ownership of the collection
     */

    /*
    Consuming adapters and iterator adaptors
    let v1_iter=v1.iter();
    let total=v1_iter.sum();
    you can't use v1_iter after this as it has been consumed
    
    Iterator adaptors  don't consume iterator.Instead they produce different iterators by changing some aspect of the original  iterator.
     */

}

   