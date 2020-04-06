fn main(){
    
    let arr = [42,2,43,4,45,6,44,8,41,9];
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

    if has_flush(&hand1) {
        println!("Hand has a flush\n");
    } else {
        println!("Hand does not have a flush\n");
    }

    if has_flush(&hand1) {
        println!("Hand has a flush\n");
    } else {
        println!("Hand does not have a flush\n");
    }

    if has_straight(&hand1) {
        println!("Hand has a straight\n");
    } else {
        println!("Hand does not have a straight\n");
    }

    if has_straight_flush(&hand1) {
        println!("Hand has a straight flush\n");
    } else {
        println!("Hand does not have a straight flush\n");
    }

}

/* Function to check if hand is a flush
 * Sort first, and compare first and last card */
fn has_flush(hand: &Vec<u32>) -> bool
{
    if get_suit(hand[0]) == get_suit(hand[4]) {
        true
    }
    else {
        false
    }
}

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


/* Function to check if hand has a straight */
fn has_straight(hand: &Vec<u32>) -> bool
{
    let mut temp = hand[0]%13;
    for i in 1..5{
        if temp+1 == hand[i]%13 {
            temp+=1;
        }else{
            return false;
        }
    }
    true
}

/* Function to check if hand has a straight flush */
fn has_straight_flush(hand: &Vec<u32>) -> bool
{
    has_flush(hand) && has_straight(hand)
}

/* Function to get the higest card */
// fn get_high_card(hand: &Vec<u32>) -> u32
// {
//     let mut temp = hand[0];
//     for i in 1..5{
//         if 
//     }
// }

