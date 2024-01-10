use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::page_config::page_config;
use turbopack_binding::{
    swc::core::{
        common::util::take::Take,
        ecma::{ast::*, visit::FoldWith},
    },
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextPageConfig {
    is_development: bool,
    is_page_file: bool,
}

#[async_trait]
impl CustomTransformer for NextPageConfig {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));

        *program = p.fold_with(&mut page_config(self.is_development, self.is_page_file));
        Ok(())
    }
}
