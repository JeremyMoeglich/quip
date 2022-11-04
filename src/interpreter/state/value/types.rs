use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
    None,
    Function,
    NativeFunction,
    Error,
    List,
    Reference,
    //Object,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Integer => write!(f, "Integer"),
            ValueType::Float => write!(f, "Float"),
            ValueType::String => write!(f, "String"),
            ValueType::Boolean => write!(f, "Boolean"),
            ValueType::None => write!(f, "None"),
            ValueType::Function => write!(f, "Function"),
            ValueType::NativeFunction => write!(f, "NativeFunction"),
            ValueType::Error => write!(f, "Error"),
            ValueType::List => write!(f, "List"),
            ValueType::Reference => write!(f, "Reference"),
            //ValueType::Object => write!(f, "Object"),
        }
    }
}
