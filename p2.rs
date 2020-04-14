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
fn get_card_rank(card: u32) -> u32 
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

//==========The following functions uses result from check_match as an argument ============*/

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

//=============================================================================================
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
    //println!("The hand before calling get_hand_ranking is {:?}", hand);
    //sort_cards(hand);
    //println!("The hand after calling get_hand_ranking is {:?}", hand);
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
    sort_cards(hand1);
    sort_cards(hand2);
    //println!("{:?} {:?}",hand1,hand2);
    let rank1 = get_hand_ranking(hand1); //Get the rankings
    let rank2 = get_hand_ranking(hand2); 

    println!("The rank of hand 1 is:{}", rank1);
    println!("The rank of hand 2 is:{}", rank2);

    //sort_cards(hand1);
    //sort_cards(hand2);

    let mut set1 = check_match(hand1); //Results from check_match
    let mut set2 = check_match(hand2);

     sort_cards(&mut set1);      //Sort the sets
     sort_cards(&mut set2);


    println!("The rank of hand 1 is:{}", get_hand_ranking(hand1));


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

            //println!("The pairs are {}, {}, and the kicker is {}", num1, num2, kicker);
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
        //sort_cards(&mut hand1);
        //hand1.sort();
        let mut rs = convert_to_string(&hand1);
        rs.sort();
        return rs; //list
    }else{
        //sort_cards(&mut hand2);
        //hand2.sort();
        let mut rs = convert_to_string(&hand2);
        rs.sort();
        return rs;
    }
    //winner(hand1,hand2);
    
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
    //let mut hand1 = vec![14,24,25,26,23];

    //let mut hand1 = vec![2,16,18,30,32];
    // let mut hand1 = vec![1,3,17,18,32]; //2,16,30,18,1
    // let mut hand2 = vec![14,23,37,25,26]; //15,3,4,5,14
    
    //let mut hand2 = vec![15,3,17,44,45];
    //sort_cards(&mut hand1);
    //sort_cards(&mut hand2);

    

    // println!("The rank of hand 1 is:{}", get_hand_ranking(&hand1));
    // println!("The rank of hand 2 is:{}", get_hand_ranking(&hand2));

    // println!("The winner is:{}", winner(&mut hand1, &mut hand2));
    
    // println!("is 1 flush? {}",has_flush(&hand1));
    // println!("is 1 straight? {}",has_straight(&hand1));

    // println!("is 2 flush? {}",has_flush(&hand2));
    // println!("is 2 straight? {}",has_straight(&hand2));
    let perm:[u32;10] = [26,52,21,49,8,51,13,50,34,40];
    let mut winner = deal(perm);
    winner.sort();
    println!("{:?}", winner);

    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();

    /* Shuffle between the 2 hands */
    for (i,item) in perm.iter().enumerate(){
        if i%2 == 0{
            hand1.push(*item);
        }else{
            hand2.push(*item);
        }        
    }

    println!("{:?} {:?}",hand1,hand2);



    println!("check_match 1: {:?}", check_match(&hand1));

    println!("check_match 2: {:?}", check_match(&hand2));



    //run the tests

    //test();

    /*
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
    // if get_card_rank(arr[0]) == get_card_rank(arr[1]) && get_card_rank(arr[2]) == get_card_rank(arr[3]){
    //     // xxyyz
    //     kicker = arr[4];
    // } else if get_card_rank(arr[0]) == get_card_rank(arr[1]) && get_card_rank(arr[3]) == get_card_rank(arr[4]){
    //     // xxzyy
    //     kicker = arr[2];
    // } else if get_card_rank(arr[1]) == get_card_rank(arr[2]) && get_card_rank(arr[3]) == get_card_rank(arr[4]){
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
    */
}

