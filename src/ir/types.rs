
#[derive(Debug, Default, Clone)]
pub enum Type {
    #[default]
    Void,
    Char,
    Number,
    Boolean,
    Pointer(Box<Type>),
    Array(Box<Type>),
}
