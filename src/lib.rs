use core::fmt::Debug;

pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveError<'a> {
    MethodNotFound {
        struct_name: &'a str,
        method_name: &'a str,
    },
    AttributeNotFound {
        struct_name: &'a str,
        attribute_name: &'a str,
    },
    InstanceNotFound {
        instance_name: &'a str,
    },
}

pub trait Interactive<'a>: Debug {
    fn __interactive_get_attribute(&'a self, attribute_name: &'a str) -> Result<'a, &dyn Debug>;
    fn __interactive_get_interactive_attribute(
        &'a mut self,
        attribute_name: &'a str,
    ) -> Result<'a, &mut dyn Interactive>;
    fn __interactive_call_method(
        &'a mut self,
        method_name: &'a str,
        args: &'a str,
    ) -> Result<'a, Option<&mut dyn Debug>>;
}
