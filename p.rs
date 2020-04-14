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
Make sure that the get_hand_ranking and other functions work normally.

NOTED BUG: IF YOU USE sort_cards, and if your hand is a straight flush, you would get a flush when calling get_hand_ranking




+ Royal Flush       - Use highest card to determine (ACE) (INDEX 0)? -> Simply compare high card

+ Straight Flush    - Use Highest card to determine -> high_card + Make sure Ace is highest

+ 4 of a kind       - Get the higher four of a kind and compare -> Simply get higher 4 + make sure Ace is highest
TRICK: xxxxy or yxxxx where x is the 4 of a kind
so we can just simply get the third number to compare

+ Full house        - The highest 3 wins -> Simply get higher 3 + make sure Ace is highest
TRICK: yyxxx or xxxyy.
so we can just simply get the third number to compare

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
TRICK: xxxyz   or yxxxz    or yzxxx
get third index.

+ 2 Pair
Pattern
zxxyy
xxzyy
xxyyz


+ 1 Pair




===Possible useful functions===
Compare and return the difference between 2 arrays
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
            let a = get_rank(arr[i-1]);
            let b = get_rank(arr[i]);
            
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

/* Function to get the rank of the card */
fn get_rank(card: u32) -> u32 
{
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
    let mut temp = hand[0];

    for i in 1..5{
        if (temp+1)%13 == hand[i]%13 {
            temp+=1;
        }else{
            return false;
        }
    }
    return true;
}

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


/*
fn has_flush(hand: &Vec<u32>) -> bool{
    let mut temp = get_suit(hand[0]);
    let mut res = true;
    for i in 1..5{
        if temp != get_suit(hand[i]){
            res = false;
        }
    }
    return res;
}
*/

