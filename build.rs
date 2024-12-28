use shadow_rs::ShadowBuilder;

fn main() -> shadow_rs::SdResult<()> {
    ShadowBuilder::builder().build().expect("failed to build shadow.rs");
    Ok(())
}
