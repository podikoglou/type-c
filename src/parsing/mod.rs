use anyhow::Result;

pub mod swc;

pub trait ToIR<T> {
    fn to_ir(&self) -> Result<T>;
}

#[macro_export]
macro_rules! def_parser {
    ($ast_type:ty, $ir_type:ty, |$param:ident| $body:expr) => {
        impl crate::parsing::ToIR<$ir_type> for $ast_type {
            fn to_ir(&self) -> anyhow::Result<$ir_type> {
                (|$param: &$ast_type| $body)(self)
            }
        }
    };
}