/* Helper function to get the suit of a card */
fn get_suit(card: u32) -> u32 {
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
fn get_hand_ranking(hand:&Vec<u32>) -> u32
{   
    //hand.sort();
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

/* Function that compares the set of 2 hands 
 * Used for 3 of kind, 4 of kind, full house */


// fn compare_set(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
//     if hand1.len() == 2 && hand2.len() == 2{ //Fix this
//         // if high_card(hand1)>high_card(hand2){
//         //     return 1;
//         // }else{
//         //     return 2;
//         // }
//         return 0;
//     } else if hand1.len() == 3 && hand2.len() == 3{ //3 of a kind
//         let t1 = get_three(hand1);
//         let t2 = get_three(hand2);

//         return compare_by_rank(t1, t2);

//     } else if hand1.len() == 4 && hand2.len() == 4{ //2 possibilities: 4 of a kind or 2 pairs
//         if (hand1[0]%13==hand1[3]%13) && (hand2[0]%13==hand2[3]%13) { //Check for 4 of a kind //REPLACE THIS MAYbe??????
//             // if high_card(hand1)>high_card(hand2){
//             //     return 1
//             // }else{
//             //     return 2
//             // }
//             return 0
//         // } else if (hand1[0]%13==hand1[1]%13 && hand1[2]%13==hand1[3]%13 && hand1[0]%13!=hand1[2]%13) && (hand2[0]%13==hand2[1]%13 && hand2[2]%13==hand2[3]%13 && hand2[0]%13!=hand2[2]%13) { //Check for 2 pairs on each hand
//         //     //Add the highest value of each pair from hand 1 to array
//         //     let arr1 = Vec::new();
//         //     arr1.push(hand[1]);
//         //     arr1.push(hand[3]);
//         //     arr1.sort(); //The lower value will be stored at 0, high at 1
            

//         //     //Add the highest value of each pair from hand 1 to array
//         //     let arr1 = Vec::new();
//         //     arr1.push(hand[1]);
//         //     arr1.push(hand[3]);
//         //     arr1.sort(); //The lower value will be stored at 0, high at 1

//         //     if high_pair1>high_pair2{
//         //         return 1
//         //     }else{
//         //         return 2
//         //     }
        
        
        
        
//         }
//     }else if hand1.len() == 5 && hand2.len() == 5{ //Full house    ---FIX THIS?
//         let t1 = get_three(hand1);
//         let t2 = get_three(hand2);

//         if t1>t2{
//             return 1
//         }else{
//             return 2
//         }
//     }
//     return 0;
// }

/* Function to compare sets in tie breaking (assuming they have the same rank)
   Take the ouput of the function check_match as input
   Used for 3 of kind, 4 of kind, full house, 2 pairs, 1 pair 
   */
fn compare_set(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    if hand1.len() == 4 { 
        if has_four_of_kind(hand1) { //======== 4 of a kind ========
            /*  2 Cases xxxxy or yxxxx where x is the rank of 4 of a kind
            and y is the rank of the kicker*/
            if hand1[2]>hand2[1] {return 1} // Compare the third card
            else {return 2}
        } else { //======== 2 pairs ========

            return 0;


        }
    } else if hand1.len() == 3 { //======== 3 of a kind ========
        /* 3 Cases xxxyz or yxxxz or yzxxx where x is the rank of 3 of a kind
        and y,z are the ranks of remaining cards*/
        if hand1[2]>hand2[1] {return 1} // Compare the third card
        else {return 2}
    } else if hand1.len() == 5 { //======== Full house ========
        /* 2 Cases xxxyy or yyxxx where x is the rank of 3 of a kind
        and y is the rank of the pair */
        if hand1[2]>hand2[1] {return 1} // Compare the third card
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

 fn get_kicker(&hand:Vec<u32>)
/*
fn winner(hand1:&Vec<u32>,hand2:&Vec<u32>) -> u32{
    let rank1 = get_hand_ranking(hand1); //Get the rankings
    let rank2 = get_hand_ranking(hand2); 

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



fn winner(hand1:&mut Vec<u32>,hand2:&mut Vec<u32>) -> u32{
    let rank1 = get_hand_ranking(hand1); //Get the rankings
    let rank2 = get_hand_ranking(hand2); 

    sort_cards(hand1);
    sort_cards(hand2);

    let mut set1 = check_match(hand1); //Results from check_match
    let mut set2 = check_match(hand2);

    println!("The rank of hand 1 is:{}", get_hand_ranking(hand1));


    if rank1>rank2 {         //Hand 1 has higher ranking
        return 1;
    } else if rank1<rank2 {  //Hand 2 has higher ranking
        return 2;
    } else { //Tie breaking begins when they have same ranking
        if rank1 == 10 { //Royal Flush - Compare the highest cards
            //If both are royal flushes, the highest cards are Aces so we would compare and get the highest suit
            return compare_by_suit(hand1[4],hand2[4]);
        } else if rank1 == 9 || rank1 == 5 { //Straight flush or flush - Compare highest cards
            
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
            return compare_set(hand1, hand2);
        } else if rank1 == 7 { //Full house - Compare and find the higher 3 of a kind in the hand
            return compare_set(hand1, hand2);
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
            return compare_set(hand1, hand2);
        } else if rank1 == 3 { //2 pairs - Compare rank of higher pair -> compare rank of other pair -> Compare rank of kicker -> Compare suits of higher pair
            /*=============================================== NEED IMPLEMENTATION ============================================*/
            
            //Sort the set
            sort_cards(&mut set1);
            sort_cards(&mut set2);

            //Kicker cards for each hand
            let mut kicker1 = 0;
            let mut kicker2 = 0;

            //Get the highest cards of each pair at index 1 and 3 since the pairs are sorted.
            //Note that these values are sorted so they are the highest value in the pair which can be used for suit comparison
            let high1 = set1[1]; //Higher pair
            let low1 = set1[3]; //Lower pair

            let high2 = set2[1]; //Higher pair
            let low2 = set2[3]; //Lower pair

            //3 Patterns can be seen when the hand is sorted where x's and y's represents the rank of each pair and z is the kicker card
            if get_rank(arr[0]) == get_rank(arr[1]) && get_rank(arr[2]) == get_rank(arr[3]){
                // xxyyz
                kicker = arr[4];
            } else if get_rank(arr[0]) == get_rank(arr[1]) && get_rank(arr[3]) == get_rank(arr[4]){
                // xxzyy
                kicker = arr[2];
            } else if get_rank(arr[1]) == get_rank(arr[2]) && get_rank(arr[3]) == get_rank(arr[4]){
                // zxxyy
                kicker = arr[0];
            }

            println!("The pairs are {}, {}, and the kicker is {}", num1, num2, kicker);
            return 0;


        } else if rank1 == 2 { //1 pair - Compare rank of pair -> compare rank of remaining cards from highest to lowest -> Compare suits of pair
            /*=============================================== NEED IMPLEMENTATION ============================================*/
            return 0;


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

fn main(){
    //let mut hand1 = vec![14,24,25,26,23];


    // let mut hand1 = vec![1,2,16,30,18];
    // let mut hand2 = vec![14,15,3,17,44];


    // println!("The rank of hand 1 is:{}", get_hand_ranking(&hand1));
    // println!("The rank of hand 2 is:{}", get_hand_ranking(&hand2));

    // println!("The winner is:{}", winner(&mut hand1, &mut hand2));


    let mut arr = vec![29,16,35,2,1];
    sort_cards(&mut arr);
    println!("{:?}", arr);

    let mut res = check_match(&arr);
    sort_cards(&mut res);

    // let num1 = res[1];
    // let num2 = res[3];


    /* 
    zxxyy
    xxzyy
    xxyyz
    */

    //2 Pairs implementation 

    // let mut kicker = 0;
    // if get_rank(arr[0]) == get_rank(arr[1]) && get_rank(arr[2]) == get_rank(arr[3]){
    //     // xxyyz
    //     kicker = arr[4];
    // } else if get_rank(arr[0]) == get_rank(arr[1]) && get_rank(arr[3]) == get_rank(arr[4]){
    //     // xxzyy
    //     kicker = arr[2];
    // } else if get_rank(arr[1]) == get_rank(arr[2]) && get_rank(arr[3]) == get_rank(arr[4]){
    //     // zxxyy
    //     kicker = arr[0];
    // }


    // println!("The pairs are {}, {}, and the kicker is {}", num1, num2, kicker);


    //Array to store the remaining cards 
    let mut kickers = Vec::new();
    
    let num3 = res[1];

    for i in 0..5{
        if arr[i]%13 != num3%13 {
            kickers.push(arr[i]);
        }
    }
    
    
    
    sort_cards(&mut kickers);
    println!("The pair is {} and the remaining cards from high to low is {}, {}, {}", num3, kickers[2], kickers[1], kickers[0]);

}


