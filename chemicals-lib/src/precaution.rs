use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Precaution {
    pub statement: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PrecautionFields {
    Statement
}

impl FieldName for PrecautionFields {}

impl Display for PrecautionFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrecautionFields::Statement => write!(f, "Statement")
        }
    }
}

impl FromStr for PrecautionFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Statement" => Ok(PrecautionFields::Statement),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for Precaution {

    type FieldNames = PrecautionFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref statement) = values[0] {
            Ok(Precaution {
                statement: statement.clone()
            })
        } else {
            Err("Incorrect type statement. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![PrecautionFields::Statement]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.statement.clone())]
    }

    fn get_field(&self, field_name: PrecautionFields) -> Option<Value> {
        match field_name {
            PrecautionFields::Statement => Some(Value::String(self.statement.clone()))
        }
    }
}

#[cfg(test)]
mod precuation_tests {

    use std::str::FromStr;

    use precaution::PrecautionFields;
    use precaution::Precaution;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_precautionfields_from_str() {
        let precaution_field = PrecautionFields::from_str("Statement");
        assert_eq!(precaution_field, Ok(PrecautionFields::Statement));
    }

    #[test]
    fn test_precaution_from_fields() {
        let fields = [
            Value::String("916225746".to_string())
        ];

        let precaution = Precaution::from_fields(&fields).unwrap();

        assert_eq!(precaution.statement, "916225746".to_string());
    }

    #[test]
    fn test_precaution_get_field_names() {
        let field_names = Precaution::get_field_names();

        assert_eq!(field_names[0], PrecautionFields::Statement);
    }

    #[test]
    fn test_precaution_get_fields() {
        let precaution = Precaution {
            statement: "444".to_string()
        };

        let fields = precaution.get_fields();

        assert_eq!(fields[0], Value::String("444".to_string()));
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_precaution_get_field() {
        let precaution = Precaution {
            statement: "1234".to_string()
        };

        let statement = precaution.get_field(PrecautionFields::Statement);

        assert_eq!(statement, Some(Value::String("1234".to_string())));
    }
}