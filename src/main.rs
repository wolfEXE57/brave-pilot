extern crate autopilot;
use std::process::Command;

fn main() {


    // spawns new instance of brave browser
    Command::new("brave")
            .spawn()    //spawns it
            .expect("brave failed to start"); // or tells it that it failed

    autopilot::key::tap(&T,&[],1,1);
    //use autopilot::key::Flag::Alt;
    //autopilot::key::tap(autopilot::key::Flag::Alt,&[],u64,u64);

}