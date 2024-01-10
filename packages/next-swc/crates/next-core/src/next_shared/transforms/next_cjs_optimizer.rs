use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::cjs_optimizer::{cjs_optimizer, Config};
use turbopack_binding::{
    swc::core::{
        common::SyntaxContext,
        ecma::{ast::*, visit::VisitMutWith},
    },
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextCjsOptimizer {
    config: Config,
}

#[async_trait]
impl CustomTransformer for NextCjsOptimizer {
    async fn transform(&self, program: &mut Program, ctx: &TransformContext<'_>) -> Result<()> {
        let mut visitor = cjs_optimizer(
            self.config.clone(),
            SyntaxContext::empty().apply_mark(ctx.unresolved_mark),
        );

        program.visit_mut_with(&mut visitor);
        Ok(())
    }
}
