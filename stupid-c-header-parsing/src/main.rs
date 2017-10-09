extern crate regex;
extern crate xml;
#[macro_use]
extern crate unwrap;

use std::fs::File;
use std::io::{Read, BufReader};
use xml::reader::{EventReader, XmlEvent};
use regex::Regex;
use std::mem;
use std::ascii::AsciiExt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Param {
    name: String,
    param_type: String,
}

#[derive(Debug, Default)]
struct Res {
    function_name: String,
    function_ret: String,
    function_params: Vec<Param>,
    callback_director: String,
    callback_virtuals: Vec<String>,
}

fn handle_type<R: Read>(mut parser: EventReader<R>, result: &mut Res) -> EventReader<R> {
    let e = unwrap!(parser.next());
    result.function_ret = match e {
        XmlEvent::Characters(value) => value,
        x => panic!("{:?}", x),
    };
    parser
}

fn handle_name<R: Read>(mut parser: EventReader<R>, result: &mut Res) -> EventReader<R> {
    let e = unwrap!(parser.next());
    result.function_name = match e {
        XmlEvent::Characters(value) => value,
        x => panic!("{:?}", x),
    };
    parser
}

fn handle_param<R: Read>(mut parser: EventReader<R>, result: &mut Res) -> EventReader<R> {
    let mut param = Param::default();
    loop {
        let e = unwrap!(parser.next());
        match e {
            XmlEvent::StartElement { name, .. } => {
                let e = unwrap!(parser.next());
                if name.local_name == "type" {
                    param.param_type = match e {
                        XmlEvent::Characters(value) => value,
                        x => panic!("{:?}", x),
                    };
                } else if name.local_name == "declname" {
                    param.name = match e {
                        XmlEvent::Characters(value) => value,
                        x => panic!("{:?}", x),
                    };
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "param" {
                    break;
                }
            }
            XmlEvent::Whitespace(_) => (),
            x => panic!("{:?}", x),
        }
    }
    result.function_params.push(param);
    parser
}

fn normalise(res: &mut Res) {
    let re_user_data = unwrap!(Regex::new(r"^void\s*\*"));
    let re_cb = unwrap!(Regex::new(
        r"^void\s*\(\s*\*\s*\)\s*\(void\s*\*\s*\w+,\s*(\w+.+)\)",
    ));
    let function_params = mem::replace(&mut res.function_params, Default::default());

    let convert = Rc::new(RefCell::new(false));
    let convert_clone = convert.clone();
    let mut first = true;
    res.callback_director = res.function_name
        .chars()
        .filter(|c| if *c == '_' {
            *convert.borrow_mut() = true;
            false
        } else {
            true
        })
        .map(|c| if *convert_clone.borrow() || first {
            let new_c = c.to_ascii_uppercase();
            *convert_clone.borrow_mut() = false;
            first = false;
            new_c
        } else {
            c.clone()
        })
        .collect::<String>();
    res.callback_director.push_str("Cb");

    for param in function_params {
        if re_user_data.is_match(&param.param_type) {
            let mut param_type = res.callback_director.clone();
            param_type.push_str(" *");
            res.function_params.push(Param {
                param_type,
                name: "obj".to_string(),
            });
            continue;
        } else if let Some(caps) = re_cb.captures(&param.param_type) {
            let params = unwrap!(caps.get(1)).as_str();
            res.callback_virtuals.push(format!(
                "virtual void {}({}) = 0;",
                param.name,
                params
            ));
            continue;
        }
        res.function_params.push(param);
    }
}

fn main() {
    let fs = unwrap!(File::open("backend_8h.xml"));
    let file = BufReader::new(fs);

    let mut parser = EventReader::new(file);
    let mut results = Vec::new();
    loop {
        let event = unwrap!(parser.next());

        match event {
            XmlEvent::StartElement { name, attributes, .. } => {
                if name.local_name != "memberdef" {
                    continue;
                }
                let mut found = false;
                for attrib in attributes {
                    if attrib.name.local_name == "kind" {
                        if attrib.value == "function" {
                            found = true;
                        } else {
                            break;
                        }
                    }
                }
                if !found {
                    continue;
                }
                let mut res = Res::default();
                loop {
                    let e = unwrap!(parser.next());
                    match e {
                        XmlEvent::StartElement { name, .. } => {
                            if name.local_name == "type" {
                                parser = handle_type(parser, &mut res);
                            } else if name.local_name == "name" {
                                parser = handle_name(parser, &mut res);
                            } else if name.local_name == "param" {
                                parser = handle_param(parser, &mut res);
                            }
                        }
                        XmlEvent::EndElement { name } => {
                            if name.local_name == "memberdef" {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                normalise(&mut res);
                results.push(res);
            }
            XmlEvent::EndDocument => break,
            _ => (),
        }
    }

    println!("{:?}", results);
}
