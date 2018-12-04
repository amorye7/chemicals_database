use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use database_lib::interface::Entry;
use database_lib::interface::Value;
use database_lib::interface::FieldName;

#[derive(Debug, Clone)]
pub struct ChemicalInventory {
    pub lot_number: String,
    pub purchase_date: String,
    pub arrival_date: String,
    pub open_date: String,
    pub expiration_date: String,
    pub disposal_date: String, //internal waste drop-off
    pub removal_date: String, //taken off campus
    pub disposal_method: String,
    pub active: bool,
    pub container_type: String,
    pub container_size: String,
    pub unit: String,
    pub percent_remaining: String
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChemicalInventoryFields {
    LotNumber,
    PurchaseDate,
    ArrivalDate,
    OpenDate,
    ExpirationDate,
    DisposalDate,
    RemovalDate,
    DisposalMethod,
    Active,
    ContainerType,
    ContainerSize,
    Unit,
    PercentRemaining
}

impl FieldName for ChemicalInventoryFields {}

impl Display for ChemicalInventoryFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChemicalInventoryFields::LotNumber => write!(f, "Lot Number"),
            ChemicalInventoryFields::PurchaseDate => write!(f, "Purchase Date"),
            ChemicalInventoryFields::ArrivalDate => write!(f, "Arrival Date"),
            ChemicalInventoryFields::OpenDate => write!(f, "Open Date"),
            ChemicalInventoryFields::ExpirationDate => write!(f, "Expiration Date"),
            ChemicalInventoryFields::DisposalDate => write!(f, "Disposal Date"),
            ChemicalInventoryFields::RemovalDate => write!(f, "Removal Date"),
            ChemicalInventoryFields::DisposalMethod => write!(f, "Active"),
            ChemicalInventoryFields::Active => write!(f, "Active"),
            ChemicalInventoryFields::ContainerType => write!(f, "Container Type"),
            ChemicalInventoryFields::ContainerSize => write!(f, "Container Size"),
            ChemicalInventoryFields::Unit => write!(f, "Unit"),
            ChemicalInventoryFields::PercentRemaining => write!(f, "Percent Remaining")
        }
    }
}

impl FromStr for ChemicalInventoryFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lot Number" => Ok(ChemicalInventoryFields::LotNumber),
            "Purchase Date" => Ok(ChemicalInventoryFields::PurchaseDate),
            "Arrival Date" => Ok(ChemicalInventoryFields::ArrivalDate),
            "Open Date" => Ok(ChemicalInventoryFields::OpenDate),
            "Expiration Date" => Ok(ChemicalInventoryFields::ExpirationDate),
            "Disposal Date" => Ok(ChemicalInventoryFields::DisposalDate),
            "Removal Date" => Ok(ChemicalInventoryFields::RemovalDate),
            "Disposal Method" => Ok(ChemicalInventoryFields::DisposalMethod),
            "Active" => Ok(ChemicalInventoryFields::Active),
            "Container Type" => Ok(ChemicalInventoryFields::ContainerType),
            "Container Size" => Ok(ChemicalInventoryFields::ContainerSize),
            "Unit" => Ok(ChemicalInventoryFields::Unit),
            "Percent Remaining" => Ok(ChemicalInventoryFields::PercentRemaining),
            _=> Err("Field does not exist".to_string()),
        }
    }
}

impl Entry for ChemicalInventory {

