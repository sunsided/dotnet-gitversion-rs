fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotnet_gitversion_build::build()?;
    Ok(())
}
