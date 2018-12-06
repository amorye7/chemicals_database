use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Hazard {
    pub statement: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HazardFields {
    Statement
}

impl FieldName for HazardFields {}

impl Display for HazardFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HazardFields::Statement => write!(f, "Statement")
        }
    }
}

impl FromStr for HazardFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Statement" => Ok(HazardFields::Statement),
            _=> Err("Fields does not exist".to_string())
        }
    }
}

impl Entry for Hazard {

    type FieldNames = HazardFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref statement) = values[0] {
            Ok(Hazard {
                statement: statement.clone()
            })
        } else {
            Err("Incorrect type for statement. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![HazardFields::Statement]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.statement.clone())]
    }

    fn get_field(&self, field_name: HazardFields) -> Option<Value> {
        match field_name {
            HazardFields::Statement => Some(Value::String(self.statement.clone()))
        }
    }
}

#[cfg(test)]
mod hazard_tests {

    use std::str::FromStr;
    
    use hazard::HazardFields;
    use hazard::Hazard;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_hazardfields_from_str() {
        let hazard_field = HazardFields::from_str("Statement");
        assert_eq!(hazard_field, Ok(HazardFields::Statement));
    }

    #[test]
    fn test_hazard_from_fields() {
        let fields = [
            Value::String("Danger".to_string())
        ];

        let hazard = Hazard::from_fields(&fields).unwrap();

        assert_eq!(hazard.statement, "Danger".to_string());
    }

    #[test]
    fn test_hazard_get_field_names() {
        let field_names = Hazard::get_field_names();

        assert_eq!(field_names[0], HazardFields::Statement);
    }

    #[test]
    fn test_hazard_get_fields() {
        let hazard = Hazard {
            statement: "Warning".to_string()
        };

        let fields = hazard.get_fields();

        assert_eq!(fields[0], Value::String("Warning".to_string()));
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_hazard_get_field() {
        let hazard = Hazard {
            statement: "Peaceful".to_string()
        };

        let statement = hazard.get_field(HazardFields::Statement);

        assert_eq!(statement, Some(Value::String("Peaceful".to_string())));
    }
}