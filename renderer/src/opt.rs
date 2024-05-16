#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RenderOptions {
    initial_header_level: u8,
}

pub trait RenderOptionsBuilder {
    fn initial_header_level(&self) -> u8;
    fn with_initial_header_level(&mut self, lvl: u8) -> &mut Self;
}

impl RenderOptionsBuilder for RenderOptions {
    fn initial_header_level(&self) -> u8 {
        self.initial_header_level
    }
    fn with_initial_header_level(&mut self, lvl: u8) -> &mut Self {
        self.initial_header_level = lvl;
        self
    }
}

// standalone document formats

pub struct RenderOptionsStandalone {
    render_options: RenderOptions,
    standalone: bool,
}

impl Default for RenderOptionsStandalone {
    fn default() -> Self {
        RenderOptionsStandalone {
            render_options: Default::default(),
            standalone: true,
        }
    }
}

pub trait RenderOptionsStandaloneBuilder {
    fn into_render_options(self) -> RenderOptions;
    fn render_options(&self) -> &RenderOptions;
    fn render_options_mut(&mut self) -> &mut RenderOptions;
    fn standalone(&self) -> bool;
    fn with_standalone(&mut self, standalone: bool) -> &mut Self;
}

impl<T> RenderOptionsBuilder for T
where
    T: RenderOptionsStandaloneBuilder,
{
    fn initial_header_level(&self) -> u8 {
        self.render_options().initial_header_level()
    }
    fn with_initial_header_level(&mut self, lvl: u8) -> &mut Self {
        self.render_options_mut().with_initial_header_level(lvl);
        self
    }
}

impl RenderOptionsStandaloneBuilder for RenderOptionsStandalone {
    fn into_render_options(self) -> RenderOptions {
        self.render_options
    }
    fn render_options(&self) -> &RenderOptions {
        &self.render_options
    }
    fn render_options_mut(&mut self) -> &mut RenderOptions {
        &mut self.render_options
    }
    fn standalone(&self) -> bool {
        self.standalone
    }
    fn with_standalone(&mut self, standalone: bool) -> &mut Self {
        self.standalone = standalone;
        self
    }
}

impl From<bool> for RenderOptionsStandalone {
    fn from(standalone: bool) -> Self {
        let mut r: RenderOptionsStandalone = Default::default();
        r.with_standalone(standalone);
        r
    }
}

impl<T> From<T> for RenderOptions
where
    T: RenderOptionsStandaloneBuilder,
{
    fn from(opts: T) -> Self {
        opts.into_render_options()
    }
}
