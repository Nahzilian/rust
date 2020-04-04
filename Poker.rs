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

    //winner(hand1,hand2);
    buf.push('c');

    return buf;
}

fn winner(arr1:[u32;5],arr2:[u32;5]){
    let mut winner = 0;
    if highcard(arr1) > highcard(arr2){
        winner = 1;
    }else{
        winner = 2;
    }
    println!("In progress");
}
// Winner function:
//High card
fn highcard(arr:[u32;5]) -> u32{
    let mut result = 0;
    for (i,item) in arr.iter().enumerate(){
        if (*item == 1) || (*item == 14) || (*item == 27) || (*item == 40){
            if result < *item{result = *item;}
            result = *item
        }else{
            if result < *item && (result !=1) && (*item != 40) && (*item != 14) && (*item != 27) {
                result = *item;
            }
        }        
    }
    println!("{:?}",arr);
    return result;
}
//Has pair
/*
    loop through the whole code and find the pairs, return the vector for all the pairs.
*/

fn hasPair(arr:[u32;5]) -> Vec<u32>{
    let mut result = Vec::new();
    let mut temp = 0;
    for (i,item1) in arr.iter().enumerate(){
        temp = *item1;
        for (j,item2) in arr.iter().enumerate(){
            if (temp + 13 == *item2) || (temp + 13*2 == *item2) || (temp + 13*3 == *item2)|| (temp + 13*4 == *item2){
                result.push(*item1);
                result.push(*item2);
            }
        }
    }
    if result.is_empty{
        
    }
    return result;
}

//has two pair
//fn hasTwoPair(arr:[u32;5]){
    //let mut result = Vec::new();
    //let mut count = 0;
    //for (i,item) in arr.iter().enumerate(){
        
    //}


  
//}
fn main(){
    let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    println!("{}",a);
    println!("{:?}",highcard([2,3,4,5,6]));
    println!("{:?}",hasPair([1,2,14,15,9]));
}