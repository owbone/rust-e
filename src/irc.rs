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

use regex::Regex;
use std::io::{BufRead, Error, Write};

static RE_MESSAGE: Regex = regex!(r"(:(?P<prefix>\S+) )?(?P<command>.+)");

pub struct MessageStream<S> {
    inner: S
}

impl<S: BufRead + Write> MessageStream<S> {
    pub fn new(inner: S) -> MessageStream<S> {
        return MessageStream {
            inner: inner
        }
    }

    pub fn read_one(&mut self) -> Result<String, Error> {
        let mut line = String::new();
        try!(self.inner.read_line(&mut line));

        let mut message = String::new();
        if let Some(captures) = RE_MESSAGE.captures(&line) {
            if let Some(prefix) = captures.name("prefix") {
                message.push_str(&format!("Prefix: \"{}\", ", prefix));
            }
            if let Some(command) = captures.name("command") {
                message.push_str(&format!("Command: \"{}\"", command));
            }
        }
        else {
            // TODO: Error handling.
            panic!("Parse failure!");
        }

        return Ok(message);
    }
}

