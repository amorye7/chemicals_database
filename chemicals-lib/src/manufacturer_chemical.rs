use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct ManufacturerChemical {
    pub manufacturer_number: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ManufacturerChemicalFields {
    ManufacturerNumber
}

impl FieldName for ManufacturerChemicalFields {}

impl Display for ManufacturerChemicalFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManufacturerChemicalFields::ManufacturerNumber => write!(f, "Manufacturer Number")
        }
    }
}

impl FromStr for ManufacturerChemicalFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Manufacturer Number" => Ok(ManufacturerChemicalFields::ManufacturerNumber),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for ManufacturerChemical {

    type FieldNames = ManufacturerChemicalFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref manufacturer_number) = values[0] {
            Ok(ManufacturerChemical {
                manufacturer_number: manufacturer_number.clone()
            })
        } else {
            Err("Incorrect type manufacturer number. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![ManufacturerChemicalFields::ManufacturerNumber]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.manufacturer_number.clone())]
    }

    fn get_field(&self, field_name: ManufacturerChemicalFields) -> Option<Value> {
        match field_name {
            ManufacturerChemicalFields::ManufacturerNumber => Some(Value::String(self.manufacturer_number.clone()))
        }
    }
}

#[cfg(test)]
mod manufacturer_chemical_tests {

    use std::str::FromStr;

    use manufacturer_chemical::ManufacturerChemicalFields;
    use manufacturer_chemical::ManufacturerChemical;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_manufacturerchemicalfields_from_str() {
        let manufacturerchemical_field = ManufacturerChemicalFields::from_str("Manufacturer Number");
        assert_eq!(manufacturerchemical_field, Ok(ManufacturerChemicalFields::ManufacturerNumber));
    }

    #[test]
    fn test_manufacturerchemicalfields_from_fields() {
        let fields = [
            Value::String("916225746".to_string())
        ];

        let manufacturer_chemical = ManufacturerChemical::from_fields(&fields).unwrap();

        assert_eq!(manufacturer_chemical.manufacturer_number, "916225746".to_string());
    }

    #[test]
    fn test_manufacturerchemical_get_field_names() {
        let field_names = ManufacturerChemical::get_field_names();

        assert_eq!(field_names[0], ManufacturerChemicalFields::ManufacturerNumber);
    }

    #[test]
    fn test_manufacturerchemical_get_fields() {
        let manufacturer_chemical = ManufacturerChemical {
            manufacturer_number: "444".to_string()
        };

        let fields = manufacturer_chemical.get_fields();

        assert_eq!(fields[0], Value::String("444".to_string()));
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_manufacturerchemical_get_field() {
        let manufacturer_chemical = ManufacturerChemical {
            manufacturer_number: "1234".to_string()
        };

        let manufacturer_number = manufacturer_chemical.get_field(ManufacturerChemicalFields::ManufacturerNumber);

        assert_eq!(manufacturer_number, Some(Value::String("1234".to_string())));
    }
}