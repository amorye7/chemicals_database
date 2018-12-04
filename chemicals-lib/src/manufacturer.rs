use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct Manufacturer {
    pub company_name: String,
    pub address: String,
    pub phone_number: String,
    pub website: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ManufacturerFields {
    CompanyName,
    Address,
    PhoneNumber,
    Website
}

impl FieldName for ManufacturerFields {}

impl Display for ManufacturerFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManufacturerFields::CompanyName => write!(f, "Company Name"),
            ManufacturerFields::Address => write!(f, "Address"),
            ManufacturerFields::PhoneNumber => write!(f, "Phone Number"),
            ManufacturerFields::Website => write!(f, "Website")
        }
    }
}

impl FromStr for ManufacturerFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Company Name" => Ok(ManufacturerFields::CompanyName),
            "Address" => Ok(ManufacturerFields::Address),
            "Phone Number" => Ok(ManufacturerFields::PhoneNumber),
            "Website" => Ok(ManufacturerFields::Website),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for Manufacturer {

    type FieldNames = ManufacturerFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref company_name) = values[0] {
            if let Value::String(ref address) = values[1] {
                if let Value::String(ref phone_number) = values[2] {
                    if let Value::String(ref website) = values [3] {
                        Ok(Manufacturer {
                            company_name: company_name.clone(),
                            address: address.clone(),
                            phone_number: phone_number.clone(),
                            website: website.clone()
                        })
                    } else {
                        Err("Incorrect type for website. Should be String".to_string())
                    }
                } else {
                    Err("Incorrect type for phone number. Should be String".to_string())
                }
            } else {
                Err("Incorrect type for address Should be String".to_string())
            }
        } else {
            Err("Incorrect type for company name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![ManufacturerFields::CompanyName,
            ManufacturerFields::Address,
            ManufacturerFields::PhoneNumber,
            ManufacturerFields::Website]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.company_name.clone()),
            Value::String(self.address.clone()),
            Value::String(self.phone_number.clone()),
            Value::String(self.website.clone())]
    }

    fn get_field(&self, field_name: ManufacturerFields) -> Option<Value> {
        match field_name {
            ManufacturerFields::CompanyName => Some(Value::String(self.company_name.clone())),
            ManufacturerFields::Address => Some(Value::String(self.address.clone())),
            ManufacturerFields::PhoneNumber => Some(Value::String(self.phone_number.clone())),
            ManufacturerFields::Website => Some(Value::String(self.website.clone()))
        }
    }
}

#[cfg(test)]
mod manufacturer_test {

    use std::str::FromStr;

    use manufacturer::ManufacturerFields;
    use manufacturer::Manufacturer;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_manufacturerfields_from_str() {
        let companyname_field = ManufacturerFields::from_str("Company Name");
        assert_eq!(companyname_field, Ok(ManufacturerFields::CompanyName));

        let address_field = ManufacturerFields::from_str("Address");
        assert_eq!(address_field, Ok(ManufacturerFields::Address));

        let phonenumber_field = ManufacturerFields::from_str("Phone Number");
        assert_eq!(phonenumber_field, Ok(ManufacturerFields::PhoneNumber));

        let website_field = ManufacturerFields::from_str("Website");
        assert_eq!(website_field, Ok(ManufacturerFields::Website));
    }

    #[test]
    fn test_manufacturer_from_fields() {
        let fields = [
            Value::String("Science Labs".to_string()),
            Value::String("234 Math Ave".to_string()),
            Value::String("123-456-7890".to_string()),
            Value::String("sciencelab.org".to_string())
        ];

        let manufacturer = Manufacturer::from_fields(&fields).unwrap();

        assert_eq!(manufacturer.company_name, "Science Labs".to_string());
        assert_eq!(manufacturer.address, "234 Math Ave".to_string());
        assert_eq!(manufacturer.phone_number, "123-456-7890".to_string());
        assert_eq!(manufacturer.website, "sciencelab.org".to_string());
    }

    #[test]
    fn test_manufacturer_get_field_names() {
        let field_names = Manufacturer::get_field_names();

        assert_eq!(field_names[0], ManufacturerFields::CompanyName);
        assert_eq!(field_names[1], ManufacturerFields::Address);
        assert_eq!(field_names[2], ManufacturerFields::PhoneNumber);
        assert_eq!(field_names[3], ManufacturerFields::Website);
        assert_eq!(field_names.len(), 4);
    }

    #[test]
    fn test_manufacturer_get_fields() {
        let manufacturer = Manufacturer {
            company_name: "McMaster".to_string(),
            address: "111 McMaster Lane".to_string(),
            phone_number: "000-000-0000".to_string(),
            website: "mcmaster.org".to_string()
        };

        let fields = manufacturer.get_fields();

        assert_eq!(fields[0], Value::String("McMaster".to_string()));
        assert_eq!(fields[1], Value::String("111 McMaster Lane".to_string()));
        assert_eq!(fields[2], Value::String("000-000-0000".to_string()));
        assert_eq!(fields[3], Value::String("mcmaster.org".to_string()));
        assert_eq!(fields.len(), 4);
    }

    #[test]
    fn test_manufacturer_get_field() {
        let manufacturer = Manufacturer {
            company_name: "Amazon".to_string(),
            address: "Seattle".to_string(),
            phone_number: "732-215-1234".to_string(),
            website: "amazon.com".to_string()
        };

        let company_name = manufacturer.get_field(ManufacturerFields::CompanyName);

        assert_eq!(company_name, Some(Value::String("Amazon".to_string())));
    }
}