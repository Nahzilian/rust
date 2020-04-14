/*
    CPS506 - Rust Assignment - Poker
    Student name: Le Thanh Dat Phan - 500916500
    Student name: Nguyen Hong Duc - 500910998
*/

/* 
   Sort function 
   It will sort by ranks, and sort by suits if they are the same
*/
fn sort_cards(arr:&mut Vec<u32>){
    let mut swapped = true; 
    while swapped {
        swapped = false; //If there are no changes, array is sorted.
        for i in 1..arr.len() {
            let a = get_card_rank(arr[i-1]);
            let b = get_card_rank(arr[i]);
            
            if a > b { //a has higher rank than b
                arr.swap(i-1, i);
                swapped = true;
            }
            if a == b { //Sort by suit once cards have the same rank
                if arr[i-1] > arr[i]{
                    arr.swap(i-1, i);
                    swapped = true;
                }
            }
        }
    }
}

/* Function to get the rank of a card */
fn get_card_rank(card: u32) -> u32{
    let mut res = card%13;
    if res == 1 {res = 14} //Ace
    else if res == 0 {res = 13} //King
    res
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
    sort_cards(&mut result); //Sort result
    result.dedup(); //Remove duplicates
    return result;
}


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

/* Function checks if the hand has a straight */
fn has_straight(hand: &Vec<u32>) -> bool{    
    let mut temp = hand[0];

    if hand[4]%13==1 && hand[0]%13 == 2 { //Special low Ace case
        for i in 1..4{
            if (temp+1)%13 == hand[i]%13 {
                temp+=1;
            }else{
                return false;
            }
        }
        return true;
    }

    for i in 1..5{
        if (temp+1)%13 == hand[i]%13 {
            temp+=1;
        }else{
            return false;
        }
    }
    return true;
}

/* Function checks if the hand has a flush or not */
fn has_flush(hand: &Vec<u32>) -> bool{
    let mut temp = get_suit(hand[0]);

    for i in 1..5{
        if temp == get_suit(hand[i]){
            temp = get_suit(hand[i]);
        } else {
            return false;
        }
    }
    return true;
}

/* Helper function to get the suit of a card */
fn get_suit(card: u32) -> u32{
    if card <= 13 {        //Clubs  
        return 1;
    } else if card <= 26 { //Diamonds
        return 2;          
    } else if card <= 39 { //Clubs
        return 3;
    } else {
        return 4;         //Spades
    }
}

/* Function checks if hand has a straight flush */
fn has_straight_flush(hand: &Vec<u32>) -> bool{
    return has_flush(hand)&& has_straight(hand);
}

/* Function checks if hand has a royal flush */
fn has_royal_flush(hand: &Vec<u32>) ->bool{
    return has_straight_flush(hand)&&(hand[4]%13 == 1)&&(hand[0]%13 == 10);
}

/* Function returns the ranking of a hand */
fn get_hand_ranking(hand:&Vec<u32>) -> u32{   
    let res = check_match(hand); //Array that store the cards in sets from check_match
    // res is input for four/three of a kind, full house, 2 pairs, pair

    if has_royal_flush(hand) {             //Royal flush
        return 10;
    } else if has_straight_flush(hand) {  //Straight flush
        return 9;
    } else if has_four_of_kind(&res) {    //4 of a kind
        return 8;
    } else if has_full_house(&res) {      //Full house
        return 7;
    } else if has_flush(hand) {           //Flush
        return 6;
    } else if has_straight(hand) {        //Straight
        return 5;
    } else if has_three_of_kind(&res) {   //3 of a kind
        return 4;
    } else if has_two_pairs(&res) {       //2 pairs
        return 3;
    } else if has_pair(&res) {            //1 pair
        return 2;
    } else {                              //High card
        return 1;
    }
}

/* ==================== Comparing functions for tie breaking ==================== */

/* Function to compare sets in tie breaking (assuming they have the same rank)
   Take the ouput of the function check_match as input
   Used for 3 of kind, 4 of kind, full house, 2 pairs, 1 pair 
   */
fn compare_set(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    if hand1.len() == 4 { 
        if has_four_of_kind(hand1) { //======== 4 of a kind ========
            /*  2 Cases xxxxy or yxxxx where x is the rank of 4 of a kind
            and y is the rank of the kicker*/
            if hand1[2]>hand2[2] {return 1} // Compare the third card
            else {return 2}
        } else { //======== 2 pairs ========

            return 0;

<<<<<<< HEAD
    let arr:[u32;10] = [3,2,17,4,44,6,19,8,7,9];
=======

        }
    } else if hand1.len() == 3 { //======== 3 of a kind ========
        /* 3 Cases xxxyz or yxxxz or yzxxx where x is the rank of 3 of a kind
        and y,z are the ranks of remaining cards*/
        if hand1[2]>hand2[2] {return 1} // Compare the third card
        else {return 2}
    } else if hand1.len() == 5 { //======== Full house ========
        /* 2 Cases xxxyy or yyxxx where x is the rank of 3 of a kind
        and y is the rank of the pair */
        if hand1[2]>hand2[2] {return 1} // Compare the third card
        else {return 2}
    } else { //======== 1 pair ========
        return 0;
    }
}


