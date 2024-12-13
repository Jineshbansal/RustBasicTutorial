use std::thread;

fn main() {     
    let (tx, rx) = std::sync::mpsc::channel();   
    let handle = thread::spawn(move || { 
        for i in 0..10000{
            println!("{}",i);
        }        
        tx.send("Hello from the thread!").unwrap();     
    });
    println!("{}", rx.recv().unwrap());
    /* 
    you should not use unwrap as it will panic if the result is an error. 
    */
    let value=rx.recv();
    match value{
        Ok(value)=>println!("{}",value),
        Err(e)=>println!("Error: {}",e),
    }

} 

   