    type FieldNames = ChemicalInventoryFields;

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref lot_number) = values[0] {
            if let Value::String(ref purchase_date) = values[1] {
                if let Value::String(ref arrival_date) = values[2] {
                    if let Value::String(ref open_date) = values[3] {
                        if let Value::String(ref expiration_date) = values[4] {
                            if let Value::String(ref disposal_date) = values[5] {
                                if let Value::String(ref removal_date) = values[6] {
                                    if let Value::String(ref disposal_method) = values[7] {
                                        if let Value::Boolean(ref active) = values[8] {
                                            if let Value::String(ref container_type) = values[9] {
                                                if let Value::String(ref container_size) = values[10] {
                                                    if let Value::String(ref unit) = values[11] {
                                                        if let Value::String(ref percent_remaining) = values[12] {
                                                            Ok(ChemicalInventory {
                                                                lot_number: lot_number.clone(),
                                                                purchase_date: purchase_date.clone(),
                                                                arrival_date: arrival_date.clone(),
                                                                open_date: open_date.clone(),
                                                                expiration_date: expiration_date.clone(),
                                                                disposal_date: disposal_date.clone(),
                                                                removal_date: removal_date.clone(),
                                                                disposal_method: disposal_method.clone(),
                                                                active: active.clone(),
                                                                container_type: container_type.clone(),
                                                                container_size: container_size.clone(),
                                                                unit: unit.clone(),
                                                                percent_remaining: percent_remaining.clone()
                                                            })
                                                        } else {
                                                            Err("Incorrect type for percent remaining. Should be String".to_string())
                                                        }
                                                    } else {
                                                        Err("Incorrect type for unit. Should be String".to_string())
                                                    }
                                                } else {
                                                    Err("Incorrect type for container size. Should be String".to_string())
                                                }
                                            } else {
                                                Err("Incorrect type for container type. Should be String".to_string())
                                            }
                                        } else {
                                            Err("Incorrect type for active. Should be boolean".to_string())
                                        }
                                    } else {
                                        Err("Incorrect type for disposal method. Shoud be String".to_string())
                                    }
                                } else {
                                    Err("Incorrect type for removal date. Should be String".to_string())
                                }
                            } else {
                                Err("Incorrect type for disposal date. Should be String".to_string())
                            }
                        } else {
                            Err("Incorrect type for expiration date. Should be String".to_string())
                        }
                    } else {
                        Err("Incorrect type for open date. Should be String".to_string())
                    }
                } else {
                    Err("Incorrect type for arrival date. Should be String".to_string())
                }
            } else {
                Err("Incorrect type for purchase date. Should be String".to_string())
            }
        } else {
            Err("Incorrect type for lot number. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![ChemicalInventoryFields::LotNumber,
            ChemicalInventoryFields::PurchaseDate,
            ChemicalInventoryFields::ArrivalDate,
            ChemicalInventoryFields::OpenDate,
            ChemicalInventoryFields::ExpirationDate,
            ChemicalInventoryFields::DisposalDate,
            ChemicalInventoryFields::RemovalDate,
            ChemicalInventoryFields::DisposalMethod,
            ChemicalInventoryFields::Active,
            ChemicalInventoryFields::ContainerType,
            ChemicalInventoryFields::ContainerSize,
            ChemicalInventoryFields::Unit,
            ChemicalInventoryFields::PercentRemaining]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.lot_number.clone()),
            Value::String(self.purchase_date.clone()),
            Value::String(self.arrival_date.clone()),
            Value::String(self.open_date.clone()),
            Value::String(self.expiration_date.clone()),
            Value::String(self.disposal_date.clone()),
            Value::String(self.removal_date.clone()),
            Value::String(self.disposal_method.clone()),
            Value::Boolean(self.active.clone()),
            Value::String(self.container_type.clone()),
            Value::String(self.container_size.clone()),
            Value::String(self.unit.clone()),
            Value::String(self.percent_remaining.clone())]
    }

    fn get_field(&self, field_name: ChemicalInventoryFields) -> Option<Value> {
        match field_name {
            ChemicalInventoryFields::LotNumber => Some(Value::String(self.lot_number.clone())),
            ChemicalInventoryFields::PurchaseDate => Some(Value::String(self.purchase_date.clone())),
            ChemicalInventoryFields::ArrivalDate => Some(Value::String(self.arrival_date.clone())),
            ChemicalInventoryFields::OpenDate => Some(Value::String(self.open_date.clone())),
            ChemicalInventoryFields::ExpirationDate => Some(Value::String(self.expiration_date.clone())),
            ChemicalInventoryFields::DisposalDate => Some(Value::String(self.disposal_date.clone())),
            ChemicalInventoryFields::RemovalDate => Some(Value::String(self.removal_date.clone())),
            ChemicalInventoryFields::DisposalMethod => Some(Value::String(self.disposal_method.clone())),
            ChemicalInventoryFields::Active => Some(Value::Boolean(self.active.clone())),
            ChemicalInventoryFields::ContainerType => Some(Value::String(self.container_type.clone())),
            ChemicalInventoryFields::ContainerSize => Some(Value::String(self.container_size.clone())),
            ChemicalInventoryFields::Unit => Some(Value::String(self.unit.clone())),
            ChemicalInventoryFields::PercentRemaining => Some(Value::String(self.percent_remaining.clone()))
        }
    }
}

