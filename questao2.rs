fn main(){

   
    let mut x:i32 =0;
    let mut a:i32 =1;
    let mut b:i32 =1;
    
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
    a = 1;
    b = 0;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
    a=0;
    b=1;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
     a=0;
     b=0;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);


}