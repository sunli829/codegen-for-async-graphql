use async_graphql_parser::schema::ObjectType;

use super::{BaseType, FileRender, RenderType, RendererFieldType, SupportFields};

pub type RendererObjectType<'a, 'b> = BaseType<'a, 'b, ObjectType>;

impl<'a, 'b> FileRender for RendererObjectType<'a, 'b> {}

impl<'a, 'b> RenderType for RendererObjectType<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

impl<'a, 'b> SupportFields for RendererObjectType<'a, 'b> {
    #[must_use]
    fn fields(&self) -> Vec<RendererFieldType> {
        self.doc
            .fields
            .iter()
            .map(|f| RendererFieldType {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }
}

impl<'a, 'b> RendererObjectType<'a, 'b> {
    pub fn implements_interfaces(&self) -> Vec<String> {
        self.doc
            .implements_interfaces
            .iter()
            .map(|f| f.node.clone())
            .collect()
    }
}
