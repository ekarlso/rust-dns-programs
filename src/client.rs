extern crate dns;
extern crate docopt;
extern crate serialize;

use std::default;

use docopt::Docopt;

use dns::client::Client;

use dns::message::{Message, Header};
use dns::query::Question;
use dns::types::Type;
use dns::utils;

static USAGE: &'static str = "
Usage: rhost <rtype> <name>
";

#[deriving(Decodable, Show)]
struct Args {
    arg_rtype: String,
    arg_name: String
}


fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let header = Header {..default::Default::default() };

    let mut questions: Vec<Question> = Vec::new();

    let q: Question = Question {
        name: utils::name_from_slice(args.arg_name.as_slice()),
        ty: from_str::<Type>(args.arg_rtype.as_slice()).unwrap(),
        ..default::Default::default()
    };

    questions.push(q);

    let mut msg = Message {
        header: header,
        questions: questions,
        ..default::Default::default()
    };

    let client: Client = Client::new();

    println!("About to query {} {} towards servers {}", args.arg_rtype, args.arg_name, client.config.servers)

    let msg = client.exchange(&mut msg).unwrap();
}
