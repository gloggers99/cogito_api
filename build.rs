use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Compile gRPC .proto files.
    tonic_prost_build::compile_protos("proto/cogito.proto")?;

    Ok(())
}