#[cfg(test)]
mod chemical_inventory_tests {

    use std::str::FromStr;

    use chemical_inventory::ChemicalInventoryFields;
    use chemical_inventory::ChemicalInventory;

    use database_lib::interface::Entry;
    use database_lib::interface::Value;

    #[test]
    fn test_chemicalinventoryfields_from_str() {
        let lotnumber_field = ChemicalInventoryFields::from_str("Lot Number");
        assert_eq!(lotnumber_field, Ok(ChemicalInventoryFields::LotNumber));

        let purchasedate_field = ChemicalInventoryFields::from_str("Purchase Date");
        assert_eq!(purchasedate_field, Ok(ChemicalInventoryFields::PurchaseDate));

        let arrivaldate_field = ChemicalInventoryFields::from_str("Arrival Date");
        assert_eq!(arrivaldate_field, Ok(ChemicalInventoryFields::ArrivalDate));

        let opendate_field = ChemicalInventoryFields::from_str("Open Date");
        assert_eq!(opendate_field, Ok(ChemicalInventoryFields::OpenDate));

        let expirationdate_field = ChemicalInventoryFields::from_str("Expiration Date");
        assert_eq!(expirationdate_field, Ok(ChemicalInventoryFields::ExpirationDate));

        let disposaldate_field = ChemicalInventoryFields::from_str("Disposal Date");
        assert_eq!(disposaldate_field, Ok(ChemicalInventoryFields::DisposalDate));

        let removaldate_field = ChemicalInventoryFields::from_str("Removal Date");
        assert_eq!(removaldate_field, Ok(ChemicalInventoryFields::RemovalDate));

        let disposalmethod_field = ChemicalInventoryFields::from_str("Disposal Method");
        assert_eq!(disposalmethod_field, Ok(ChemicalInventoryFields::DisposalMethod));

        let active_field = ChemicalInventoryFields::from_str("Active");
        assert_eq!(active_field, Ok(ChemicalInventoryFields::Active));

        let containertype_field = ChemicalInventoryFields::from_str("Container Type");
        assert_eq!(containertype_field, Ok(ChemicalInventoryFields::ContainerType));

        let containersize_field = ChemicalInventoryFields::from_str("Container Size");
        assert_eq!(containersize_field, Ok(ChemicalInventoryFields::ContainerSize));

        let unit_field = ChemicalInventoryFields::from_str("Unit");
        assert_eq!(unit_field, Ok(ChemicalInventoryFields::Unit));

        let percentremaining_field = ChemicalInventoryFields::from_str("Percent Remaining");
        assert_eq!(percentremaining_field, Ok(ChemicalInventoryFields::PercentRemaining));
    }

    #[test]
    fn test_chemicalinventory_from_fields() {
        let fields = [
            Value::String("12".to_string()),
            Value::String("12/3/2018".to_string()),
            Value::String("11/2/2018".to_string()),
            Value::String("3/4/2019".to_string()),
            Value::String("3/4/2024".to_string()),
            Value::String("12/4/2018".to_string()),
            Value::String("1/4/2019".to_string()),
            Value::String("With Gloves".to_string()),
            Value::Boolean(true),
            Value::String("Plastic Bottle".to_string()),
            Value::String("3".to_string()),
            Value::String("Gallon".to_string()),
            Value::String("50%".to_string())
        ];

        let chemical_inventory = ChemicalInventory::from_fields(&fields).unwrap();

        assert_eq!(chemical_inventory.lot_number, "12".to_string());
        assert_eq!(chemical_inventory.purchase_date, "12/3/2018".to_string());
        assert_eq!(chemical_inventory.arrival_date, "11/2/2018".to_string());
        assert_eq!(chemical_inventory.open_date, "3/4/2019".to_string());
        assert_eq!(chemical_inventory.expiration_date, "3/4/2024".to_string());
        assert_eq!(chemical_inventory.disposal_date, "12/4/2018".to_string());
        assert_eq!(chemical_inventory.removal_date, "1/4/2019".to_string());
        assert_eq!(chemical_inventory.disposal_method, "With Gloves".to_string());
        assert_eq!(chemical_inventory.active, true);
        assert_eq!(chemical_inventory.container_type, "Plastic Bottle".to_string());
        assert_eq!(chemical_inventory.container_size, "3".to_string());
        assert_eq!(chemical_inventory.unit, "Gallon".to_string());
        assert_eq!(chemical_inventory.percent_remaining, "50%".to_string());
    }

