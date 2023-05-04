use std::process;
use std::thread::sleep;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::filter::LevelFilter;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

use tracing::{info, info_span};
use url::Url;


#[tokio::main]
async fn main() -> Result<(), tracing_loki::Error> {
    let (layer, task) = tracing_loki::builder()
        .label("host", "10.0.0.2")?
        .label("application", "rust-kafka-producerv2")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://10.0.0.3:3100").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        .with(LevelFilter::INFO)//Transmita solo los eventos de tipo info
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    // The background task needs to be spawned so the logs actually get
    // delivered.
    tokio::spawn(task);

    let broker = "haproxy:9095";
    let topic = "compras-ejecutadas";

    let data = "hello, kafka 12345";

    if let Err(e) = produce_message(data, topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {:?}", e);
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())

}

fn produce_message<'a, 'b>(
    data: &str,
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {

    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))// ~ give the brokers one second time to ack the message
        .with_required_acks(RequiredAcks::One)// ~ require only one broker to ack the message
        .create()?;

    // panic!("No doy mas");
    loop{
        for i in 0..100_000{

            let message= format!("Mensaje id -{} enviado", i).to_string();
            tracing::info!(
                task = "rust-kafka-producer",
                result = "success",
                message,
            );

            producer.send(&Record {
                topic,
                partition: -1, //causes the producer to find out one on its own using its underlying partitioner.
                key: format!("{}", i),
                value: format!("{} {}", data, i).as_bytes(), //we're sending 'data' as a 'value'. there will be no key
            }).unwrap();

            println!("enviado {}", i);

            // return Ok(());
        }
    }
}

/*
use std::thread;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use std::time::Duration;
use rdkafka::{ClientContext, Message};

fn main() {
    println!("INGRESO -!");

    //Creamos un productor
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "haproxy:9095")
        .set("request.required.acks", "all")
        .set("message.timeout.ms", "5000") // Configura el timeout del mensaje aquí (en milisegundos)
        .set("queue.buffering.max.ms", "0") // Do not buffer
        // .set("sasl.mechanism", "PLAIN")
        // .set("security.protocol", "SASL_PLAINTEXT")
        // .set("sasl.username", "admin")
        // .set("sasl.password", "password")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Producer creation error");

    for i0 in 0..50 {
        //Dejamos los mensajes en cola
        for i in 0..100 {
            let payload= format!("::this is the payload {} <<<<", i.clone());
            let key= format!("{}", i.clone());
            let result= producer
                .send(
                BaseRecord::to("topic-test")
                    .payload(payload.as_str())
                    .key(key.as_str()),
                );
            println!("{} {}", i0, i);

            match result {
                Ok(_) => {
                    println!("Message sent successfully");
                }
                Err((err, _message)) => println!("Error sending message: {:?}", err),
            }
        }
        producer.flush(Duration::from_millis(1000));

        //Enviamos los mensajes al broker
        //thread::sleep(Duration::from_millis(3000));
    }

    println!("TERMINO!");
}
*/