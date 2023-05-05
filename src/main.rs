use std::fs;
use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};


use native_tls::{Certificate, TlsConnector};
use postgres::{Client, NoTls};
use postgres_native_tls::MakeTlsConnector;

fn main() {

    setup_log();

    let broker = "kafka-haproxy:9095";
    let topic = "compras-ejecutadas";

    let data = "hello, kafka 12345";

    if let Err(e) = produce_message(data, topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {:?}", e);
    }

}

fn produce_message<'a, 'b>(
    data: &str,
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {

    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let cert = fs::read("db-certificate-cockroach.crt").unwrap();//es el mismo ca.crt
    let cert = Certificate::from_pem(&cert).unwrap();

    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();

    let connector = MakeTlsConnector::new(connector);
    let mut client= Client::connect(
        "postgresql://savne:password@cockroach-haproxy:26257/defaultdb", connector
    ).unwrap();

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))// ~ give the brokers one second time to ack the message
        .with_required_acks(RequiredAcks::One)// ~ require only one broker to ack the message
        .create()?;


    loop{

        for i in 0..100_000{

            let value= format!("{} {}", data, i);
            producer.send(&Record {
                topic,
                partition: -1, //causes the producer to find out one on its own using its underlying partitioner.
                key: format!("{}", i),
                value: value.as_bytes(), //we're sending 'data' as a 'value'. there will be no key
            }).unwrap();

            info!("enviado {}", value);

            let query_result= client.query_one("
                INSERT INTO events (type, data)
                VALUES ('producer', $1)
                returning id", &[ &value],
            ).unwrap();

        }
    }
}

fn setup_log() {

    let level = log::LevelFilter::Info;
    let file_path = "./log/producerv2.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        // Especifica como quiero que se guarde en el archivo
        .encoder(Box::new(PatternEncoder::new("{d} {l} {f} {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);

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