fn deal(arr:[u32;10]) -> String{
    let mut buf = String::with_capacity("Hand 1 won".len());
    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();


    for (i,item) in arr.iter().enumerate(){
        if i%2 == 0{
            hand1.push(*item);
        }else{
            hand2.push(*item);
        }        
    }
    hand1.sort();
    hand2.sort();

    println!("{:?}",hand1);
    println!("{:?}",hand2);

    buf.push('c');
    return buf;
}

fn winner(n:i32) -> i32{
    return 1
}
// Winner function:
//High card
fn highcard(arr:[u32;5]){
    for (i,item) in arr.iter().enumerate(){
        if i%2 == 0{
            hand1.push(*item);
        }else{
            hand2.push(*item);
        }        
    }
}

fn main(){
    let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    println!("{}",a);
}