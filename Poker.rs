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
//High card             (Gets the highest value?)
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
    This function includes:
    pair
    2 pairs 
    3 of a kind
    4 of a kind
    full house
*/

fn hasOrder(arr:[u32;5]) -> Vec<u32>{
    let mut result = Vec::new();
    let mut tp = Vec::new();
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

    // This needs to return something else rather than pushing it in the result vector
    if result.len() == 4{
        if result[1] + 13 == result[2]{
            tp.push(4);
        } else{
            tp.push(2);
        }
    }
    else if result.len() == 3{
        tp.push(3);
    }
    else if result.len() == 2{
        tp.push(3);
    }
    
    return result;
}


//Straight types
/*
This includes:
straight
straight flush
royal flush
*/
fn hasStraight(arr:[u32;5]) -> u32{
    let mut temp = arr[0]%13;
    for i in 1..5{
        if temp+1 == arr[i]%13 {
            temp+=1;
        }else{
            return 0;
        }
    }
    return arr[0];
}


//Flush
/*
    Check to see if the hand has flush, if yes return the order of the flush(1 for club 2 for diamond ....) 
*/
// fn hasFlush(arr:[u32;5]) -> u32{
//     let mut arr = a;
// }



fn main(){
    let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    println!("{}",a);
    println!("{:?}",highcard([2,3,15,16,28]));
    println!("{:?}",hasOrder([2,3,15,16,28]));
    println!("{}",hasStraight([1,2,3,4,5]));
    println!("{}",hasStraight([1,2,3,4,6]));
}