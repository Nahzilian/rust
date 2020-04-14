/* Planning
10	Royal Flush
9	Straight Flush
8	4 of a kind
7	Full house
6 	Flush
5	Straight
4	3 of a kind
3 	Two Pair
2	1 Pair
1	High Card


Priority
===Tie breaking===

Change high card function
Make a new function similar to high card


+ Royal Flush       - Use highest card to determine (ACE) (INDEX 0)? -> Simply compare high card

+ Straight Flush    - Use Highest card to determine -> high_card + Make sure Ace is highest

+ 4 of a kind       - Get the higher four of a kind and compare -> Simply get higher 4 + make sure Ace is highest


+ Full house        - The highest 3 wins -> Simply get higher 3 + make sure Ace is highest

+ Flush             - Highest rank card wins (Get the highest cards to compare?) -> Compare by value, if equal, compare by suit, make sure Ace is highest
                        -> Make a loop to compare, each time get from high to low -> Compare suit of highest card (get highest by using high_card)

+ Straight          - Highest card of the sequence wins -> Get high_card -> If highest equal -> Compare by suit
                    -> Parse to
 Q J 10 9 8
12 24 37 9 8     -> 8 9 37 24 12
Translate into -> Q J 10 9 8

NOTE: Ace can be high or low 
(10,J,Q,K,A) (Ace being highest)       or      low (A,2,3,4,5) (5 being highest)

+ 3 of a kind       - The highest 3 wins -> Simply get higher 3 + make sure Ace is highest

+ 2 Pair

+ 1 Pair




===Possible useful functions===
Compare and return the difference between 2 arrays
*/










/* Function returns the highest card in the hand */
fn high_card(hand:&Vec<u32>) -> u32{
    let mut result = 0;
    for item in hand.iter(){
        if *item%13 == 1{
            if result < *item {result = *item;}
            result = *item
        } else if result < *item && (result%13 !=1){
                result = *item;
        }
    }        

    println!("{:?}",hand);
    return result;
}

fn convert(hand:&mut Vec<u32>){
    for i in hand.iter(){
        //res.push(*i % 13);
        *hand[i] = *hand[i]%13;
    }
    hand.sort();
}


fn get_high(hand:&Vec<u32>) -> u32{
    let mut result = 0;
    //let mut converted = convert(hand).dedup();
    convert(mut hand).dedup();
    let mut val;
    if converted.0 == 1{
        val = converted.0;
    }else{
        val = converted[converted.len-1];
    }
    
    for item in hand.iter(){
        if *item % 13 == val{
            if result < *item {result = *item;}
            result = *item
        } else if result < *item && (result%13 !=1){
                result = *item;
        }
    }        

    println!("{:?}",hand);
    return result;
}

