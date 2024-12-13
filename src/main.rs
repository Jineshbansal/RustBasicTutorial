use std::thread;

fn main() {    

    let v=vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    thread::spawn(move ||{
        for i in v.iter(){
            println!("Thread 1: {}", i);
        }
    });

} 

   