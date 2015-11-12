#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate chrono;

use rustc_serialize::json;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use chrono::offset::local::Local;
use chrono::offset::TimeZone;


#[derive(Debug, RustcEncodable)]
struct SyslogLine {
    date: String,
    machine: String,
    tag: String,
    msg: String
}


fn append_msg(s: &mut Option<SyslogLine>, line: String) {
    match s {
        &mut Some(ref mut s) => s.msg = format!("{}\n{}", s.msg, line),
        &mut None => ()
    }
}


fn main() {
    let f = File::open("/var/log/system.log").unwrap();
    let mut file = BufReader::new(&f);
    let mut buffer = String::new();
    let mut line = None;
    while file.read_line(&mut buffer).unwrap() > 0 {        
        if buffer.len() > 15 {
            let datestr = &format!("{} 2015", &buffer[0..15]);
            let date = match Local.datetime_from_str(datestr, "%b %e %T %Y") {
                Ok(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
                Err(e) => {
                    append_msg(&mut line, buffer.clone());
                    buffer.clear();
                    continue
                }
            };
            let rest = &buffer.clone()[16..];
            let colon = match rest.find(':') {
                Some(c) => c,
                None => {
                    append_msg(&mut line, buffer.clone());
                    buffer.clear();
                    continue
                }
            };
            let mut data = rest[..colon].split(' ');
            let machine = match data.next() {
                Some(m) => m,
                None => {
                    append_msg(&mut line, buffer.clone());
                    buffer.clear();
                    continue
                }
            };
            let tag = match data.next() {
                Some(t) => t,
                None => {
                    append_msg(&mut line, buffer.clone());
                    buffer.clear();
                    continue
                }
            };
            let msg = &rest[colon+2..];
            match line {
                Some(l) => println!("{}", json::encode(&l).unwrap()),
                None => ()
            };
            line = Some(SyslogLine{date: date, machine: machine.to_string(), tag: tag.to_string(), msg: msg.to_string()});
        } else {
            append_msg(&mut line, buffer.clone())
        }
        
        buffer.clear();
    }
}
