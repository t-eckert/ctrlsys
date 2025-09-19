fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/timer/ctrlsys/timer/v1/timer.proto")?;
    Ok(())
}
