use std::io::prelude::*;
use std::net::TcpStream;

fn create_request(_path: std::option::Option<&str>, _host: std::option::Option<&str>, user_agent: &str) -> String {
    // craete request
    let mut http_request = String::new();
    http_request.push_str(format!("{}",
                                  match _path {
                                      None => "can't find Path!",
                                      Some(ref x) => x,
                                  }
                                  ).as_str());
    http_request.push_str("\r\n");
    http_request.push_str(format!("{}",
                                  match _host {
                                      None => "can't find Host!",
                                      Some(ref x) => x,
                                  }
                                  ).as_str());
    http_request.push_str("\r\n");
    http_request.push_str(format!("User-Agent: {}", user_agent).as_str());
    // http_request.push_str("\r\n");
    // http_request.push_str("Connection: close");
    http_request.push_str("\r\n");
    http_request.push_str("\r\n");

    // return request
    http_request
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    // show and parse request from client!
    println!("\nClient Request:\n{}", String::from_utf8_lossy(&buffer[..]));
    
    let data = String::from_utf8_lossy(&buffer);
    let mut data = data.split("\r\n");
    let _path = data.next();
    println!("firt line for parse:\n{:?}\n", _path);
    let _host = data.next();
    println!("second line for parse:\n{:?}\n", _host);

    let user_agent = "Mozilla/5.0 (windows NT; Windows NT 10.0; en-US)";
    
    let mut host = match _host {
        None => "can't parse host!".split(" "),
        Some(ref x) => x.split(" "),
    };
    host.next();
    let host = host.next();
    println!("host is:\n{:?}\n", host);

    // open tcp socket stream
    let mut stream_forward = TcpStream::connect(format!("{}",//"ident.me:80"
        match host {
            None => String::from("can't connect to parsed server!"),
            Some(ref x) => if x.contains(":") { format!("{}", x) } else { format!("{}:80", x) }, // what the hell, 80?
        }
        ).as_str()).expect("can't connect to server!");

    // create request ;)
    let request_as_bytes = create_request(_path, _host, user_agent);
    let request_as_bytes: &[u8] = request_as_bytes.as_bytes();

    println!("\nOur Request:\n{}", String::from_utf8_lossy(&request_as_bytes[..]));
    
    stream_forward
        .write_all(request_as_bytes)
        .expect("can't send as bytes!");
    
    // read response
    let mut http_response = String::new();
    stream_forward
        .read_to_string(&mut http_response)
        .expect("can't read response!");
    
    println!("\nOur Response:\n{}", http_response);
    
    // send response to client
    stream.write(http_response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // close tcp connection
    // stream_forward
    //     .shutdown(Shutdown::Both)
    //     .expect("can't close tcp connection!");
}
