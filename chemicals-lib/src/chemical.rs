use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Chemical {
    pub chemical_name: String, 
    pub purpose: String,
    pub state_of_matter: String, //change type NO PLASMA
    pub msds_sds_path: String,
    pub qr_code: String, //change type
    pub opened_life_span: String,
    pub unopened_life_span: String,
    pub controlled_substance: bool,
    pub restricted_substance: bool,
    pub petroleum_base: bool,
    pub signal_word: String, //leave for new type
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChemicalFields {
    ChemicalName,
    Purpose,
    StateOfMatter,
    MsdsSdsPath,
    QrCode,
    OpenedLifeSpan,
    UnopenedLifeSpan,
    ControlledSubstance,
    RestrictedSubstance,
    PetroleumBase,
    SignalWord
}

impl FieldName for ChemicalFields {}

impl Display for ChemicalFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChemicalFields::ChemicalName => write!(f, "Chemical Name"),
            ChemicalFields::Purpose => write!(f, "Purpose"),
            ChemicalFields::StateOfMatter => write!(f, "State of Matter"),
            ChemicalFields::MsdsSdsPath => write!(f, "MSDS/SDS Path"),
            ChemicalFields::QrCode => write!(f, "QR Code"),
            ChemicalFields::OpenedLifeSpan => write!(f, "Opened Life Span"),
            ChemicalFields::UnopenedLifeSpan => write!(f, "Unopened Life Span"),
            ChemicalFields::ControlledSubstance => write!(f, "Controlled Substance"),
            ChemicalFields::RestrictedSubstance => write!(f, "Restricted Substance"),
            ChemicalFields::PetroleumBase => write!(f, "Petroleum Base"),
            ChemicalFields::SignalWord => write!(f, "Signal Word")
        }
    }
}

impl FromStr for ChemicalFields {
    type Err =String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Chemical Name" => Ok(ChemicalFields::ChemicalName),
            "Purpose" => Ok(ChemicalFields::Purpose),
            "State Of Matter" => Ok(ChemicalFields::StateOfMatter),
            "MSDS/SDS Path" => Ok(ChemicalFields::MsdsSdsPath),
            "QR Code" => Ok(ChemicalFields::QrCode),
            "Opened Life Span" => Ok(ChemicalFields::OpenedLifeSpan),
            "Unopened Life Span" => Ok(ChemicalFields::UnopenedLifeSpan),
            "Controlled Substance" => Ok(ChemicalFields::ControlledSubstance),
            "Restricted Substance" => Ok(ChemicalFields::RestrictedSubstance),
            "Petroleum Base" => Ok(ChemicalFields::PetroleumBase),
            "Signal Word" => Ok(ChemicalFields::SignalWord),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for Chemical {

    type FieldNames = ChemicalFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref chemical_name) = values[0] {
            if let Value::String(ref purpose) = values[1] {
                if let Value::String(ref state_of_matter) = values[2] {
                    if let Value::String(ref msds_sds_path) = values[3] {
                        if let Value::String(ref qr_code) = values[4] {
                            if let Value::String(ref opened_life_span) = values[5] {
                                if let Value::String(ref unopened_life_span) = values[6] {
                                    if let Value::Boolean(ref controlled_substance) = values[7] {
                                        if let Value::Boolean(ref restricted_substance) = values[8] {
                                            if let Value::Boolean(ref petroleum_base) = values[9] {
                                                if let Value::String(ref signal_word) = values[10] {
                                                    Ok(Chemical {
                                                        chemical_name: chemical_name.clone(),
                                                        purpose: purpose.clone(),
                                                        state_of_matter: state_of_matter.clone(),
                                                        msds_sds_path: msds_sds_path.clone(),
                                                        qr_code: qr_code.clone(),
                                                        opened_life_span: opened_life_span.clone(),
                                                        unopened_life_span: unopened_life_span.clone(),
                                                        controlled_substance: controlled_substance.clone(),
                                                        restricted_substance: restricted_substance.clone(),
                                                        petroleum_base: petroleum_base.clone(),
                                                        signal_word: signal_word.clone()
                                                    })
                                                } else {
                                                    Err("Incorrect type for signal word. Should be String".to_string())
                                                }
                                            } else {
                                                Err("Incorrect type for petroleum base. Should be boolean".to_string())
                                            }
                                        } else {
                                            Err("Incorrect type for restricted substance. Should be boolean".to_string())
                                        }
                                    } else {
                                        Err("Incorrect type for controlled substance. Should be boolean".to_string())
                                    }
                                } else {
                                    Err("Incorrect type for unopened life span. Should be String".to_string())
                                }
                            } else {
                                Err("Incorrect type for opened life span. Should be String".to_string())
                            }
                        } else {
                            Err("Incorrect type for qr code. Should be String".to_string())
                        }
                    } else {
                        Err("Incorrect type for msds/sds path. Should be String".to_string())
                    }
                } else {
                    Err("Incorrect type for state of matter. Should be String".to_string())
                }
            } else {
                Err("Incorrect type for purpose. Should be String".to_string())
            }
        } else {
            Err("Incorrect type of chemical name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![ChemicalFields::ChemicalName,
            ChemicalFields::Purpose,
            ChemicalFields::StateOfMatter,
            ChemicalFields::MsdsSdsPath,
            ChemicalFields::QrCode,
            ChemicalFields::OpenedLifeSpan,
            ChemicalFields::UnopenedLifeSpan,
            ChemicalFields::ControlledSubstance,
            ChemicalFields::RestrictedSubstance,
            ChemicalFields::PetroleumBase,
            ChemicalFields::SignalWord]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.chemical_name.clone()),
            Value::String(self.purpose.clone()),
            Value::String(self.state_of_matter.clone()),
            Value::String(self.msds_sds_path.clone()),
            Value::String(self.qr_code.clone()),
            Value::String(self.opened_life_span.clone()),
            Value::String(self.unopened_life_span.clone()),
            Value::Boolean(self.controlled_substance.clone()),
            Value::Boolean(self.restricted_substance.clone()),
            Value::Boolean(self.petroleum_base.clone()),
            Value::String(self.signal_word.clone())]
    }

    fn get_field(&self, field_name: ChemicalFields) -> Option<Value> {
        match field_name {
            ChemicalFields::ChemicalName => Some(Value::String(self.chemical_name.clone())),
            ChemicalFields::Purpose => Some(Value::String(self.purpose.clone())),
            ChemicalFields::StateOfMatter => Some(Value::String(self.state_of_matter.clone())),
            ChemicalFields::MsdsSdsPath => Some(Value::String(self.msds_sds_path.clone())),
            ChemicalFields::QrCode => Some(Value::String(self.qr_code.clone())),
            ChemicalFields::OpenedLifeSpan => Some(Value::String(self.opened_life_span.clone())),
            ChemicalFields::UnopenedLifeSpan => Some(Value::String(self.unopened_life_span.clone())),
            ChemicalFields::ControlledSubstance => Some(Value::Boolean(self.controlled_substance.clone())),
            ChemicalFields::RestrictedSubstance => Some(Value::Boolean(self.restricted_substance.clone())),
            ChemicalFields::PetroleumBase => Some(Value::Boolean(self.petroleum_base.clone())),
            ChemicalFields::SignalWord => Some(Value::String(self.signal_word.clone()))
        }
    }
}

