use std::env;

pub mod conn;

extern crate paho_mqtt as mqtt;

fn main () {
    let cli: mqtt::client::Client = conn::do_conn();

    let rx = cli.start_consuming();

    conn::subscribe_topics(&cli);

    println!("Processing requests...");

    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg);
        } else if !cli.is_connected() {
            if conn::try_reconnect(&cli) {
                println!("Resubscribe topics");
                conn::subscribe_topics(&cli);
            } else {
                break;
            }
        }
    }

    let topics_env = env::var("SUB_TOPICS").unwrap();
    // let topics: &[&str] = &[topic.as_str()];
    let topics = &topics_env.as_str().split(",").collect::<Vec<&str>>();

    println!("{:?}", &topics);

    if cli.is_connected() {
        println!("Disconnecting");
        cli.unsubscribe_many(&topics).unwrap();
        cli.disconnect(None).unwrap();
    }
    println!("Exiting");
}
