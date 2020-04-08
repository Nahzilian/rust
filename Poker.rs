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
// Winner function:
//High card             (Gets the highest value?)
fn highcard(arr:&Vec<u32>) -> u32{
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


fn check_match(arr:&Vec<u32>) -> Vec<u32>{
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

fn has_pair(hand: &Vec<u32>) -> bool{
    return hand.len() == 2;
}

fn has_three_of_kind(hand: &Vec<u32>) -> bool{
    return hand.len() == 3;
}

fn has_two_pairs(hand: &Vec<u32>) -> bool{
    return hand.len() == 4;
}

fn has_four_of_kind(hand: &Vec<u32>) -> bool{
    let mut check = true;
    for (i,item1) in hand.iter().enumerate(){
        for (j,item2) in hand.iter().enumerate(){
            if *item1 % 13 != *item2 %13{
               check = false;
            }
        }
    }
    return check && has_two_pairs(hand);
}

fn has_full_house(hand: &Vec<u32>) -> bool{
    return hand.len() == 5;
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

/*
    Checking winner: if 1 => hand 1 win, if 2 => hand 2 win, if tie 3
*/
fn winner(hand1:&Vec<u32>,hand2:&Vec<u32>) -> u32{
    let mut a = check_match(hand1);
    let mut b = check_match(hand2);
    if has_royal_flush(hand1)&&has_royal_flush(hand2){
        return 3;
    }else if has_straight_flush(hand1)&&has_straight_flush(hand2){
        if highcard(hand1) > highcard(hand2){
            return 1;
        }else{
            return 2;
        }
    }else if has_four_of_kind(&a) && has_four_of_kind(&b){
        return 3; //
    }else if has_full_house(&a) && has_full_house(&b){
        return 3;
    }else if has_flush(hand1) && has_flush(hand2){
        return 3;
    }else if has_straight(hand1) && has_straight(hand2){
        return 3;
    }else if has_three_of_kind(&a) && has_three_of_kind(&b){
        return 3;
    }else if has_two_pairs(&a) && has_two_pairs(&b){
        return 3;
    }else if has_pair(&a) && has_pair(&b){
        return 3;
    }else{
        // add when different rank
        if highcard(hand1) > highcard(hand2){
            return 1;
        }else{
            return 2;
        }
    }
}


fn main(){

    // let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    // println!("{}",a);
    // println!("{:?}",highcard([1,14,27,40,52]));
    // println!("{:?}",hasOrder([2,3,15,16,28]));
    let mut hand1 = Vec::new();
    //let mut hand2 = Vec::new();

    /**
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
    **/

    hand1.push(1);
    hand1.push(2);
    hand1.push(3);
    hand1.push(4);
    hand1.push(5);

    
    println!("{:?}",check_match(&hand1));


}