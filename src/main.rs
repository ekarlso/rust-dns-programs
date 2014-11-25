#![feature(slicing_syntax)]

extern crate dns;

use std::default;
use std::io;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};

use dns::message::{Header,Message,Resource};
use dns::types::Type;
use dns::query::QueryResponse;

fn read(buffer: &mut [u8, ..512], length: uint) -> io::IoResult<Message> {
  let mut reader = std::io::BufReader::new(buffer.slice_to(length));

  let msg = Message::from_reader(&mut reader);
  println!("Received query: {}", msg);

  msg
}


fn generate_response(query: &Message) -> io::IoResult<Message> {
    let header = Header { id: query.header.id, qr: QueryResponse::RESPONSE, rd: query.header.rd, ..default::Default::default() };

    let mut answers = Vec::new();

    for q in query.questions.iter() {
        let name = q.name.clone();
        let data: Vec<u8> = "10.0.0.1".as_bytes().to_vec();

        let r = Resource{
            name: name,
            ty: q.ty,
            class: q.class,
            ttl: 600,
            rdata: data
        };

        answers.push(r)
    }

    let msg = Message {
        header: header,
        questions: query.questions.clone(),
        answers: answers,
        ..default::Default::default()
    };

    return Ok(msg);
}

fn write(buffer: &mut [u8, ..512], response: &mut Message) -> io::IoResult<u64> {
    let mut writer = std::io::BufWriter::new(buffer.as_mut_slice());

    match response.write_to(&mut writer) {
        Err(e) => return Err(e), Ok(_) => ()
    };

    return writer.tell();
}

fn process(socket: &mut UdpSocket, source: &SocketAddr, buffer: &mut [u8, ..512], length: uint) -> io::IoResult<()> {
    let mut response = match read(buffer, length) {
        Ok(m) => match generate_response(&m) {
            Ok(r) => r,
            Err(e) => return Err(e)
        },
        Err(e) => return Err(e)
    };


    let mut b = [0, ..512];
    let length = write(&mut b, &mut response);

    return match length {
        Ok(length) => socket.send_to(b.slice_to(length as uint), *source),
        Err(e) => Err(e)
    };
}

fn main() {
    let addr = SocketAddr {
        ip: Ipv4Addr(127, 0, 0, 1), port: 5053
    };

    let mut socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    let mut buffer = [0, ..512];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((length, src)) => println!("processed: {}", process(&mut socket, &src, &mut buffer, length)),
            Err(e) => println!("couldn't receive a datagram: {}", e)
        }
    }
}