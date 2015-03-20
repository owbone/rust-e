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
use std::io::{BufRead, Write};
use std::io::Error as IoError;

pub enum Error {
    InvalidLine(String),
    InvalidCommand(String),
    InvalidParams(&'static str, String),
    IoError(IoError),
}

pub enum Command {
    Notice(NoticeParams),
}

pub struct NoticeParams {
    pub target: String,
    pub message: String,
}

pub struct Message {
    pub prefix: Option<String>,
    pub command: Command,
}

pub struct MessageStream<S> {
    inner: S,
}

impl<S: BufRead + Write> MessageStream<S> {
    pub fn new(inner: S) -> MessageStream<S> {
        return MessageStream {
            inner: inner
        }
    }

    pub fn read_one(&mut self) -> Result<Message, Error> {
        let mut line = String::new();
        if let Err(error) = self.inner.read_line(&mut line) {
            return Err(Error::IoError(error));
        }
        self.parse_line(&line)
    }

    fn parse_line(&self, line: &str) -> Result<Message, Error> {
        static RE: Regex = regex!(r"(?::(?P<prefix>\S+) )?(?P<command>.+)");

        if let Some(captures) = RE.captures(&line) {
            Ok(Message {
                prefix: match captures.name("prefix") {
                    Some(s) => { Some(s.to_string()) }
                    None => { None }
                },
                command: match captures.name("command") {
                    Some(s) => { 
                        match self.parse_command(s) {
                            Ok(c) => { c }
                            Err(e) => { return Err(e); }
                        }
                    }
                    None => { 
                        return Err(Error::InvalidLine(line.to_string()));
                    }
                }
            })
        }
        else {
            Err(Error::InvalidLine(line.to_string()))
        }
    }

    fn parse_command(&self, command: &str) -> Result<Command, Error> {
        static RE: Regex = regex!(r"(?P<type>\S+)( (?P<params>.+))?");

        if let Some(captures) = RE.captures(&command) {
            if let Some(name) = captures.name("type") {
                let params = captures.name("params").unwrap_or("");

                match name {
                    "NOTICE" => { self.parse_notice(params) }
                    _ => { Err(Error::InvalidCommand(command.to_string())) }
                }
            }
            else {
                Err(Error::InvalidCommand(command.to_string()))
            }
        }
        else {
            Err(Error::InvalidCommand(command.to_string()))
        }
    }

    fn parse_notice(&self, params: &str) -> Result<Command, Error> {
        static RE: Regex = regex!(r"(?P<target>\S+) :?(?P<message>.*)");

        if let Some(captures) = RE.captures(params) {
            Ok(Command::Notice(NoticeParams {
                target: match captures.name("target") {
                    Some(target) => {
                        target.to_string()
                    }
                    None => {
                        return Err(Error::InvalidParams("NOTICE", 
                                                        params.to_string()))
                    }
                },
                message: match captures.name("message") {
                    Some(message) => {
                        message.to_string() 
                    }
                    None => { 
                        return Err(Error::InvalidParams("NOTICE", 
                                                        params.to_string())) 
                    }
                }
            }))
        }
        else {
            Err(Error::InvalidParams("NOTICE", params.to_string()))
        }
    }
}