#[cfg(test)]
mod chemical_tests {

    use std::str::FromStr;

    use chemical::ChemicalFields;
    use chemical::Chemical;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_chemicalfields_from_str() {
        let chemicalname_field = ChemicalFields::from_str("Chemical Name");
        assert_eq!(chemicalname_field, Ok(ChemicalFields::ChemicalName));

        let purpose_field = ChemicalFields::from_str("Purpose");
        assert_eq!(purpose_field, Ok(ChemicalFields::Purpose));

        let stateofmatter_field = ChemicalFields::from_str("State Of Matter");
        assert_eq!(stateofmatter_field, Ok(ChemicalFields::StateOfMatter));

        let msdssdspath_field = ChemicalFields::from_str("MSDS/SDS Path");
        assert_eq!(msdssdspath_field, Ok(ChemicalFields::MsdsSdsPath));

        let qrcode_field = ChemicalFields::from_str("QR Code");
        assert_eq!(qrcode_field, Ok(ChemicalFields::QrCode));

        let openedlifespan_field = ChemicalFields::from_str("Opened Life Span");
        assert_eq!(openedlifespan_field, Ok(ChemicalFields::OpenedLifeSpan));

        let unopenedlifespan_field = ChemicalFields::from_str("Unopened Life Span");
        assert_eq!(unopenedlifespan_field, Ok(ChemicalFields::UnopenedLifeSpan));

        let controlledsubstance_field = ChemicalFields::from_str("Controlled Substance");
        assert_eq!(controlledsubstance_field, Ok(ChemicalFields::ControlledSubstance));

        let restrictedsubstance_field = ChemicalFields::from_str("Restricted Substance");
        assert_eq!(restrictedsubstance_field, Ok(ChemicalFields::RestrictedSubstance));

        let petroleumbase_field = ChemicalFields::from_str("Petroleum Base");
        assert_eq!(petroleumbase_field, Ok(ChemicalFields::PetroleumBase));

        let signalword_field = ChemicalFields::from_str("Signal Word");
        assert_eq!(signalword_field, Ok(ChemicalFields::SignalWord));
    }

