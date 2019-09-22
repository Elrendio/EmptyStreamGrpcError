mod oneof;
mod oneof_grpc;

use futures::{sink::Sink, stream::Stream, Future};
use grpcio::RpcContext;
use std::sync::Arc;
use std::time::Duration;

/// This function is responsible for building the gRPC server
/// and registering gRPC services.
pub fn build_server() -> grpcio::Server {
	let port: u16 = std::env::var("PORT")
		.unwrap_or_else(|_| "4000".to_owned())
		.parse()
		.expect("PORT should be an u16");

	let env = Arc::new(grpcio::Environment::new(10));
	grpcio::ServerBuilder::new(Arc::clone(&env))
		.channel_args(
			grpcio::ChannelBuilder::new(env)
				.keepalive_permit_without_calls(true)
				.http2_max_ping_strikes(std::i32::MAX)
				.http2_max_pings_without_data(0)
				.http2_min_recv_ping_interval_without_data(Duration::from_secs(50))
				.http2_min_sent_ping_interval_without_data(Duration::from_secs(305))
				.keepalive_time(Duration::from_secs(310))
				.keepalive_timeout(Duration::from_secs(340))
				.build_args(),
		)
		.register_service(build_service())
		.bind("0.0.0.0", port)
		.build()
		.unwrap()
}

pub fn build_service() -> grpcio::Service {
	oneof_grpc::create_oneof(Oneof)
}

#[derive(Clone)]
struct Oneof;

impl oneof_grpc::Oneof for Oneof {
	fn test(
		&mut self,
		ctx: RpcContext,
		_req: oneof::MessageWithOneof,
		sink: grpcio::UnarySink<oneof::Empty>,
	) {
		ctx.spawn(sink.success(oneof::Empty::new()).map_err(|_| ()))
	}
}
