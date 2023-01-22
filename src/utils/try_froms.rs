use crate::{prelude::*, errors::{AnyError, DatabaseError}};
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<W<Value>> for Object {
    type Error = AnyError;
    fn try_from(val: W<Value>) -> Result<Object, AnyError> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(AnyError::DatabaseError(DatabaseError::Other)),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = AnyError;
    fn try_from(val: W<Value>) -> Result<Array, AnyError> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(AnyError::DatabaseError(DatabaseError::Other)),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = AnyError;
    fn try_from(val: W<Value>) -> Result<i64, AnyError> {
        match val.0 {
			Value::Number(obj) => Ok(obj.as_int()),
			_ => Err(AnyError::DatabaseError(DatabaseError::Other)),
		}
    }
}

impl TryFrom<W<Value>> for bool {
	type Error = AnyError;
	fn try_from(val: W<Value>) -> Result<bool, AnyError> {
		match val.0 {
			Value::False => Ok(false),
			Value::True => Ok(true),
            _ => Err(AnyError::DatabaseError(DatabaseError::Other)),
		}
	}
}

impl TryFrom<W<Value>> for String {
	type Error = AnyError;
	fn try_from(val: W<Value>) -> Result<String, AnyError> {
		match val.0 {
			Value::Strand(strand) => Ok(strand.as_string()),
			Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(AnyError::DatabaseError(DatabaseError::Other)),
		}
	}
}