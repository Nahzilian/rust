fn deal(arr:[u32;10]) -> String{
    let mut buf = String::with_capacity("Hand 1 won".len());
    
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

    /* Sort the hand */
    hand1.sort();
    hand2.sort();

    //winner(hand1,hand2);
    buf.push('c');

    return buf;
}

/* This function returns the highest card in the hand */
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

    //println!("{:?}",hand);
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


/* Function returns an array of cards that has the same values to reveal possible four of kind, three of a kind, pairs, ... */
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

fn split_full_house(arr:&Vec<u32>) -> Vec<u32>{
    let mut first = Vec::new();
    let mut second = Vec::new();
    let a = arr[0];
    for i in arr.iter(){
        if *i % 13 != a % 13{
            first.push(i);
        }else{
            second.push(i);
        }
    }
}

fn get_top(arr:&Vec<u32>) -> {
    let mut res = 0;
    for i in arr.iter(){
        if res < *i%13{
            res = *i%13;
        }
    }
    return res;
}

fn has_pair(result: &Vec<u32>) -> bool{
    return result.len() == 2;
}

fn has_three_of_kind(result: &Vec<u32>) -> bool{
    return result.len() == 3;
}

fn has_two_pairs(result: &Vec<u32>) -> bool{
    return result.len() == 4;
}

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

fn has_full_house(result: &Vec<u32>) -> bool{
    return result.len() == 5;
}

// Compare set for 2 pairs

// compare largest
//compare second largest
// compare rank