/* Function compares rank of 2 cards
 * Returns 1 if first card is higher, 2 if 2 is higher, 3 if equal*/
fn compare_by_rank(card1:u32, card2:u32) -> u32
{
    // Get the rank
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

/* Function compares suit of 2 cards
 * Returns 1 if first card is higher, 2 if 2 is higher, 3 if equal*/
fn compare_by_suit(card1:u32, card2:u32) -> u32
 {
     // Get the suit 
     let val1 = get_suit(card1);
     let val2 = get_suit(card2);
 
     if val1>val2 {1}
     else if val1<val2 {2}
     else {3}
 }

 /* Get the kicker card of a hand */
fn get_kicker(hand:&Vec<u32>) ->u32 {
    let mut kicker = 0;
    if get_card_rank(hand[0]) == get_card_rank(hand[1]) && get_card_rank(hand[2]) == get_card_rank(hand[3]){
        // xxyyz
        kicker = hand[4];
    } else if get_card_rank(hand[0]) == get_card_rank(hand[1]) && get_card_rank(hand[3]) == get_card_rank(hand[4]){
        // xxzyy
        kicker = hand[2];
    } else if get_card_rank(hand[1]) == get_card_rank(hand[2]) && get_card_rank(hand[3]) == get_card_rank(hand[4]){
        // zxxyy
        kicker = hand[0];
    }
    return kicker;
}

/* This function finds the winning hand
 * Return 1 if hand 1 wins, and 2 otherwise */
fn winner(hand1:&mut Vec<u32>,hand2:&mut Vec<u32>) -> u32{
    sort_cards(hand1); //Sort the hands 
    sort_cards(hand2);

    let rank1 = get_hand_ranking(hand1); //Get the rankings
    let rank2 = get_hand_ranking(hand2); 

    let mut set1 = check_match(hand1); //Results from check_match
    let mut set2 = check_match(hand2);

    sort_cards(&mut set1);      //Sort the sets
    sort_cards(&mut set2);

    if rank1>rank2 {         //Hand 1 has higher ranking
        return 1;
    } else if rank1<rank2 {  //Hand 2 has higher ranking
        return 2;
    } else { //Tie breaking begins when they have same ranking
        if rank1 == 10 { //Royal Flush - Compare the highest cards
            //If both are royal flushes, the highest cards are Aces so we would compare and get the highest suit
            return compare_by_suit(hand1[4],hand2[4]);
        } else if rank1 == 9 || rank1 == 5 { //Straight flush or straight - Compare highest cards
            println!("{:?} {:?}",hand1,hand2);


            //Variables to store results
            //Defaults to last card in sorted hand
            let mut high1 = hand1[4]; 
            let mut high2 = hand2[4];
            
            //Check for low Ace case
            if hand1[3]%13 == 5 && hand1[4]%13 == 1 {
                high1 = hand1[3];
            }

            if hand2[3]%13 == 5 && hand2[4]%13 == 1 {
                high2 = hand2[3];
            }

            let mut res = compare_by_rank(high1, high2);
            if res == 3 { //If same rank, compare by suit.
                res = compare_by_suit(high1,high2);
            }
            return res;

        } else if rank1 == 8 { //4 of a kind - Compare and find the higher four
            return compare_set(&set1, &set2);
        } else if rank1 == 7 { //Full house - Compare and find the higher 3 of a kind in the hand
            return compare_set(&set1, &set2);
        } else if rank1 == 6 { //Flush - Compare highest, then second highest, ...
            let mut res;
            for i in (0..5).rev(){ //Compare by rank
                println!("LOOP: {}", i);
                res = compare_by_rank(hand1[i],hand2[i]);
                if res != 3 {return res} //If they are not equal, return winning hand
            }
            
            //If both hands have the same ranks, then compare the suit of highest card
            res = compare_by_suit(hand1[4],hand2[4]);
            return res;
        } else if rank1 == 4 { //3 of a kind - Compare and find the higher 3 of a kind in the hand
            return compare_set(&set1, &set2);
        } else if rank1 == 3 { //2 pairs - Compare rank of higher pair -> compare rank of other pair -> Compare rank of kicker -> Compare suits of higher pair
            /*=============================================== NEED IMPLEMENTATION ============================================*/

            //Kicker cards for each hand
            let kicker1;
            let kicker2;

            //Get the highest cards of each pair at index 1 and 3 since the pairs are sorted.
            //Note that these values are sorted so they are the highest value in the pair which can be used for suit comparison
            let high1 = set1[1]; //Higher pair
            let low1 = set1[3]; //Lower pair

            let high2 = set2[1]; //Higher pair
            let low2 = set2[3]; //Lower pair

            //3 Patterns can be seen when the hand is sorted where x's and y's represents the rank of each pair and z is the kicker card
            // xxyyz     or       xxzyy          or zxxyy
            kicker1 = get_kicker(hand1);
            kicker2 = get_kicker(hand2);

            let mut res = compare_by_rank(high1,high2); //Compare by rank of the highest pair
            if res !=3 {return res} //If not equal return result
            
            res = compare_by_rank(low1,low2); //If rank of higher pair is equal, compare by rank of the lower pair
            if res !=3 {return res} //If not equal return result

            res = compare_by_rank(kicker1,kicker2); //If rank of lower pair is equal, compare rank of kicker cards
            if res !=3 {return res} //If not equal return result

            res = compare_by_suit(high1,high2); //If all are equal, compare by suit of the higher cards
            return res;

        } else if rank1 == 2 { //1 pair - Compare rank of pair -> compare rank of remaining cards from highest to lowest -> Compare suits of pair
            /*=============================================== NEED IMPLEMENTATION ============================================*/

            //Kicker cards stored in an array for each hand
            let mut kickers1 = Vec::new();
            let mut kickers2 = Vec::new();

            //Get the value of each pair 
            let pair1 = set1[1]; //Pair from hand 1
            let pair2 = set2[1]; //Pair from hand 2

            println!("{}     {}", pair1, pair2);

            
            for i in 0..5{
                if hand1[i]%13 != pair1%13 { //Push the remaining 3 cards besides the pair to the array
                    kickers1.push(hand1[i]);
                }
                if hand2[i]%13 != pair2%13 { 
                    kickers2.push(hand2[i]);
                }
            }

            sort_cards(&mut kickers1); //Sort the array of remaining cards
            sort_cards(&mut kickers2);

            println!("{:?}     {:?}", kickers1, kickers2);



            let mut res;

            res = compare_by_rank(pair1,pair2); //Compare by rank the pairs
            if res != 3 {return res}

            for i in (0..3).rev(){
                println!("LOOP: {}", i);
                res = compare_by_rank(kickers1[i],kickers2[i]);
                if res != 3 {return res} //If they are not equal, return winning hand
            }

            res = compare_by_suit(pair1,pair2); //If all remaining cards are equal in rank, compare suit of the pairs
            return res;
        } 
        else { //Compare highest card, then second highest and so on -> If all equal the compare the rank of highest card
            let mut res;
            for i in (0..5).rev(){ //Compare by rank
                println!("LOOP: {}", i);
                res = compare_by_rank(hand1[i],hand2[i]);
                if res != 3 {return res} //If they are not equal, return winning hand
            }
            
            //If both hands have the same ranks, then compare the suit of highest card
            res = compare_by_suit(hand1[4],hand2[4]);
            return res;
        }
    }
}


fn deal(arr:[u32;10]) -> Vec<String>{
    /* Create growable arrays for hand 1 and hand 2*/ 
>>>>>>> 946a351126153f6364450110dd6609582a48860a
    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();

    /* Shuffle between the 2 hands */
    for (i,item) in arr.iter().enumerate(){
        if i%2 == 0{
            hand1.push(*item);
        }else{
            hand2.push(*item);
        }        
    }

    let winning_hand = winner(&mut hand1,&mut hand2);
    if winning_hand == 1 {
        let mut rs = convert_to_string(&hand1);
        rs.sort();
        return rs;
    }else{
        let mut rs = convert_to_string(&hand2);
        rs.sort();
        return rs;
    }
}

fn convert_to_string(hand: &Vec<u32>) -> Vec<String>{
    let club = String::from("C");
    let diamond = String::from("D");
    let heart = String::from("H");
    let spade = String::from("S");
    let mut v = Vec::new();

    for card in hand.iter(){
        let mut val = *card % 13;
        if val == 0{
            val = 13
        }
        if get_suit(*card) == 1{
            v.push(format!("{}{}",val,club));
        }else if get_suit(*card) == 2{
            v.push(format!("{}{}",val,diamond));
        }else if get_suit(*card) == 3{
            v.push(format!("{}{}",val,heart));
        }else if get_suit(*card) == 4{
            v.push(format!("{}{}",val,spade));
        }
    }
    return v;
}

fn main(){
    let perm:[u32;10] = [26,52,21,49,8,51,13,50,34,40];
    let winner = deal(perm);
    println!("{:?}", winner);
}