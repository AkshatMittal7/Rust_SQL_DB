//Defines datatype of the database
#[cfg(not(feature = "json"))]
#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

impl DataType {
    pub fn name(&self) -> String {
        match self {
            Self::String(_) => {
                return "String".to_string();
            }
            Self::Integer32(_) => {
                return "i32".to_string();
            }
            Self::Float32(_) => {
                return "f32".to_string();
            }
        }
    }
}
