use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Component {
    pub chemical_name: String,
    pub common_name: String,
    pub cas_number: String,
    pub substance_number: String,
    pub trade_secret_number: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ComponentFields {
    ChemicalName,
    CommonName,
    CasNumber,
    SubstanceNumber,
    TradeSecretNumber
}

impl FieldName for ComponentFields {}

impl Display for ComponentFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentFields::ChemicalName => write!(f, "Chemical Name"),
            ComponentFields::CommonName => write!(f, "Common Name"),
            ComponentFields::CasNumber => write!(f, "CAS Number"),
            ComponentFields::SubstanceNumber => write!(f, "Substance Number"),
            ComponentFields::TradeSecretNumber => write!(f, "Trade Secret Number")
        }
    }
}

impl FromStr for ComponentFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Chemical Name" => Ok(ComponentFields::ChemicalName),
            "Common Name" => Ok(ComponentFields::CommonName),
            "CAS Number" => Ok(ComponentFields::CasNumber),
            "Substance Number" => Ok(ComponentFields::SubstanceNumber),
            "Trade Secret Number" => Ok(ComponentFields::TradeSecretNumber),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for Component {

    type FieldNames = ComponentFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref chemical_name) = values[0] {
            if let Value::String(ref common_name) = values[1] {
                if let Value::String(ref cas_number) = values[2] {
                    if let Value::String(ref substance_number) = values[3] {
                        if let Value::String(ref trade_secret_number) = values[4] {
                            Ok(Component {
                                chemical_name: chemical_name.clone(),
                                common_name: common_name.clone(),
                                cas_number: cas_number.clone(),
                                substance_number: substance_number.clone(),
                                trade_secret_number: trade_secret_number.clone()
                            })
                        } else {
                            Err("Incorrect type for trade secret number. Should be String".to_string())
                        }
                    } else {
                        Err("Incorrect type for subtance number. Should be String".to_string())
                    }
                } else {
                    Err("Incorrect type for CAS number. Should be String".to_string())
                }
            } else {
                Err("Incorrect type for common name. Should be String".to_string())
            }
        } else {
            Err("Incorrect type for chemical name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![ComponentFields::ChemicalName,
            ComponentFields::CommonName,
            ComponentFields::CasNumber,
            ComponentFields::SubstanceNumber,
            ComponentFields::TradeSecretNumber]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.chemical_name.clone()),
            Value::String(self.common_name.clone()),
            Value::String(self.cas_number.clone()),
            Value::String(self.substance_number.clone()),
            Value::String(self.trade_secret_number.clone())]
    }

    fn get_field(&self, field_name: ComponentFields) -> Option<Value> {
        match field_name {
            ComponentFields::ChemicalName => Some(Value::String(self.chemical_name.clone())),
            ComponentFields::CommonName => Some(Value::String(self.common_name.clone())),
            ComponentFields::CasNumber => Some(Value::String(self.cas_number.clone())),
            ComponentFields::SubstanceNumber => Some(Value::String(self.substance_number.clone())),
            ComponentFields::TradeSecretNumber => Some(Value::String(self.trade_secret_number.clone()))
        }
    }
}

#[cfg(test)]
mod component_test {

    use std::str::FromStr;

    use component::ComponentFields;
    use component::Component;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_componentfields_from_str() {
        let component_field = ComponentFields::from_str("Chemical Name");
        assert_eq!(component_field, Ok(ComponentFields::ChemicalName));

        let component_field = ComponentFields::from_str("Common Name");
        assert_eq!(component_field, Ok(ComponentFields::CommonName));

        let component_field = ComponentFields::from_str("CAS Number");
        assert_eq!(component_field, Ok(ComponentFields::CasNumber));

        let component_field = ComponentFields::from_str("Substance Number");
        assert_eq!(component_field, Ok(ComponentFields::SubstanceNumber));

        let component_field = ComponentFields::from_str("Trade Secret Number");
        assert_eq!(component_field, Ok(ComponentFields::TradeSecretNumber));
    }

    #[test]
    fn test_component_from_fields() {
        let fields = [
            Value::String("Strong Bond Epoxy".to_string()),
            Value::String("Epoxy".to_string()),
            Value::String("199-92-2995".to_string()),
            Value::String("43".to_string()),
            Value::String("1".to_string())
        ];

        let component = Component::from_fields(&fields).unwrap();

        assert_eq!(component.chemical_name, "Strong Bond Epoxy".to_string());
        assert_eq!(component.common_name, "Epoxy".to_string());
        assert_eq!(component.cas_number, "199-92-2995".to_string());
        assert_eq!(component.substance_number, "43".to_string());
        assert_eq!(component.trade_secret_number, "1".to_string());
    }    

    #[test]
    fn test_component_get_field_names() {
        let field_names = Component::get_field_names();

        assert_eq!(field_names[0], ComponentFields::ChemicalName);
        assert_eq!(field_names[1], ComponentFields::CommonName);
        assert_eq!(field_names[2], ComponentFields::CasNumber);
        assert_eq!(field_names[3], ComponentFields::SubstanceNumber);
        assert_eq!(field_names[4], ComponentFields::TradeSecretNumber);
    }

    #[test]
    fn test_component_get_fields() {
        let component = Component {
            chemical_name: "Cleaning Isopropyl Alcohol".to_string(),
            common_name: "IPA".to_string(),
            cas_number: "44-32-192".to_string(),
            substance_number: "22".to_string(),
            trade_secret_number: "2".to_string()
        };

        let fields = component.get_fields();

        assert_eq!(fields[0], Value::String("Cleaning Isopropyl Alcohol".to_string()));
        assert_eq!(fields[1], Value::String("IPA".to_string()));
        assert_eq!(fields[2], Value::String("44-32-192".to_string()));
        assert_eq!(fields[3], Value::String("22".to_string()));
        assert_eq!(fields[4], Value::String("2".to_string()));
        assert_eq!(fields.len(), 5);
    }

    #[test]
    fn test_component_get_field() {
        let component = Component {
            chemical_name: "TetraAcrylate".to_string(),
            common_name: "Oligmer".to_string(),
            cas_number: "000-00".to_string(),
            substance_number: "4".to_string(),
            trade_secret_number: "33".to_string()
        };

        let chemical_name = component.get_field(ComponentFields::ChemicalName);

        assert_eq!(chemical_name, Some(Value::String("TetraAcrylate".to_string())));
    }
}