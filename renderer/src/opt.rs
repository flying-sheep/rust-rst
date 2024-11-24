use derive_builder::Builder;

#[derive(Builder, Debug, Default, PartialEq, Eq, Hash, Clone)]
#[builder(derive(Debug, PartialEq, Eq, Hash))]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct RenderOptions {
    // The initial header level
    #[builder(default = 0)]
    pub initial_header_level: u8,
    /// Produce a standalone document
    #[builder(default = true)]
    pub standalone: bool,
}

impl RenderOptionsBuilder {
    pub fn build(&mut self) -> RenderOptions {
        self.fallible_build()
            .expect("All required fields set at initialization")
    }
}

impl From<bool> for RenderOptions {
    fn from(standalone: bool) -> Self {
        RenderOptionsBuilder::default()
            .standalone(standalone)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::RenderOptionsBuilder;

    #[test]
    fn no_required_fields_added_to_render_options() {
        RenderOptionsBuilder::default().build();
    }
}
