use dotenv::dotenv;
use std::{
    env,
    process,
    thread,
    time::Duration
};

extern crate dotenv;
extern crate paho_mqtt as mqtt;

pub fn do_conn () -> mqtt::client::Client {
    println!("connected mod conn");

    dotenv().ok();

    // println!("{}", env::var("BROKER_HOST").unwrap().as_str());
    // println!("{}", env::var("CLIENT_NAME").unwrap().as_str());
    // println!("{}", env::var("PUB_TOPIC").unwrap().as_str());
    // println!("{}", env::var("SUB_TOPICS").unwrap().as_str());
    // println!("{}", env::var("USERNAME").unwrap().as_str());
    // println!("{}", env::var("PASSWORD").unwrap().as_str());
    // println!("{}", env::var("QOSS").unwrap().as_str());
    // println!("{}", env::var("QOS").unwrap().as_str().parse::<i32>().unwrap());

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(env::var("BROKER_HOST").unwrap().as_str())
        .client_id(env::var("CLIENT_NAME").unwrap().as_str())
        .finalize();

    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let lwt = mqtt::MessageBuilder::new()
        .topic("testtopic/1")
        .payload("Consumer lost connection")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .user_name(env::var("USERNAME").unwrap().as_str())
        .password(env::var("PASSWORD").unwrap().as_str())
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .will_message(lwt)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    return cli;
}

pub fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Wainting to retry connection..");

    for _ in 0..12 {
        thread::sleep(Duration::from_millis(5000));

        if cli.reconnect().is_ok() {
            println!("Successfully reconnected.");

            return true;
        }
    }

    println!("Unable to reconnect after several attempts.");

    false
}

pub fn subscribe_topics(cli: &mqtt::Client) {
    let topics_env = env::var("SUB_TOPICS").unwrap();
    let qoss_env = env::var("QOSS").unwrap();

    let topics = topics_env.as_str().split(",").collect::<Vec<&str>>();
    let qoss = qoss_env.as_str().split(",").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();
     // &[qos.as_str().parse::<i32>().unwrap()];

    println!("topics: {:?}", &topics);
    println!("qoss: {:?}", &qoss);

    if let Err(e) = cli.subscribe_many(&topics, &qoss) {
        println!("Error subscribes topics: {:?}", e);
        process::exit(1);
    }
}
