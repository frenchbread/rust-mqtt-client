use std::env;

pub mod conn;

extern crate paho_mqtt as mqtt;

fn main () {
    let cli: mqtt::client::Client = conn::do_conn();

    let qoss_env = env::var("QOSS").unwrap();
    let qoss = qoss_env.as_str().split(",").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for num in 0..5 {
        let content = "Hello world! ".to_string() + &num.to_string();
        let msg = mqtt::Message::new(env::var("PUB_TOPIC").unwrap().as_str(), content.clone(), qoss[0]);

        println!("Publishing messages on the {:?} topic: {:?}", env::var("PUB_TOPIC").unwrap().as_str(), content.clone());

        let tok = cli.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);

            break;
        }
    }

    let tok = cli.disconnect(None);

    println!("Disconnect from the broker");

    tok.unwrap();
}
