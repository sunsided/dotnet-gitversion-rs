fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the `gitversion.rs` file.
    // The intermediate representation is returned for use by subsequent build steps.
    let _version = dotnet_gitversion_build::build()?;

    Ok(())
}
