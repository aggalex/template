pub trait Template {
    type Output;

    fn define(self) -> <Self as Template>::Output;
}

pub trait TemplateConstruction: /*FnOnce() + */ Default + Template {
    fn on_create(&mut self, f: impl FnOnce(&mut Self::Output) + 'static);
    fn create(self) -> Self::Output;
    fn build<O>(self, f: impl FnOnce(Self::Output) -> O) -> O;
}