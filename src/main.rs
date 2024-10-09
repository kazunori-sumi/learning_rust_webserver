use std::net::TcpListener;

fn main() {
	println!("bind tcp listener!!!");
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("TcpListener is bound to the localhost:8080");

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		println!("Connection established!");
	}
}

