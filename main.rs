const BIN: u32 = 2;
fn main(){
    /*
    const BASE: u32 = 10;
    let x = 5;
    let string = "Jello";
    let mut mutable;
    mutable = true;
    println!("Base: {}",BIN);
    println!("Base: {}",BASE);
    println!("value: {} {}",x,string);
    println!("This is mutable,correct? {}",mutable);
    println!("Hello World!");
    println!("A value: {}",test(2));

    let mut y = 1;
    y = y+1;
    println!("{}",y);
    let mut array = [0,0,0,0,0];
    array[0] = 1;
    println!("{}",array[0]);
    array.push(1);
    println!("{:?}",array);
    let mut arr = Vec::new();
    test2(arr);
    */
    let mut arr = [1,2,3,4,5,6];
    for (i,item) in arr.iter().enumerate(){
        println!("{}",*item);
        println!("{}",*item+1)
     }

}

fn test(n:i32) ->i32{
    n+1
}
