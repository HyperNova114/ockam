#[ockam::test]
async fn my_test(ctx: &mut ockam_node::Context) -> ockam_core::Result<()> {
    ctx.address();
    Ok(())
}

fn main() {}
