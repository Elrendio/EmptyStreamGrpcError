use std::path::Path;

pub fn main() {
	protoc_grpcio::compile_grpc_protos::<_, Vec<&Path>, _>(
		&vec![Path::new("./src/oneof.proto")],
		vec![&Path::new("./src")],
		Path::new("./src"),
		Some(protobuf_codegen::Customize {
			generate_accessors: Some(false),
			..std::default::Default::default()
		}),
	)
	.expect("gRPC codegen failed")
}
