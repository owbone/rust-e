// The MIT License (MIT)
//
// Copyright (c) 2015 Oliver Bone
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

extern crate getopts;
extern crate regex;

mod irc;

use getopts::Options;
use std::env;
use std::iter::FromIterator;
use  std::io::BufReader;
use std::io::Error as IoError;
use std::str::FromStr;
use std::net::TcpStream;

type IrcStream = TcpStream;

static DEFAULT_PORT_NUMBER: u16 = 6667;

fn main() {
    let mut opts = Options::new();
    opts.optopt("s", "server", "Address", "ADDRESS");
    opts.optopt("p", "port", "Port", "PORT");

    let args = Vec::from_iter(env::args());
    let program = args[0].clone();

    let matches = match opts.parse(args) {
        Ok(m) => { m }
        Err(e) => {
            println!("Invalid address: {}", e);
            return usage(&program, opts);
        }
    };

    let hostname = match matches.opt_str("s") {
        Some(a) => { a }
        None => { return usage(&program, opts); }
    };

    let port = if let Some(p) = matches.opt_str("p") {
        match u16::from_str(&p) {
            Ok(p) => { p }
            Err(e) => {
                println!("Invalid port: {}", e);
                return usage(&program, opts);
            }
        }
    }
    else {
        DEFAULT_PORT_NUMBER
    };

    println!("Connecting to {}:{}...", hostname, port);

    match tcp_connect(&hostname, port) {
        Ok(stream) => {
            println!("Connected.");

            let stream_buf = BufReader::new(stream);
            let mut message_stream = irc::MessageStream::new(stream_buf);

            while let Ok(message) = message_stream.read_one() {
                match message.command {
                    irc::Command::Notice(params) => {
                        println!("Notice @ {} : {}",
                                 params.target,
                                 params.message);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}!", e);
        }
    }
}

fn usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn tcp_connect(hostname: &str, port: u16) -> Result<IrcStream, IoError> {
    let address = format!("{}:{}", hostname, port);
    let tcp_stream = try!(TcpStream::connect(&address[..]));
    Ok(tcp_stream)
}
