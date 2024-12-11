fn main() {
    println!("Hello, world!");
    let num: i32= 5;
    let result: i32 = fibo(num);
    println!("Fibonacci of {} is {}", num, result);
}

fn fibo(num:i32)->i32{
    let mut first=0;
    let mut second=1;
    if num==0{
        return first;
    }
    if num==1{
        return second;
    }
    for _ in 1..num{
        let temp=second;
        second=first+second;
        first=temp;
    }
    return second;
}
