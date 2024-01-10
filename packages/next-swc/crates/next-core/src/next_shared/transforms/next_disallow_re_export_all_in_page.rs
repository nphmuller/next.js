use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::disallow_re_export_all_in_page::disallow_re_export_all_in_page;
use turbopack_binding::{
    swc::core::{
        common::util::take::Take,
        ecma::{ast::*, visit::FoldWith},
    },
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextDisallowReExportAllInPage {
    is_page_file: bool,
}

#[async_trait]
impl CustomTransformer for NextDisallowReExportAllInPage {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));
        *program = p.fold_with(&mut disallow_re_export_all_in_page(self.is_page_file));
        Ok(())
    }
}
