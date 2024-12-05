// Input : https://github.com/EloiStree/2024_08_29_ScratchToWarcraft

use std::thread;
use std::time::Duration;
use rand::Rng;

use wowint::udp::WowIntegerTarget;




fn main(){

    // Create a new WowIntegerPlayer instance with a dummy IP, port, and index
    let mut player = WowIntegerTarget::new("192.168.1.114", 7000, 0);

    loop {
        let random_number = rand::thread_rng().gen_range(0..1000);
        player.send_integer(random_number);
        thread::sleep(Duration::from_secs(1));
        player.send_integer(random_number+1000);
        thread::sleep(Duration::from_secs(1));
        
    }
}
