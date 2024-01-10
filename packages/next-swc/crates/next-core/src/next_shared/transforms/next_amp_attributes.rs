use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::amp_attributes::amp_attributes;
use turbopack_binding::{
    swc::core::{
        common::util::take::Take,
        ecma::{ast::*, visit::FoldWith},
    },
    turbopack::ecmascript::{CustomTransformer, TransformContext},
};

#[derive(Debug)]
struct NextAmpAttributes {}

#[async_trait]
impl CustomTransformer for NextAmpAttributes {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));
        *program = p.fold_with(&mut amp_attributes());
        Ok(())
    }
}
