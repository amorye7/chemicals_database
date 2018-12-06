use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Pictogram {
    pub picture_name: String,
    pub pictogram_path: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PictogramFields {
    PictureName,
    PictogramPath
}

impl FieldName for PictogramFields {}

impl Display for PictogramFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PictogramFields::PictureName => write!(f, "PictureName"),
            PictogramFields::PictogramPath => write!(f, "Pictogram Path")
        }
    }
}

impl FromStr for PictogramFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Picture Name" => Ok(PictogramFields::PictureName),
            "Pictogram Path" => Ok(PictogramFields::PictogramPath),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for Pictogram {

    type FieldNames = PictogramFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref picture_name) = values[0] {
            if let Value::String(ref pictogram_path) = values[1] {
                Ok(Pictogram {
                    picture_name: picture_name.clone(),
                    pictogram_path: pictogram_path.clone()
                })
            } else {
                Err("Incorrect type for pictogram path. Should be String".to_string())
            }
        } else {
            Err("Incorrect type for picture name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![PictogramFields::PictureName,
            PictogramFields::PictogramPath]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.picture_name.clone()),
            Value::String(self.pictogram_path.clone())]
    }

    fn get_field(&self, field_name: PictogramFields) -> Option<Value> {
        match field_name {
            PictogramFields::PictureName => Some(Value::String(self.picture_name.clone())),
            PictogramFields::PictogramPath => Some(Value::String(self.pictogram_path.clone()))
        }
    }
}

#[cfg(test)]
mod pictogram_test {

    use std::str::FromStr;

    use pictogram::PictogramFields;
    use pictogram::Pictogram;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_pictogramfields_from_str() {
        let pictogram_field = PictogramFields::from_str("Picture Name");
        assert_eq!(pictogram_field, Ok(PictogramFields::PictureName));

        let pictogram_field = PictogramFields::from_str("Pictogram Path");
        assert_eq!(pictogram_field, Ok(PictogramFields::PictogramPath));
    }

    #[test]
    fn test_pictogram_from_fields() {
        let fields = [
            Value::String("Environmental Hazard".to_string()),
            Value::String("9".to_string())
        ];

        let pictogram = Pictogram::from_fields(&fields).unwrap();

        assert_eq!(pictogram.picture_name, "Environmental Hazard".to_string());
        assert_eq!(pictogram.pictogram_path, "9".to_string());
    }

    #[test]
    fn test_pictogram_get_field_names() {
        let field_names = Pictogram::get_field_names();

        assert_eq!(field_names[0], PictogramFields::PictureName);
        assert_eq!(field_names[1], PictogramFields::PictogramPath);
    }

    #[test]
    fn test_pictogram_get_fields() {
        let pictogram = Pictogram {
            picture_name: "Health Hazard".to_string(),
            pictogram_path: "8".to_string()
        };

        let fields = pictogram.get_fields();

        assert_eq!(fields[0], Value::String("Health Hazard".to_string()));
        assert_eq!(fields[1], Value::String("8".to_string()));
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_pictogram_get_field() {
        let pictogram = Pictogram {
            picture_name: "Toxic".to_string(),
            pictogram_path: "6".to_string()
        };

        let picture_name = pictogram.get_field(PictogramFields::PictureName);

        assert_eq!(picture_name, Some(Value::String("Toxic".to_string())));
    }
}