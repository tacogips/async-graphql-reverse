pub struct RendererConfig<'a> {
    pub custom_datasource_using: Option<&'a str>,
}

impl<'a> RendererConfig<'a> {
    pub fn data_source_using(&self) -> &'a str {
        self.custom_datasource_using
            .unwrap_or("use crate::datasource::DataSource")
    }
}
