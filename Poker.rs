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
        if *item%13 == 1{
            if result < *item{result = *item;}
            result = *item
        }else if result < *item && (result%13 !=1){
                result = *item;
            }
        }        

    println!("{:?}",arr);
    return result;
}
//Has pair
/*
    This part includes:
    pair
    2 pairs 
    3 of a kind
    4 of a kind
    full house
*/


fn check_match(arr:[u32;5]) -> Vec<u32>{
    let mut result = Vec::new();
    for (i,item1) in arr.iter().enumerate(){
        for (j,item2) in arr.iter().enumerate(){
            if *item1 != *item2{
                if *item1 % 13 == *item2 % 13{
                    result.push(*item1);
                    result.push(*item2);
                }
            }
        }
    }
    result.sort();
    result.dedup();
    return result;
}



    // This needs to return something else rather than pushing it in the result vector
    if result.len() == 4{
        if result[1] + 13 == result[2]{
            tp.push(4);
        } else{
            tp.push(2);
        }
    }
    return has_two_pair(hand)&& result;
}

fn has_full_house(hand: &Vec<u32>) -> bool{
    return has_pair(hand) && has_three_of_kind(hand);
}


//-----------------------All functions below can be compared with highcard()---------------//
//Straight types
/*
This includes:
straight
straight flush
royal flush
*/
fn has_straight(hand: &Vec<u32>) -> bool{


    let mut valHand = Vec::new(); //New vect to hold the value (number) of the cards
    for i in 0..5{
        valHand.push(hand[i]%13);           
    }
    valHand.sort();
    for i in 0..5{
        println!("{} ", valHand[i])
    }
    let mut temp = valHand[0];

    if valHand[0]==0 && valHand[1]== 1 //Special case
    {
        temp = valHand[2];
        for i in 3..5{
            if temp+1 == valHand[i] {
                temp+=1;
            }else{
                return false;
            }
        }
        return true;
    }


    for i in 1..5{
        if temp+1 == valHand[i] {
            temp+=1;
        }else{
            return false;
        }
    }
    return true;
}


//Flush
/*
    Check to see if the hand has flush, if yes return the order of the flush(1 for club 2 for diamond ....) 
*/

fn has_flush(hand: &Vec<u32>) -> bool{
    let mut result = true;
    for (i,item1) in hand.iter().enumerate(){
        for(i,item2)in hand.iter().enumerate(){
            if get_suit(*item1) != get_suit(*item2) {
                result = false;
            }
        }
    }
    return result;
    
}

// Helper functions

/* Function to get the suit of a card */
fn get_suit(card: u32) -> char {
    if card <= 13 {
        return 'C';
    } else if card <= 26 {
        return 'D';
    } else if card <= 39 {
        return 'H';
    } else {
        return 'S';
    }
}



/* Function to check if hand has a straight flush */
fn has_straight_flush(hand: &Vec<u32>) -> bool{
    return has_flush(hand)&& has_straight(hand);
}

fn has_royal_flush(hand: &Vec<u32>) ->bool{
    return has_straight_flush(hand)&&(hand[0] == 1)&&(hand[1] == 10);
}

fn main(){

    // let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    // println!("{}",a);
    // println!("{:?}",highcard([1,14,27,40,52]));
    // println!("{:?}",hasOrder([2,3,15,16,28]));

    let arr = [38,2,50,4,13,6,40,8,41,9];
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

    for i in 0..5{
        println!("{} ", hand1[i])
    }

    if has_straight(&hand1) {
        println!("Hand has a straight\n");
    } else {
        println!("Hand does not have a straight\n");
    }

}