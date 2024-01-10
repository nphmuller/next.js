use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::shake_exports::{shake_exports, Config};
use turbopack_binding::{
    swc::core::{
        common::util::take::Take,
        ecma::{ast::*, visit::FoldWith},
    },
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextShakeExports {
    ignore: Vec<String>,
}

#[async_trait]
impl CustomTransformer for NextShakeExports {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));

        *program = p.fold_with(&mut shake_exports(Config {
            ignore: self.ignore.iter().map(|s| s.clone().into()).collect(),
        }));
        Ok(())
    }
}
