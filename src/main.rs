use std::str;
use nom::{
    bytes::complete::tag, 
    bytes::complete::take_until,
    number::complete::float,
    error::ParseError, 
    sequence::tuple, IResult

};

#[derive(Debug)]
enum Method {
    GET,
    POST
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET => write!(f,"GET"),
            Self::POST => write!(f,"POST")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Request {
    method: String,
    url: String,
    protocol: String,
    version: f32,
}
fn parse_method(input: &str) -> IResult<&str,&str,nom::error::Error<&str>>{
    for method in vec![Method::GET, Method::POST] {
        if let Ok(matched) = tag::<&str,&str,nom::error::Error<&str>>(method.to_string().as_str())(input) {
            return Ok(matched);
        }
    }
    Err(nom::Err::Error(nom::error::Error::from_error_kind(input, nom::error::ErrorKind::Fail)))
}

fn parse_whitespace(input: &str) -> IResult<&str,&str,nom::error::Error<&str>> {
    tag(" ")(input) 
}


fn parse_request(input: &str) -> IResult<&str, &str, nom::error::Error<&str>>{
    match parse_whitespace(input) {
        Ok(result) => {    
            take_until(" ")(result.0)
        }
        Err(e) => Err(e)
    }    
}

fn parse_protocol(input: &str) -> IResult<&str,&str, nom::error::Error<&str>> {
    match parse_whitespace(input) {
        Ok(result) => {    
            take_until("/")(result.0)
        }
        Err(e) => Err(e)
    }    
}

fn parse_version(input: &str) -> IResult<&str,f32,nom::error::Error<&str>> {
    match tag::<&str,&str,nom::error::Error<&str>>("/")(input) {
        Ok(result) => {
            float(result.0)
        }
        Err(e) => Err(e)
    }
}

fn run_parser(input: &str) -> Result<Request,String> {
    let result = tuple((parse_method,parse_request,parse_protocol,parse_version))(input);
    match result {
        Ok(values) => {
            Ok(Request {
                method: values.1.0.to_string(),
                url: values.1.1.to_string(),
                protocol: values.1.2.to_string(),
                version: values.1.3,
            })
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

fn main() {
    let get = "GET /home/ HTTP/1.1\r\n";
    println!("GET: {:#?}",run_parser(get));
    assert!(run_parser(get).is_ok());
    let post = "POST /update/ HTTP/1.1\r\n";
    println!("POST: {:#?}",run_parser(post));
    assert!(run_parser(post).is_ok());
    let wrong = "WRONG /wrong/ HTTP/1.1\r\n";
    println!("WRONG: {:#?}",run_parser(wrong));
    assert!(run_parser(wrong).is_err());
}
