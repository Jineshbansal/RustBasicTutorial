use std::thread;

fn main() {     
    let (tx,rc)= std::sync::mpsc::channel();
    for i in 0..10 {
        let producer=tx.clone();
        thread::spawn(move || {
            let mut ans:u64=0;
            for j in 0..1000000 {
                ans+=1000000*i+j;
                if i==9 && j==999999 {
                    println!("Thread {} finished",1000000*i+j);
                }
            }
            producer.send(ans).unwrap();
        });
    }
    let mut ans=0;
    drop(tx);
    for val in rc {
        ans+=val;
    }
    let mut sum:u64=0;
    for i in 1..10000000 { 
        sum+=i;
    }
    println!("{}",ans);
    println!("{}",sum);

} 

   