fn compare_pairs(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    
}
fn compare_set(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    if hand1.len() == 2 && hand2.len() == 2{ //Fix this
        if high_card(hand1)>high_card(hand2){
            return 1
        }else{
            return 2
        }
    } else if hand1.len() == 3 && hand2.len() == 3{ //3 of a kind
        let t1 = get_three(hand1);
        let t2 = get_three(hand2);

        if t1>t2{
            return 1
        }else{
            return 2
        }
<<<<<<< HEAD
    } else if hand1.len() == 4 && hand2.len() == 4{ //2 possibilities: 4 of a kind or 2 pairs
        if (hand1[0]%13==hand1[3]%13) && (hand2[0]%13==hand2[3]%13) { //Check for 4 of a kind
=======
    }else if hand1.len() == 4 && hand2.len() == 4{
        //another if statement
        if (!has_four_of_kind(hand1))&&has_two_pairs(hand1)&&(!has_four_of_kind(hand2))&&has_two_pairs(hand2){
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c
            if high_card(hand1)>high_card(hand2){
                return 1
            }else{
                return 2
            }
<<<<<<< HEAD
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
    }else if hand1.len() == 5 && hand2.len() == 5{ //Full house
        let t1 = get_three(hand1);
        let t2 = get_three(hand2);

        if t1>t2{
=======
        }else if has_four_of_kind(hand1)&&!has_two_pairs(hand1)&&(!has_four_of_kind(hand2))&&has_two_pairs(hand2){
            return compare_pairs(hand1,hand2);
        }
        
    }else if hand1.len() == 5 && hand2.len() == 5{
        if high_card(hand1)>high_card(hand2){
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c
            return 1
        }else{
            return 2
        }
    }
    return 0;
}



// fn compare_pairs(res: &Vec<u32>, hand: &Vec<u32>) -> u32{ 
//     let diff = Vec::new(); //Array to store difference
// }

// fn compare_pairs(res: &Vec<u32>, hand: &Vec<u32>) -> u32{ 
//     let diff = Vec::new(); //Array to store difference
// }




//-----------------------All functions below can be compared with high_card()---------------//
//Straight types
/*
This includes:
straight
straight flush
royal flush
*/


fn has_straight(hand: &Vec<u32>) -> bool{
    let mut val_hand = Vec::new(); //New vect to hold the value (number) of the cards
    for i in 0..5{
        val_hand.push(hand[i]%13);           
    }
    val_hand.sort();
<<<<<<< HEAD
    
=======
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c
    let mut temp = val_hand[0];

    if val_hand[0]==0 && val_hand[1]== 1 //Special case (King,Ace,...)
    {
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


    for i in 1..5{
        if temp+1 == val_hand[i] {
            temp+=1;
        }else{
            return false;
        }
    }
    return true;
}

fn convert_mod(hand: &Vec<u32>)-Vec<u32>{
    let result = Vec::new();
    for i in hand.iter(){
        result.push(*i % 13);
    }
    result.sort();
    return result; 
}

fn filter(hand: &Vec<u32>,temp: &Vec<u32>){
    let mut result = 0;
    for i in hand.iter(){
        if *i % 13 != temp[0] || *i % 13 != temp[1]{
            result = *i;
        } 
    }
    return result;
}

fn compare_pairs_new(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    let mut temp1 = convert_mod(check_match(hand1));
    let mut temp2 = convert_mod(check_match(hand2));    
    // compare highest
    if temp1[3] > temp2[3]{
        return 1;
    }else if temp1[3] < temp2[3]{
        return 2;
    }else{
        if temp1[1] > temp2[1]{
            return 1;
        }else if temp1[1] < temp2[1]{
            return 2;
        }else{
            // fix
            if filter(hand1,temp1.dedup()) % 13 > filter(hand2,temp2.dedup()) % 13{
                return 1;
            }else if filter(hand1,temp1.dedup()) % 13 < filter(hand2,temp2.dedup()) % 13{
                return 2;
            }else{
                if high_card(hand1) > high_card(hand2){
                    return 1;
                }else{
                    return 2;
                }
            }
        }
    }
}
//Flush
/* Check to see if the hand has flush, if yes return the order of the flush(1 for club 2 for diamond ....) */
fn has_flush(hand: &Vec<u32>) -> bool{
    let mut result = true;
    for item1 in hand.iter(){
        for item2 in hand.iter(){
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

/* Function to check if hand has a royal flush */
fn has_royal_flush(hand: &Vec<u32>) ->bool{
    return has_straight_flush(hand)&&(hand[0]%13 == 1)&&(hand[1]%13 == 10);
}

/*
    Checking winner: if 1 => hand 1 win, if 2 => hand 2 win, if tie 3
*/


/* Function to get the ranking */
fn get_ranking(hand:&Vec<u32>) -> u32
{   
    let res = check_match(hand);            //Array that store the cards in sets from check_match

<<<<<<< HEAD
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
=======
    if has_royal_flush(hand) {
        return 10;
    } else if has_straight_flush(hand) {
        return 9;
    } else if has_four_of_kind(&res) {
        return 8;
    } else if has_full_house(&res) {
        return 7;
    } else if has_flush(hand) {
        return 6;
    } else if has_straight(hand) {
        return 5;
    } else if has_three_of_kind(&res) {
        return 4;
    } else if has_two_pairs(&res) {
        return 3;
    } else if has_pair(&res) {
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c
        return 2;
    } else {                                //High card
        return 1;
    }
}

/*
fn winner(hand1:&Vec<u32>,hand2:&Vec<u32>) -> u32{
    let rank1 = get_ranking(hand1); //Get the rankings
    let rank2 = get_ranking(hand2); 

    if (rank1>rank2) {         //Hand 1 has higher ranking
        return 1;
    } else if (rank1<rank2) {  //Hand 2 has higher ranking
        return 2;
    } else {                   //Tie breaking begins
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
            
        }
        }



<<<<<<< HEAD
    if has_royal_flush(hand1)&&has_royal_flush(hand2){
        return 3;
    }else if has_straight_flush(hand1)&&has_straight_flush(hand2){
        if high_card(hand1) > high_card(hand2){
            return 1;
        }else{
            return 2;
        }
    }else if has_four_of_kind(&a) && has_four_of_kind(&b){
        return compare_set(&a,&b);
    }else if has_full_house(&a) && has_full_house(&b){
        return compare_set(&a,&b);
    }else if has_flush(hand1) && has_flush(hand2){
        // check ranking of the card
        if high_card(hand1) > high_card(hand2){
            return 1;
        }else{
            return 2;
        }
    }else if has_straight(hand1) && has_straight(hand2){
=======
fn winner(hand1:&Vec<u32>,hand2:&Vec<u32>) -> u32{
    let mut a = check_match(hand1);
    let mut b = check_match(hand2);
    if get_ranking(&hand1) > get_ranking(&hand2){
        return 1;
    }else if get_ranking(&hand1) == get_ranking(&hand2){
        if has_royal_flush(hand1)&&has_royal_flush(hand2){
            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        }else if has_straight_flush(hand1)&&has_straight_flush(hand2){
            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        }else if has_four_of_kind(&a) && has_four_of_kind(&b){
            return compare_set(&a,&b);
        }else if has_full_house(&a) && has_full_house(&b){
            return compare_set(&a,&b);
        }else if has_flush(hand1) && has_flush(hand2){
            if get_ranking(&hand1) > get_ranking(&hand2){
                return 1;
            }else{
                return 2;
            }
            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        }else if has_straight(hand1) && has_straight(hand2){
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c

            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        }else if has_three_of_kind(&a) && has_three_of_kind(&b){
            return compare_set(&a,&b);
        }else if has_two_pairs(&a) && has_two_pairs(&b){
            return compare_set(&a,&b);
        }else if has_pair(&a) && has_pair(&b){
            return compare_set(&a,&b);
        }else{
            if high_card(hand1) > high_card(hand2){
                return 1;
            }else{
                return 2;
            }
        }
        
    }else{
        return 2;
    }
}*/


fn compare_straight(hand1: &Vec<u32>,hand2: &Vec<u32>) -> u32{
    let mut val_hand1 = Vec::new(); //New vect to hold the value (number) of the cards
    for i in 0..5{
        val_hand1.push(hand1[i]%13);           
    }
    val_hand1.sort();

    let mut val_hand2 = Vec::new(); //New vect to hold the value (number) of the cards
    for i in 0..5{
        val_hand2.push(hand2[i]%13);           
    }
    val_hand2.sort();

    let mut high_card1 = hand1[0]; //Variables to store the high cards
    let mut high_card2 = hand2[0];

    if (val_hand1[0]==1 && val_hand1[4]==5){ //Specific case of low Ace (Ace to 5)
        for i in 0..5{
            if hand1[i]%13==5 {high_card1 = hand1[i]}
        }
    } else {high_card1 = high_card(hand1)}  //For other cases, get the high card
    
    
    if (val_hand2[0]==1 && val_hand2[4]==5){ //Specific case of low Ace (Ace to 5)
        for i in 0..5{
            if hand2[i]%13==5 {high_card2 = hand2[i]}
        }
    } else {high_card2 = high_card(hand2)}  //For other cases, get the high card

    if high_card1>high_card2{
        return 1
    }else{
        return 2
    }
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


fn convert(hand:&Vec<u32>){
    let mut result = "a"
    return result;
}


fn main(){

    // let  a = deal([42,2,3,4,5,6,7,8,9,9]);
    // println!("{}",a);
    // println!("{:?}",high_card([1,14,27,40,52]));
    // println!("{:?}",hasOrder([2,3,15,16,28]));
<<<<<<< HEAD
    //let mut hand1 = Vec::new();
    //let mut hand2 = Vec::new();
=======
    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c

    /*
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
    */
<<<<<<< HEAD

    // hand1.push(1);
    // hand1.push(2);
    // hand1.push(13);
    // hand1.push(12);
    // hand1.push(11);

    let mut hand1 = vec![1,27,40,16,17];
    let mut hand2 = vec![13,39,52,45,31];


    let res = get_three(&hand1);
    println!("{:?}", res);

    let res2 = get_three(&hand2);
    println!("{:?}", res2);

    //println!("Ranking: {}", get_ranking(&hand1));
    //println!("{}", compare_straight(&hand1,&hand2));



    //println!("{:?}",high_card(&hand1));

=======
    //Royal fl
    hand1.push(1);
    hand1.push(10);
    hand1.push(11);
    hand1.push(12);
    hand1.push(13);
    // Straight fl
    hand2.push(14);
    hand2.push(15);
    hand2.push(16);
    hand2.push(17);
    hand2.push(18);
    //let res = check_match(&hand1);
    //println!("{:?}", res);

    //println!("{}", has_royal_flush(&hand1));

    //println!("{:?}",high_card(&hand1));

    println!("{}",winner(&hand1,&hand2));
    println!("Hand2 checking");
    println!("{}",get_ranking(&hand2));// hand 1 is the winner
    //println!("{}",has_royal_flush(&hand2));
>>>>>>> b1559d7ff473a824b762728c0aca59954fcc4a1c

}