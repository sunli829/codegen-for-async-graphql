mod extension;

use super::{
    Context, FieldRenderer, FileRender, Output, RenderDependencies, RenderType,
    RendererInterfaceType, Save, SupportFields,
};

use extension::Renderer;

use proc_macro2::TokenStream;

use async_graphql_parser::schema::InterfaceType;

impl Save for InterfaceType {}

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().interface_types().iter().for_each(|f| {
            Renderer::model_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .interface_types()
            .iter()
            .map(|f| Renderer::token_stream(f))
            .collect()
    }
}
