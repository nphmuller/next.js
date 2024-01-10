use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::pure::pure_magic;
use turbopack_binding::{
    swc::core::ecma::{ast::*, visit::VisitMutWith},
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextPure {}

#[async_trait]
impl CustomTransformer for NextPure {
    async fn transform(&self, program: &mut Program, ctx: &TransformContext<'_>) -> Result<()> {
        program.visit_mut_with(&mut pure_magic(ctx.comments.clone()));
        Ok(())
    }
}