    #[test]
    fn test_chemicalinventory_get_field_names() {
        let field_names = ChemicalInventory::get_field_names();

        assert_eq!(field_names[0], ChemicalInventoryFields::LotNumber);
        assert_eq!(field_names[1], ChemicalInventoryFields::PurchaseDate);
        assert_eq!(field_names[2], ChemicalInventoryFields::ArrivalDate);
        assert_eq!(field_names[3], ChemicalInventoryFields::OpenDate);
        assert_eq!(field_names[4], ChemicalInventoryFields::ExpirationDate);
        assert_eq!(field_names[5], ChemicalInventoryFields::DisposalDate);
        assert_eq!(field_names[6], ChemicalInventoryFields::RemovalDate);
        assert_eq!(field_names[7], ChemicalInventoryFields::DisposalMethod);
        assert_eq!(field_names[8], ChemicalInventoryFields::Active);
        assert_eq!(field_names[9], ChemicalInventoryFields::ContainerType);
        assert_eq!(field_names[10], ChemicalInventoryFields::ContainerSize);
        assert_eq!(field_names[11], ChemicalInventoryFields::Unit);
        assert_eq!(field_names[12], ChemicalInventoryFields::PercentRemaining);
        assert_eq!(field_names.len(), 13);
    }

    #[test]
    fn test_chemicalinventory_get_fields() {
        let chemical_inventory = ChemicalInventory {
            lot_number: "1".to_string(),
            purchase_date: "2".to_string(),
            arrival_date: "3".to_string(),
            open_date: "4".to_string(),
            expiration_date: "5".to_string(),
            disposal_date: "6".to_string(),
            removal_date: "7".to_string(),
            disposal_method: "Trash".to_string(),
            active: false,
            container_type: "Can".to_string(),
            container_size: "8".to_string(),
            unit: "Oz".to_string(),
            percent_remaining: "20".to_string()
        };

        let fields = chemical_inventory.get_fields();

        assert_eq!(fields[0], Value::String("1".to_string()));
        assert_eq!(fields[1], Value::String("2".to_string()));
        assert_eq!(fields[2], Value::String("3".to_string()));
        assert_eq!(fields[3], Value::String("4".to_string()));
        assert_eq!(fields[4], Value::String("5".to_string()));
        assert_eq!(fields[5], Value::String("6".to_string()));
        assert_eq!(fields[6], Value::String("7".to_string()));
        assert_eq!(fields[7], Value::String("Trash".to_string()));
        assert_eq!(fields[8], Value::Boolean(false));
        assert_eq!(fields[9], Value::String("Can".to_string()));
        assert_eq!(fields[10], Value::String("8".to_string()));
        assert_eq!(fields[11], Value::String("Oz".to_string()));
        assert_eq!(fields[12], Value::String("20".to_string()));
        assert_eq!(fields.len(), 13);
    }

    #[test]
    fn test_chemicalinventory_get_field() {
        let chemical_inventory = ChemicalInventory {
            lot_number: "00".to_string(),
            purchase_date: "00/00/00".to_string(),
            arrival_date: "01/00/00".to_string(),
            open_date: "02/00/00".to_string(),
            expiration_date: "03/00/00".to_string(),
            disposal_date: "04/00/00".to_string(),
            removal_date: "05/00/00".to_string(),
            disposal_method: "In Chemical Waste".to_string(),
            active: true,
            container_type: "Jar".to_string(),
            container_size: "5".to_string(),
            unit: "grams".to_string(),
            percent_remaining: "80".to_string()
        };

        let lot_number = chemical_inventory.get_field(ChemicalInventoryFields::LotNumber);
        
        assert_eq!(lot_number, Some(Value::String("00".to_string())));
    }

}