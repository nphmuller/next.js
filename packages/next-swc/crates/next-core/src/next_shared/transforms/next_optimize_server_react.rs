use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::optimize_server_react::{optimize_server_react, Config};
use turbopack_binding::{
    swc::core::{
        common::util::take::Take,
        ecma::{ast::*, visit::FoldWith},
    },
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextOptimizeServerReact {
    optimize_use_state: bool,
}

#[async_trait]
impl CustomTransformer for NextOptimizeServerReact {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));

        *program = p.fold_with(&mut optimize_server_react(Config {
            optimize_use_state: self.optimize_use_state,
        }));
        Ok(())
    }
}
