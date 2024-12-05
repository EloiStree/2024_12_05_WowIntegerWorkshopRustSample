// Input : https://github.com/EloiStree/2024_08_29_ScratchToWarcraft

use std::thread;
use std::time::Duration;
use rand::Rng;

use wowint::utility::{
    WowIntegerTarget,
    IntegerUdpSender
};


fn main(){

    // Create a new WowIntegerPlayer instance with a dummy IP, port, and index
    let player: WowIntegerTarget = 
    WowIntegerTarget::new( 
        "192.168.1.37", 
    7000,
     1);

    loop {
        let random_number = rand::thread_rng().gen_range(1100..1104);
        let _ = player.send_integer_to_target(random_number);
        print!("Sent: {}\n", random_number);
        thread::sleep(Duration::from_secs(1));
        print!("Sent: {}\n", random_number+1000);
        let _ = player.send_integer_to_target(random_number+1000);
        thread::sleep(Duration::from_secs(1));
        
    }
}
