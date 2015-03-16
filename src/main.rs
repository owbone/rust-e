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

use getopts::Options;
use std::env;
use std::str::FromStr;
use std::net::TcpStream;
use std::u8;

static DEFAULT_PORT_NUMBER: u8 = 6667;

fn main() {
    let mut opts = Options::new();
    opts.optopt("s", "server", "Address", "ADDRESS");
    opts.optopt("p", "port", "Port", "PORT");

    let matches = match opts.parse(env::args()) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()); }
    };

    let hostname = match matches.opt_str("s") {
        Some(a) => { a }
        None => { panic!("Server address is required!"); }
    };

    let port = if let Some(p) = matches.opt_str("p") {
        match u8::from_str(p.as_slice()) {
            Ok(p) => { p }
            Err(e) => { panic!(format!("Invalid port number: {}", p)); }
        }
    }
    else {
        DEFAULT_PORT_NUMBER
    };

    if !connect(hostname.as_slice(), port) {
        println!("Failed to connect to server!");
    }
}

fn connect(hostname: &str, port: u8) -> bool {
    let address = format!("{}:{}", hostname, port);

    let mut stream = match TcpStream::connect(address.as_slice()) {
        Ok(s) => { s }
        Err(e) => { return false; }
    };

    true
}
