use std::vec;


fn main() {
    let num=vec![1,2,3,4,5,6,7,8,9,10];
    let even_values=get_even_values(num);
    println!("Even values are {:?}",even_values);
}

fn get_even_values(num:Vec<i32>)->Vec<i32>{
    let mut even_values=Vec::new();
    for i in num{
        if i%2==0{
            even_values.push(i);
        }
    }
    even_values
}
 
   