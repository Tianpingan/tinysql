use crate::catalog::column::DataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    // NULL is less than any non-NULL values
    // Null,
    Boolean(bool),
    TinyInt(i8),
    SmallInt(i16),
    Integer(i32),
}
impl Value {
    pub fn from_bytes(bytes: &[u8], data_type: DataType) -> Self {
        match data_type {
            DataType::Boolean => Self::Boolean(Self::boolean_from_bytes(bytes)),
            DataType::TinyInt => Self::TinyInt(i8::from_be_bytes([bytes[0]])),
            DataType::SmallInt => Self::SmallInt(i16::from_be_bytes([bytes[0], bytes[1]])),
            DataType::Integer => {
                Self::Integer(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            _ => panic!("Not implemented"),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Boolean(v) => Self::boolean_to_bytes(*v),
            Self::TinyInt(v) => v.to_be_bytes().to_vec(),
            Self::SmallInt(v) => v.to_be_bytes().to_vec(),
            Self::Integer(v) => v.to_be_bytes().to_vec(),
        }
    }

    pub fn from_sqlparser_value(value: &sqlparser::ast::Value, data_type: DataType) -> Self {
        match value {
            sqlparser::ast::Value::Number(v, _) => match data_type {
                DataType::TinyInt => Self::TinyInt(v.parse::<i8>().unwrap()),
                DataType::SmallInt => Self::SmallInt(v.parse::<i16>().unwrap()),
                DataType::Integer => Self::Integer(v.parse::<i32>().unwrap()),
                _ => panic!("Not implemented"),
            },
            // sqlparser::ast::Value::SingleQuotedString(_) => {}
            sqlparser::ast::Value::Boolean(b) => Value::Boolean(*b),
            _ => unreachable!(),
        }
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Boolean(v1) => match other {
                Self::Boolean(v2) => v1.cmp(v2),
                _ => panic!("Not implemented"),
            },
            Self::TinyInt(v1) => match other {
                Self::TinyInt(v2) => v1.cmp(v2),
                _ => panic!("Not implemented"),
            },
            Self::SmallInt(v1) => match other {
                Self::SmallInt(v2) => v1.cmp(v2),
                _ => panic!("Not implemented"),
            },
            Self::Integer(v1) => match other {
                Self::Integer(v2) => v1.cmp(v2),
                _ => panic!("Not implemented"),
            },
        }
    }

    pub fn boolean_from_bytes(bytes: &[u8]) -> bool {
        bytes[0] != 0
    }
    pub fn boolean_to_bytes(value: bool) -> Vec<u8> {
        if value {
            vec![1]
        } else {
            vec![0]
        }
    }
}