    #[test]
    fn test_chemical_from_fields() {
        let fields = [
            Value::String("Isopropyl Alcohol".to_string()),
            Value::String("Cleaning".to_string()),
            Value::String("Liquid".to_string()),
            Value::String("Isopropyl Alcohol MSDS".to_string()),
            Value::String("124".to_string()),
            Value::String("20 years".to_string()),
            Value::String("10 years".to_string()),
            Value::Boolean(false),
            Value::Boolean(false),
            Value::Boolean(false),
            Value::String("Warning".to_string())
        ];

        let chemical = Chemical::from_fields(&fields).unwrap();

        assert_eq!(chemical.chemical_name, "Isopropyl Alcohol".to_string());
        assert_eq!(chemical.purpose, "Cleaning".to_string());
        assert_eq!(chemical.state_of_matter, "Liquid".to_string());
        assert_eq!(chemical.msds_sds_path, "Isopropyl Alcohol MSDS".to_string());
        assert_eq!(chemical.qr_code, "124".to_string());
        assert_eq!(chemical.opened_life_span, "20 years".to_string());
        assert_eq!(chemical.unopened_life_span, "10 years".to_string());
        assert_eq!(chemical.controlled_substance, false);
        assert_eq!(chemical.restricted_substance, false);
        assert_eq!(chemical.petroleum_base, false);
        assert_eq!(chemical.signal_word, "Warning".to_string());
    }

    #[test]
    fn test_chemical_get_field_names() {
        let field_names = Chemical::get_field_names();

        assert_eq!(field_names[0], ChemicalFields::ChemicalName);
        assert_eq!(field_names[1], ChemicalFields::Purpose);
        assert_eq!(field_names[2], ChemicalFields::StateOfMatter);
        assert_eq!(field_names[3], ChemicalFields::MsdsSdsPath);
        assert_eq!(field_names[4], ChemicalFields::QrCode);
        assert_eq!(field_names[5], ChemicalFields::OpenedLifeSpan);
        assert_eq!(field_names[6], ChemicalFields::UnopenedLifeSpan);
        assert_eq!(field_names[7], ChemicalFields::ControlledSubstance);
        assert_eq!(field_names[8], ChemicalFields::RestrictedSubstance);
        assert_eq!(field_names[9], ChemicalFields::PetroleumBase);
        assert_eq!(field_names[10], ChemicalFields::SignalWord);
        assert_eq!(field_names.len(), 11);
    }

    #[test]
    fn test_chemical_get_fields() {
        let chemical = Chemical {
            chemical_name: "Epoxy".to_string(),
            purpose: "Adhesive".to_string(),
            state_of_matter: "Solid".to_string(),
            msds_sds_path: "Epoxy MSDS".to_string(),
            qr_code: "91622".to_string(),
            opened_life_span: "5 years".to_string(),
            unopened_life_span: "10 years".to_string(),
            controlled_substance: false,
            restricted_substance: false,
            petroleum_base: false,
            signal_word: "Warning".to_string()
        };

        let fields = chemical.get_fields();

        assert_eq!(fields[0], Value::String("Epoxy".to_string()));
        assert_eq!(fields[1], Value::String("Adhesive".to_string()));
        assert_eq!(fields[2], Value::String("Solid".to_string()));
        assert_eq!(fields[3], Value::String("Epoxy MSDS".to_string()));
        assert_eq!(fields[4], Value::String("91622".to_string()));
        assert_eq!(fields[5], Value::String("5 years".to_string()));
        assert_eq!(fields[6], Value::String("10 years".to_string()));
        assert_eq!(fields[7], Value::Boolean(false));
        assert_eq!(fields[8], Value::Boolean(false));
        assert_eq!(fields[9], Value::Boolean(false));
        assert_eq!(fields[10], Value::String("Warning".to_string()));
        assert_eq!(fields.len(), 11);
    }

    #[test]
    fn test_chemical_get_field() {
        let chemical = Chemical {
            chemical_name: "Cyanoacrylate".to_string(),
            purpose: "Adhesive".to_string(),
            state_of_matter: "Solid".to_string(),
            msds_sds_path: "Cyanoacrylate".to_string(),
            qr_code: "5746".to_string(),
            opened_life_span: "15 years".to_string(),
            unopened_life_span: "15 days".to_string(),
            controlled_substance: true,
            restricted_substance: true,
            petroleum_base: true,
            signal_word: "Danger".to_string()
        };

        let chemical_name = chemical.get_field(ChemicalFields::ChemicalName);

        assert_eq!(chemical_name, Some(Value::String("Cyanoacrylate".to_string())));
    }
}