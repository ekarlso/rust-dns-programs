extern crate dns;

use std::default;
use std::str::from_str;

use std::io::BufWriter;
use std::io::net::ip::{IpAddr,Port,SocketAddr};
use std::io::net::udp::UdpSocket;

use dns::client::Client;
use dns::client::config::ClientConfig;

use dns::io::DNSWriter;
use dns::message::{Message, Header};
use dns::query::Question;
use dns::types::Type;
use dns::utils;

fn main() {
    //sock.send_to([7u8, 7u8].as_slice(), dst);

    let header = Header {..default::Default::default() };

    let mut questions: Vec<Question> = Vec::new();

    let q: Question = Question {
        name: utils::name_from_slice("google.com"),
        ty: Type::A,
        ..default::Default::default()
    };

    questions.push(q);

    let mut msg = Message {
        header: header,
        questions: questions,
        ..default::Default::default()
    };


    let client: Client = Client::new();
    client.exchange(&mut msg);

    println!("Msg {}", msg);

}