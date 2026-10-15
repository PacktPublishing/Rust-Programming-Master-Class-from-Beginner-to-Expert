
mod travel_history {
    enum Conveyance {  
        Car, 
        Train,
        Air, 
    }
}


pub fn allowance() {
    let travel_1 = travel_history::Conveyance::Car; 
    let travel_2 = travel_history::Conveyance::Train;
}
