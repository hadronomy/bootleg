use shadow_rs::ShadowBuilder;

fn main() -> shadow_rs::SdResult<()> {
    ShadowBuilder::builder().build().expect("Failed to build shadow.rs");
    Ok(())
}
