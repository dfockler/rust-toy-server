use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};
use std::thread::Thread;
use std::old_io::File;

fn main() {

	//Get the Result from the bind
    let listener = TcpListener::bind("192.168.2.20:4567").unwrap();

    //Get the Result from the bind and get the contents
    let mut acceptor = listener.listen().unwrap();

    //Infinite loop over incoming connections
    for stream in acceptor.incoming() {

    	//match the stream based on the Result returned
    	match stream {
    		Err(e) => { println!("Error: {}", e); }
    		Ok(stream) => {
    			Thread::spawn(move|| {
    				handle_client(stream)
    			});
    		}
    	}
    }
}

fn handle_client(mut stream: TcpStream) {

	let mut buf = [0];
	println!("Status: {:?}", stream.read(&mut buf));

	let status = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n";
	stream.write(status.as_bytes());

	let contents = File::open(&Path::new("index.html")).read_to_end().unwrap();
	stream.write(contents.as_slice());
	
	stream.close_write();
}