/* Function returns an array of cards that has the same 
 * values to reveal possible four of kind, three of a kind, pairs, ... 
*/
fn check_match(arr:&Vec<u32>) -> Vec<u32>{
    let mut result = Vec::new();
    for item1 in arr.iter(){
        for item2 in arr.iter(){
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

//=======The following functions uses result from check_match as an argument =========*/

/* Function checks if the hand has a pair */
fn has_pair(result: &Vec<u32>) -> bool{
    return result.len() == 2;
}

/* Function checks if the hand has 2 pairs */
fn has_two_pairs(result: &Vec<u32>) -> bool{  //ADD MORE HERE
    return result.len() == 4;
}

/* Function checks if the function has three of a kind */
fn has_three_of_kind(result: &Vec<u32>) -> bool{    
    return result.len() == 3;
}

/* Function checks if the function has four of a kind */
fn has_four_of_kind(result: &Vec<u32>) -> bool{
    let mut check = true;
    for item1 in result.iter(){
        for item2 in result.iter(){
            if *item1 % 13 != *item2 %13{
               check = false;
            }
        }
    }
    return check && has_two_pairs(result);
}

/* Function checks if the function has full house */
fn has_full_house(result: &Vec<u32>) -> bool{
    return result.len() == 5;
}

//=======================================================================================

/* Function checks if the hand has a straight */
fn has_straight(hand: &Vec<u32>) -> bool{
    let mut val_hand = Vec::new(); //New vector to hold the value (number) of the cards
    for i in 0..5{                 //Ignoring the suit
        val_hand.push(hand[i]%13);           
    }
    val_hand.sort();
    
    let mut temp = val_hand[0];

    if val_hand[0]==0 && val_hand[1]== 1 //Special case from 10 to Ace
    {                                    //King, Ace, 10, Jack, Queen
        temp = val_hand[2];
        for i in 3..5{
            if temp+1 == val_hand[i] {
                temp+=1;
            }else{
                return false;
            }
        }
        return true;
    }

    for i in 1..5{ //For any other case
        if temp+1 == val_hand[i] {
            temp+=1;
        }else{
            return false;
        }
    }
    return true;
}

/* Function checks if the hand has a flush */
fn has_flush(hand: &Vec<u32>) -> bool{
    for item1 in hand.iter(){
        for item2 in hand.iter(){
            if get_suit(*item1) != get_suit(*item2) {
                return false;
            }
        }
    }
    return true;
}

/* Helper function to get the suit of a card */
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

/* Function checks if hand has a straight flush */
fn has_straight_flush(hand: &Vec<u32>) -> bool{
    return has_flush(hand)&& has_straight(hand);
}

/* Function checks if hand has a royal flush */
fn has_royal_flush(hand: &Vec<u32>) ->bool{
    return has_straight_flush(hand)&&(hand[0]%13 == 1)&&(hand[1]%13 == 10);
}

/* Function returns the ranking of a hand */
fn get_ranking(hand:&Vec<u32>) -> u32
{   
    let res = check_match(hand); //Array that store the cards in sets from check_match
    // res is input for four/three of a kind, full house, 2 pairs, pair

    if(has_royal_flush(hand)) {             //Royal flush
        return 10;
    } else if (has_straight_flush(hand)) {  //Straight flush
        return 9;
    } else if (has_four_of_kind(&res)) {    //4 of a kind
        return 8;
    } else if (has_full_house(&res)) {      //Full house
        return 7;
    } else if (has_flush(hand)) {           //Flush
        return 6;
    } else if (has_straight(hand)) {        //Straight
        return 5;
    } else if (has_three_of_kind(&res)) {   //3 of a kind
        return 4;
    } else if (has_two_pairs(&res)) {       //2 pairs
        return 3;
    } else if (has_pair(&res)) {            //1 pair
        return 2;
    } else {                                //High card
        return 1;
    }
}

/* ==================== Comparing functions for tie breaking ==================== */

/* Function that compares the set of 2 hands 
 * Used for 3 of kind, 4 of kind, full house */


fn compare_set(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    if hand1.len() == 2 && hand2.len() == 2{ //Fix this
        if high_card(hand1)>high_card(hand2){
            return 1;
        }else{
            return 2;
        }
    } else if hand1.len() == 3 && hand2.len() == 3{ //3 of a kind
        let t1 = get_three(hand1);
        let t2 = get_three(hand2);

        return compare_cards(t1, t2);

    } else if hand1.len() == 4 && hand2.len() == 4{ //2 possibilities: 4 of a kind or 2 pairs
        if (hand1[0]%13==hand1[3]%13) && (hand2[0]%13==hand2[3]%13) { //Check for 4 of a kind //REPLACE THIS MAYbe??????
            if high_card(hand1)>high_card(hand2){
                return 1
            }else{
                return 2
            }
        // } else if (hand1[0]%13==hand1[1]%13 && hand1[2]%13==hand1[3]%13 && hand1[0]%13!=hand1[2]%13) && (hand2[0]%13==hand2[1]%13 && hand2[2]%13==hand2[3]%13 && hand2[0]%13!=hand2[2]%13) { //Check for 2 pairs on each hand
        //     //Add the highest value of each pair from hand 1 to array
        //     let arr1 = Vec::new();
        //     arr1.push(hand[1]);
        //     arr1.push(hand[3]);
        //     arr1.sort(); //The lower value will be stored at 0, high at 1
            

        //     //Add the highest value of each pair from hand 1 to array
        //     let arr1 = Vec::new();
        //     arr1.push(hand[1]);
        //     arr1.push(hand[3]);
        //     arr1.sort(); //The lower value will be stored at 0, high at 1

        //     if high_pair1>high_pair2{
        //         return 1
        //     }else{
        //         return 2
        //     }
        
        
        
        
        }
    }else if hand1.len() == 5 && hand2.len() == 5{ //Full house    ---FIX THIS?
        let t1 = get_three(hand1);
        let t2 = get_three(hand2);

        if t1>t2{
            return 1
        }else{
            return 2
        }
    }
    return 0;
}

/* Function to get three of a kind */
fn get_three(arr:&Vec<u32>) -> u32{
    
    let mut res = 0;

    let mut first = Vec::new();
    let mut second = Vec::new();
    let a = arr[0]%13;
    for i in arr.iter(){
        if *i%13 != a{
            first.push(i);
        }else{
            second.push(i);
        }
    }
    first.sort();
    second.sort();
    if (first.len()==3) { 
        res = *first[2];
    } else {
        res = *second[2];
    }
    //if res == 0 { res = 13; }
    return res;
}

/* Function compares face value of 2 cards
 * Returns 1 if first card is higher, 2 if 2 is higher, 3 if equal*/
fn compare_cards(card1:u32, card2:u32) -> u32
{
    // Get the face value
    let mut val1 = card1%13;
    let mut val2 = card2%13;

    //Convert 1(Ace) to 14, 0(King) to 13
    if val1 == 1 {val1 = 14}
    else if val1 == 0 {val1 = 13}

    //Convert 1(Ace) to 14, 0(King) to 13
    if val2 == 1 {val2 = 14}
    else if val2 == 0 {val2 = 13}

    if val1>val2 {1}
    else if val1<val2 {2}
    else {3}
}

/*
fn winner(hand1:&Vec<u32>,hand2:&Vec<u32>) -> u32{
    let rank1 = get_ranking(hand1); //Get the rankings
    let rank2 = get_ranking(hand2); 

    if (rank1>rank2) {         //Hand 1 has higher ranking
        return 1;
    } else if (rank1<rank2) {  //Hand 2 has higher ranking
        return 2;
    } else { //Tie breaking begins
        //Get sets of pair(s), 3 or of a kinds, and full house from hand
        let set1 = check_match(hand1);
        let set2 = check_match(hand2);

        if (rank1==10 || rank1==6 || rank1==9 || rank1==1){ //Royal flush + Straight flush + flush + high card
            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        } else if (rank1==8 || rank1==7){      //4 of a kind + full house
            return compare_set(&set1,&set2);
        } else if (rank1==5){                        //straight
            return compare_straight(&hand1,&hand2);
        } else if (rank1==4){                        //three of a kind
            return compare_set(&set1,&set2);
        } else if (rank1==3){                        //2 pair
            return compare_set(&set1,&set2);
        } else if (rank1==2) {                       //1 pair
            return 0;
        }
    }
}*/

fn main(){
    let mut hand1 = vec![12,24,37,9,8];
    let mut hand2 = vec![13,5,9,8,11];


    //let res1 = get_three(&hand1);
    println!("{}", get_high(&hand1));

    
    //println!("{}", high_card(hand2));

    //let arr1 = check_match(&hand1);


    // let res2 = get_three(&hand2);
    // println!("{:?}", res2);

    // let arr2 = check_match(&hand2);

    // println!("{:?}", compare_set(&arr1, &arr2));
}