fn conv2(arr:[u32;10]) -> Vec<u32>{
    let mut v = Vec::new();
    for i in arr.iter(){
        v.push(*i);
    }
    return v
}
fn test() { 
    let test_cases = [[14, 10, 17, 11, 15, 7, 16, 9, 18, 8],[16, 10, 18, 8, 14, 9, 17, 7, 15, 11],[16, 49, 14, 9, 17, 35, 15, 22, 18, 48],[16, 9, 14, 48, 15, 35, 18, 22, 17, 49],[23, 9, 49, 2, 10, 48, 50, 22, 36, 35],[49, 35, 23, 22, 50, 2, 10, 48, 36, 9],[48, 40, 34, 42, 21, 44, 8, 45, 35, 50],[35, 42, 34, 44, 21, 40, 8, 45, 48, 50],[5, 34, 2, 31, 4, 29, 3, 28, 7, 30],[4, 30, 2, 34, 3, 28, 5, 29, 7, 31],[2, 47, 3, 10, 31, 46, 32, 24, 17, 48],[32, 10, 31, 48, 2, 46, 3, 24, 17, 47],[28, 12, 15, 11, 2, 13, 30, 26, 29, 39],[28, 39, 2, 11, 15, 13, 30, 26, 29, 12],[22, 13, 16, 26, 30, 25, 43, 2, 3, 12],[3, 13, 30, 25, 43, 2, 22, 26, 16, 12],[14, 41, 15, 52, 1, 27, 2, 28, 51, 40],[1, 41, 15, 27, 2, 40, 51, 28, 14, 52],[23, 46, 41, 42, 10, 15, 48, 2, 44, 50],[44, 50, 23, 15, 41, 46, 48, 2, 10, 42],[7, 51, 6, 47, 8, 49, 13, 28, 41, 29],[41, 29, 13, 51, 6, 28, 8, 49, 7, 47],[41, 29, 12, 49, 6, 47, 8, 28, 7, 52],[12, 47, 7, 49, 6, 29, 8, 28, 41, 52],[47, 40, 20, 52, 7, 51, 46, 49, 33, 50],[7, 50, 46, 52, 47, 51, 33, 40, 20, 49],[17, 47, 18, 22, 16, 35, 15, 9, 14, 49],[17, 9, 15, 49, 14, 22, 18, 47, 16, 35],[14, 9, 17, 32, 16, 22, 18, 47, 15, 49],[14, 47, 17, 9, 15, 22, 18, 32, 16, 49],[17, 26, 16, 52, 18, 51, 14, 39, 15, 38],[15, 52, 16, 38, 14, 26, 18, 39, 17, 51],[10, 42, 23, 45, 49, 40, 50, 47, 36, 44],[23, 44, 49, 40, 36, 45, 50, 42, 10, 47],[34, 16, 48, 50, 21, 29, 8, 42, 35, 45],[8, 16, 48, 42, 34, 50, 21, 29, 35, 45],[50, 41, 23, 42, 10, 40, 36, 31, 49, 30],[50, 41, 36, 42, 10, 30, 23, 40, 49, 31],[31, 25, 32, 26, 2, 13, 30, 4, 3, 12],[2, 26, 31, 25, 30, 4, 3, 13, 32, 12],[30, 26, 31, 13, 32, 24, 2, 12, 3, 4],[30, 24, 2, 12, 32, 4, 3, 13, 31, 26],[2, 24, 3, 22, 31, 13, 30, 4, 32, 12],[31, 4, 30, 13, 2, 22, 3, 12, 32, 24],[3, 13, 1, 26, 5, 11, 9, 12, 7, 39],[3, 13, 1, 11, 9, 26, 5, 12, 7, 39],[3, 2, 1, 12, 5, 13, 7, 25, 9, 26],[5, 13, 7, 2, 1, 25, 3, 26, 9, 12],[3, 26, 1, 20, 9, 2, 5, 12, 7, 13],[9, 2, 5, 26, 7, 12, 1, 13, 3, 20],[6, 50, 7, 40, 41, 49, 8, 51, 13, 52],[8, 51, 41, 40, 6, 50, 13, 52, 7, 49],[6, 52, 7, 51, 26, 50, 13, 49, 8, 40],[7, 40, 13, 50, 6, 52, 26, 51, 8, 49],[21, 51, 8, 50, 26, 52, 13, 40, 6, 49],[8, 50, 6, 51, 26, 52, 21, 49, 13, 40],[21, 50, 26, 40, 34, 52, 13, 51, 8, 49],[26, 52, 34, 40, 13, 50, 8, 49, 21, 51],[17, 10, 15, 7, 14, 8, 16, 11, 18, 9],[18, 8, 14, 11, 17, 10, 15, 9, 16, 7],[15, 22, 18, 49, 17, 48, 16, 9, 14, 35],[14, 35, 18, 49, 16, 22, 17, 48, 15, 9],[17, 47, 3, 48, 32, 46, 31, 10, 2, 24],[31, 46, 17, 48, 2, 47, 32, 24, 3, 10],[13, 49, 8, 28, 41, 51, 7, 29, 6, 47],[8, 49, 13, 47, 7, 28, 6, 29, 41, 51],[16, 38, 15, 52, 18, 51, 14, 39, 17, 26],[17, 51, 14, 52, 16, 39, 15, 26, 18, 38],[17, 9, 18, 8, 16, 10, 14, 11, 15, 7],[16, 10, 15, 9, 17, 11, 18, 8, 14, 7],[18, 48, 15, 22, 17, 49, 14, 9, 16, 35],[14, 49, 18, 22, 16, 35, 17, 9, 15, 48],[3, 39, 2, 13, 30, 12, 32, 26, 31, 11],[2, 11, 30, 39, 31, 13, 32, 26, 3, 12],[33, 49, 46, 51, 47, 52, 20, 40, 7, 50],[47, 52, 33, 40, 20, 50, 7, 49, 46, 51],[17, 26, 15, 38, 18, 39, 16, 51, 14, 52],[18, 38, 16, 52, 15, 26, 14, 39, 17, 51],[32, 26, 2, 12, 30, 4, 31, 13, 3, 25],[2, 4, 3, 13, 32, 26, 30, 25, 31, 12],[32, 4, 2, 24, 30, 13, 3, 12, 31, 26],[3, 26, 32, 24, 2, 13, 30, 12, 31, 4],[3, 13, 32, 12, 31, 22, 30, 4, 2, 24],[3, 13, 32, 12, 31, 4, 2, 22, 30, 24],[34, 40, 26, 52, 13, 50, 21, 51, 8, 49],[26, 52, 21, 49, 8, 51, 13, 50, 34, 40]];
    let answer_key = [["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["2H", "3H", "4H", "5H", "8H"],["2H", "3H", "4H", "5H", "8H"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["11C", "12C", "13C", "13D", "13H"],["11C", "12C", "13C", "13D", "13H"],["12C", "12D", "13C", "13D", "2C"],["12C", "12D", "13C", "13D", "2C"],["13S", "1H", "1S", "2H", "2S"],["13S", "1H", "1S", "2H", "2S"],["10C", "10D", "2S", "5S", "9S"],["10C", "10D", "2S", "5S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["10S", "13S", "2H", "3H", "8S"],["10S", "13S", "2H", "3H", "8S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"]];
    let mut failed: Vec<([u32;10], Vec<String>, [&str;5])> = Vec::new();
    for i in 0..test_cases.len() {
        let winner = deal(test_cases[i]);
        if winner != answer_key[i] {
            failed.push((test_cases[i], winner, answer_key[i]));
        }
    }
    println!("Failed: {:?}", failed);
}

// fn main() {
//     //basic example test
//     let perm:[u32;10] = [1,2,3,4,5,6,7,8,9,10];
//     let winner:[String;5] = deal(perm);
//     println!("{:?}", winner);

//     //run the tests
//     test();
// }

// fn test() {   
//     let test_cases = [[14, 10, 17, 11, 15, 7, 16, 9, 18, 8],[16, 10, 18, 8, 14, 9, 17, 7, 15, 11],[16, 49, 14, 9, 17, 35, 15, 22, 18, 48],[16, 9, 14, 48, 15, 35, 18, 22, 17, 49],[23, 9, 49, 2, 10, 48, 50, 22, 36, 35],[49, 35, 23, 22, 50, 2, 10, 48, 36, 9],[48, 40, 34, 42, 21, 44, 8, 45, 35, 50],[35, 42, 34, 44, 21, 40, 8, 45, 48, 50],[5, 34, 2, 31, 4, 29, 3, 28, 7, 30],[4, 30, 2, 34, 3, 28, 5, 29, 7, 31],[2, 47, 3, 10, 31, 46, 32, 24, 17, 48],[32, 10, 31, 48, 2, 46, 3, 24, 17, 47],[28, 12, 15, 11, 2, 13, 30, 26, 29, 39],[28, 39, 2, 11, 15, 13, 30, 26, 29, 12],[22, 13, 16, 26, 30, 25, 43, 2, 3, 12],[3, 13, 30, 25, 43, 2, 22, 26, 16, 12],[14, 41, 15, 52, 1, 27, 2, 28, 51, 40],[1, 41, 15, 27, 2, 40, 51, 28, 14, 52],[23, 46, 41, 42, 10, 15, 48, 2, 44, 50],[44, 50, 23, 15, 41, 46, 48, 2, 10, 42],[7, 51, 6, 47, 8, 49, 13, 28, 41, 29],[41, 29, 13, 51, 6, 28, 8, 49, 7, 47],[41, 29, 12, 49, 6, 47, 8, 28, 7, 52],[12, 47, 7, 49, 6, 29, 8, 28, 41, 52],[47, 40, 20, 52, 7, 51, 46, 49, 33, 50],[7, 50, 46, 52, 47, 51, 33, 40, 20, 49],[17, 47, 18, 22, 16, 35, 15, 9, 14, 49],[17, 9, 15, 49, 14, 22, 18, 47, 16, 35],[14, 9, 17, 32, 16, 22, 18, 47, 15, 49],[14, 47, 17, 9, 15, 22, 18, 32, 16, 49],[17, 26, 16, 52, 18, 51, 14, 39, 15, 38],[15, 52, 16, 38, 14, 26, 18, 39, 17, 51],[10, 42, 23, 45, 49, 40, 50, 47, 36, 44],[23, 44, 49, 40, 36, 45, 50, 42, 10, 47],[34, 16, 48, 50, 21, 29, 8, 42, 35, 45],[8, 16, 48, 42, 34, 50, 21, 29, 35, 45],[50, 41, 23, 42, 10, 40, 36, 31, 49, 30],[50, 41, 36, 42, 10, 30, 23, 40, 49, 31],[31, 25, 32, 26, 2, 13, 30, 4, 3, 12],[2, 26, 31, 25, 30, 4, 3, 13, 32, 12],[30, 26, 31, 13, 32, 24, 2, 12, 3, 4],[30, 24, 2, 12, 32, 4, 3, 13, 31, 26],[2, 24, 3, 22, 31, 13, 30, 4, 32, 12],[31, 4, 30, 13, 2, 22, 3, 12, 32, 24],[3, 13, 1, 26, 5, 11, 9, 12, 7, 39],[3, 13, 1, 11, 9, 26, 5, 12, 7, 39],[3, 2, 1, 12, 5, 13, 7, 25, 9, 26],[5, 13, 7, 2, 1, 25, 3, 26, 9, 12],[3, 26, 1, 20, 9, 2, 5, 12, 7, 13],[9, 2, 5, 26, 7, 12, 1, 13, 3, 20],[6, 50, 7, 40, 41, 49, 8, 51, 13, 52],[8, 51, 41, 40, 6, 50, 13, 52, 7, 49],[6, 52, 7, 51, 26, 50, 13, 49, 8, 40],[7, 40, 13, 50, 6, 52, 26, 51, 8, 49],[21, 51, 8, 50, 26, 52, 13, 40, 6, 49],[8, 50, 6, 51, 26, 52, 21, 49, 13, 40],[21, 50, 26, 40, 34, 52, 13, 51, 8, 49],[26, 52, 34, 40, 13, 50, 8, 49, 21, 51],[17, 10, 15, 7, 14, 8, 16, 11, 18, 9],[18, 8, 14, 11, 17, 10, 15, 9, 16, 7],[15, 22, 18, 49, 17, 48, 16, 9, 14, 35],[14, 35, 18, 49, 16, 22, 17, 48, 15, 9],[17, 47, 3, 48, 32, 46, 31, 10, 2, 24],[31, 46, 17, 48, 2, 47, 32, 24, 3, 10],[13, 49, 8, 28, 41, 51, 7, 29, 6, 47],[8, 49, 13, 47, 7, 28, 6, 29, 41, 51],[16, 38, 15, 52, 18, 51, 14, 39, 17, 26],[17, 51, 14, 52, 16, 39, 15, 26, 18, 38],[17, 9, 18, 8, 16, 10, 14, 11, 15, 7],[16, 10, 15, 9, 17, 11, 18, 8, 14, 7],[18, 48, 15, 22, 17, 49, 14, 9, 16, 35],[14, 49, 18, 22, 16, 35, 17, 9, 15, 48],[3, 39, 2, 13, 30, 12, 32, 26, 31, 11],[2, 11, 30, 39, 31, 13, 32, 26, 3, 12],[33, 49, 46, 51, 47, 52, 20, 40, 7, 50],[47, 52, 33, 40, 20, 50, 7, 49, 46, 51],[17, 26, 15, 38, 18, 39, 16, 51, 14, 52],[18, 38, 16, 52, 15, 26, 14, 39, 17, 51],[32, 26, 2, 12, 30, 4, 31, 13, 3, 25],[2, 4, 3, 13, 32, 26, 30, 25, 31, 12],[32, 4, 2, 24, 30, 13, 3, 12, 31, 26],[3, 26, 32, 24, 2, 13, 30, 12, 31, 4],[3, 13, 32, 12, 31, 22, 30, 4, 2, 24],[3, 13, 32, 12, 31, 4, 2, 22, 30, 24],[34, 40, 26, 52, 13, 50, 21, 51, 8, 49],[26, 52, 21, 49, 8, 51, 13, 50, 34, 40]];
//     let answer_key = [["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["2H", "3H", "4H", "5H", "8H"],["2H", "3H", "4H", "5H", "8H"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["11C", "12C", "13C", "13D", "13H"],["11C", "12C", "13C", "13D", "13H"],["12C", "12D", "13C", "13D", "2C"],["12C", "12D", "13C", "13D", "2C"],["13S", "1H", "1S", "2H", "2S"],["13S", "1H", "1S", "2H", "2S"],["10C", "10D", "2S", "5S", "9S"],["10C", "10D", "2S", "5S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["10S", "13S", "2H", "3H", "8S"],["10S", "13S", "2H", "3H", "8S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"]];
//     let mut failed: Vec<([u32;10], [String;5], [&str;5])> = Vec::new();
//     for i in 0..test_cases.len() {
//         let winner = deal(test_cases[i]);
//         if winner != answer_key[i] {
//             failed.push((test_cases[i], winner, answer_key[i]));
//         }
//     }
//     println!("Failed: {:?}", failed);
// }

