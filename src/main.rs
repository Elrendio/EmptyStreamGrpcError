use futures::future::Future;

fn main() {
	let mut server = empty_stream_error::build_server();
	server.start();

	for &(ref host, port) in server.bind_addrs() {
		println!("listening on {}:{}", host, port);
	}
	let (tx, rx) = std::sync::mpsc::channel();
	ctrlc::set_handler(move || match tx.send(()) {
		Ok(()) => {}
		Err(_) => println!("Could not send interrupt to listener"),
	})
	.expect("Error setting interrupt handler");

	rx.recv().unwrap(); // Wait for interrupt signal
	println!("Closing server...");
	server.shutdown().wait().unwrap();
}
