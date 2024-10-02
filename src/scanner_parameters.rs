use super::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use tracing::warn;

#[derive(Debug, Clone)]
struct Instrument {
    name: String,
    type_: String,
    sec_type: String,
    nscan_sec_type: String,
    filters: Vec<String>,
    group: String,
    short_name: String,
    cloud_scan_not_supported: bool,
    feature_codes: String,
}

#[derive(Debug, Clone)]
struct Location {
    display_name: String,
    short_name: String,
    tooltip: String,
    raw_price_only: String,
    location_code: String,
    instrument: String,
    route_exchange: String,
    delayed_only: String,
    access: String,
    child_locations: Vec<Location>,
}

#[derive(Debug, Clone)]
struct ScanType {
    display_name: String,
    scan_code: String,
    instruments: HashSet<String>,
    search_name: String,
    access: String,
    // feature?
    // settings?
    // respSizeLimit?
    // snapshotSizeLimit?
    // delayedAvail?
    // searchDefault?
    location_filter: Vec<String>,
}

#[derive(Debug)]
struct Filter {
    pub id: String,
    pub category: String,
    pub access: String,
    pub abstract_fields: Vec<AbstractField>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum ScannerFieldType {
    Boolean,
    Combo,
    ConvertedCombo, // only 1 filer: ISSUER_COUNTRY_CODE; // if ComboField$ConvertedComboField -> scanner.filter.StringField -> Combo
    Conid,          // only 1 filter: UNDCONID
    Date,           // valid variants mm/yyyy or yyyymmdd
    Double,
    Int,
    String,
    StringList, // only 1 filter: BOND_STK_SYMBOL
    SubstrList, // only 1 filter: BOND_ISSUER

    NotYetImplementedOrInitState(String),
}

impl From<&str> for ScannerFieldType {
    fn from(value: &str) -> Self {
        match value {
            "scanner.filter.BooleanField" => ScannerFieldType::Boolean,
            "scanner.filter.ComboField" => ScannerFieldType::Combo,
            "scanner.filter.StringField" => ScannerFieldType::Combo, // (!)
            "scanner.filter.ComboField$ConvertedComboField" => ScannerFieldType::ConvertedCombo, // its StringField!!! -> Combo
            "scanner.filter.ConidField" => ScannerFieldType::Conid,
            "scanner.filter.DateField" => ScannerFieldType::Date,
            "scanner.filter.DoubleField" => ScannerFieldType::Double,
            "scanner.filter.IntField" => ScannerFieldType::Int,
            // "scanner.filter.StringField" => ScannerFieldType::String,
            "scanner.filter.StringListField" => ScannerFieldType::StringList,
            "scanner.filter.SubstrListField" => ScannerFieldType::SubstrList,
            new => {
                warn!("Not covered filters field type: {}", new);
                ScannerFieldType::NotYetImplementedOrInitState(new.to_string())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct AbstractField {
    parameter_type: ScannerFieldType,
    code: String,
    code_not: String,
    display_name: String,
    tooltip: String,
    separator: String,
    combo_values: Vec<ComboValue>,
}

#[derive(Debug, Clone)]
struct ComboValue {
    code: String,
    display_name: String,
    tooltip: String,
    default: String,
    // synthetic_all: bool,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScannerParametersError {
    #[error("Failed parsing Int. Cause {0}")]
    /// Failed parsing Int.
    ParsingDate(#[from] std::num::ParseIntError),

    #[error("Incorrect day number in date. Cause {0}")]
    /// Incorrect month number in date.
    IncorrectDayNumberInDate(u8),
    #[error("Incorrect month number in date. Cause {0}")]
    /// Incorrect month number in date.
    IncorrectMonthNumberInDate(u8),
    #[error("Incorrect year number in date. Cause {0}")]
    /// Incorrect year number in date.
    IncorrectYearNumberInDate(u16),
    #[error("Init state")]
    /// Init state
    InitState,
    #[error("No result")]
    /// No result
    NoResult,
    #[error("Cause {0}")]
    /// quick_xml::Error error
    QuickXml(#[from] quick_xml::Error),
}

static EMPTY_STR: &str = "";

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_char_is_lowercase = false;

    for (i, c) in s.char_indices() {
        if c == '(' || c == ')' {
            continue;
        }
        if c.is_uppercase() {
            if i > 0 && prev_char_is_lowercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else if c.is_whitespace() || c == '-' || c == '.' {
            result.push('_');
        } else {
            result.push(c);
        }
        prev_char_is_lowercase = c.is_lowercase();
    }

    result
}

pub fn to_camel_case_save_acronyms(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next || c.is_uppercase() {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        } else {
            capitalize_next = true;
        }
    }

    result
}

pub fn camel_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(current_char) = chars.next() {
        if !result.is_empty() && result.chars().last().unwrap() != '_' {
            result.push('_');
        }
        if current_char.is_uppercase() {
            result.push(current_char.to_lowercase().next().unwrap());
        } else {
            result.push(current_char);
        }

        if let Some(&next_char) = chars.peek() {
            if next_char.is_uppercase() && current_char.is_lowercase() {
                result.push('_');
            }
        }
    }
    result.to_lowercase()
}

pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[test]
fn print_data_for_macros_input() {
    let GenDataForMacrosInput {
        gen_instrument,
        gen_location,
        gen_scan_code,
        gen_enums,
        gen_filters,
        list_of_filters_struct,
        unimplemented_filters,
    } = gen_data_for_macros_input().unwrap();
    // === OUTPUT ===

    println!("=== gen_instrument ===");
    gen_instrument.iter().for_each(|e| println!("{}", e));
    println!("=== gen_location ===");
    gen_location.iter().for_each(|e| println!("{}", e));
    println!("=== gen_scan_code ===");
    gen_scan_code.iter().for_each(|e| println!("{}", e));
    println!("=== gen_enums ===");
    gen_enums.iter().for_each(|e| println!("{}", e));
    println!("=== gen_filters ===");
    gen_filters.iter().for_each(|e| println!("{}", e));

    // println("=== list_of_filters_struct ===");
    // list_of_filters_struct.iter().for_each(|e| println!("{}", e));
    // println("=== unimplemented_filters ===");
    // unimplemented_filters.iter().for_each(|e| println!("{}", e));
}

struct GenDataForMacrosInput {
    gen_instrument: Vec<String>,
    gen_location: Vec<String>,
    gen_scan_code: Vec<String>,
    gen_enums: Vec<String>,
    gen_filters: Vec<String>,
    list_of_filters_struct: Vec<String>,
    unimplemented_filters: HashSet<String>,
}

fn gen_data_for_macros_input() -> Result<GenDataForMacrosInput, ScannerParametersError> {
    let scanner_parameters = parse_xml();

    let ParsedParameters {
        all_instruments,
        all_locations,
        all_scan_types,
        all_filters,
    } = match parse_xml() {
        Ok(result) => result,
        Err(error) => Err(error)?,
    };

    let all_instruments = all_instruments.unwrap();
    let all_locations = all_locations.unwrap();
    let all_scan_types = all_scan_types.unwrap();
    let all_filters = all_filters.unwrap();

    println!("────────────");
    println!("all_instruments: {}", &all_instruments.len());
    println!("all_locations: {}", &all_locations.len());
    println!("all_scan_types: {}", &all_scan_types.len());
    println!("all_filters: {}", &all_filters.len());
    println!("────────────");

    let mut gen_instrument = vec![];
    let mut gen_location = vec![];
    let mut gen_scan_code = vec![];
    let mut gen_enums = vec![];
    let mut gen_filters = vec![];

    let mut list_of_filters_struct = vec![];

    let mut unimplemented_filters = HashSet::new();

    let mut map_instrument_code_to_scan_code_struct_names: HashMap<String, String> = HashMap::new();
    let mut check_if_enum_name_created = HashSet::new();

    let postfix_select_location = "SelectLocation".to_string();
    let postfix_select_scan_code = "SelectScanCode".to_string();
    let postfix_select_filters = "SelectFilters".to_string();

    // === INSTRUMENTS ===
    gen_instrument.push("impl_select_instrument![ ScannerSubscription =>".to_string());

    for value in all_instruments.iter() {
        let Instrument {
            name,
            type_,
            filters,
            ..
        } = value;

        let struct_instrument_name = to_camel_case_save_acronyms(name);

        let mut struct_name_select_location = struct_instrument_name.clone();
        struct_name_select_location.push_str(&postfix_select_location);

        let mut struct_name_select_scan_code = struct_instrument_name.clone();
        struct_name_select_scan_code.push_str(&postfix_select_scan_code);

        gen_instrument.push(format!(
            r#"    ("{}", {}, {}, "{}"),"#,
            name.to_string(),
            to_snake_case(name),
            struct_name_select_location,
            type_
        ));

        // === LOCATIONS ===

        gen_location.push(format!(
            "\n impl_select_location![ {}, {}  =>",
            struct_name_select_location, struct_name_select_scan_code
        ));

        map_instrument_code_to_scan_code_struct_names
            .insert(type_.to_string(), struct_name_select_scan_code);

        if let Some(location) = all_locations.get(type_) {
            recursion_over_location(location, &mut gen_location);
            gen_location.push("]; \n".to_string());
        }

        // === SCAN CODES ===

        let mut structs_name_for_impl_filters = struct_instrument_name.clone();
        structs_name_for_impl_filters.push_str(&postfix_select_filters);

        if let Some(old_struct_name) = map_instrument_code_to_scan_code_struct_names.get(type_) {
            gen_scan_code.push(format!(
                "impl_select_scan_code![ {}, {} =>",
                old_struct_name, structs_name_for_impl_filters
            ));
        }

        for (_key, value) in all_scan_types.iter() {
            if value.instruments.get(type_).is_some() {
                let ScanType {
                    display_name,
                    scan_code,
                    search_name,
                    ..
                } = value;

                let doc = if !display_name.is_empty() && !search_name.is_empty() {
                    display_name.to_string() + "." + search_name
                } else if search_name.is_empty() {
                    display_name.to_string()
                } else {
                    search_name.to_string()
                };

                gen_scan_code.push(format!(
                    r#"    ("{}", {}, "{}"),"#,
                    doc,
                    to_snake_case(scan_code),
                    scan_code
                ));
            }
        }
        gen_scan_code.push("];".to_string());

        // === FILTERS ===

        let mut structs_name_for_impl_filters = struct_instrument_name.clone();
        structs_name_for_impl_filters.push_str(&postfix_select_filters);

        list_of_filters_struct.push(structs_name_for_impl_filters.clone() + ",");

        gen_filters.push(format!(
            "impl_select_filters![ {} =>",
            structs_name_for_impl_filters
        ));

        for filter_code in filters {
            if let Some(filter) = all_filters.get(filter_code) {
                for abstract_field in filter.abstract_fields.iter() {
                    let AbstractField {
                        parameter_type,
                        code,
                        code_not,
                        display_name,
                        tooltip,
                        ..
                    } = abstract_field;

                    let parameter_struct_name = match code.as_str() {
                        "bondAssetSubTypeStrBeginsWithOneOf" => "BondAssetSubTypeStr",
                        "bondUSStateLike" => "BondUSState",
                        c => &capitalize_first_letter(c),
                    };
                    let parameter_struct_name = parameter_struct_name
                        .to_string()
                        .replace("Above", "")
                        .replace("Below", "");

                    let parameter_type = match parameter_type {
                        ScannerFieldType::Combo | &ScannerFieldType::ConvertedCombo => {
                            parameter_struct_name
                        }
                        ScannerFieldType::Double => "f64".to_string(),
                        ScannerFieldType::Int => "i64".to_string(),
                        ScannerFieldType::String => "String".to_string(),
                        ScannerFieldType::Boolean => "bool".to_string(),
                        ScannerFieldType::Conid => "u64".to_string(),
                        ScannerFieldType::Date => "ScannerDate".to_string(),
                        ScannerFieldType::StringList => "BondStkSymbols".to_string(),
                        ScannerFieldType::SubstrList => "BondIssuers".to_string(),
                        ScannerFieldType::NotYetImplementedOrInitState(_) => unreachable!(),
                    };

                    let display_name = display_name.replace("&amp;", "n");

                    let doc = if !display_name.is_empty() && !tooltip.is_empty() {
                        format!("{display_name}. {tooltip}")
                    } else if display_name.is_empty() {
                        tooltip.to_string()
                    } else {
                        display_name.to_string()
                    };

                    gen_filters.push(format!(
                        r#"    ({}, "{}", {}, "{}"),"#,
                        to_snake_case(&code),
                        code,
                        parameter_type,
                        doc
                    ));

                    if !code_not.is_empty() {
                        gen_filters.push(format!(
                            r#"    ({}, "{}", {}, "{}"),"#,
                            to_snake_case(&code),
                            code_not,
                            parameter_type,
                            doc
                        ));
                    }

                    // === GEN COMBO ENUMS ===

                    if !check_if_enum_name_created.contains(&parameter_type) {
                        if abstract_field.combo_values.len() > 0 {
                            gen_enums.push(format!(
                                r#"create_enums_for_filters![ {}  => "#,
                                parameter_type
                            ));

                            let mut enum_variants = String::new();
                            for combo_value in &abstract_field.combo_values {
                                let ComboValue {
                                    code,
                                    //Documentation 2
                                    display_name,
                                    //Documentation 1
                                    tooltip,
                                    default,
                                } = combo_value;
                                if default == "true" {
                                    continue;
                                }

                                let mut enum_var_name = if tooltip.is_empty() {
                                    display_name
                                } else {
                                    tooltip
                                }
                                .replace("T-", "T")
                                .replace('-', "minus")
                                .replace('+', "plus");

                                if enum_var_name.is_empty() {
                                    enum_var_name = code.clone();
                                }

                                enum_variants.push_str(&format!(
                                    r#"({}, "{}"),"#,
                                    to_camel_case_save_acronyms(&enum_var_name),
                                    code
                                ));
                            }
                            gen_enums.push(enum_variants + "];\n");
                        }
                        check_if_enum_name_created.insert(parameter_type);
                    }
                }
            } else {
                unimplemented_filters.insert(filter_code.clone());
            }
        }
        gen_filters.push("];".to_string());
    }
    gen_instrument.push("];".to_string());

    Ok(GenDataForMacrosInput {
        gen_instrument,
        gen_location,
        gen_scan_code,
        gen_enums,
        gen_filters,
        list_of_filters_struct,
        unimplemented_filters,
    })
}

fn recursion_over_location(location: &Location, all_location: &mut Vec<String>) {
    let Location {
        display_name,
        tooltip,
        location_code,
        child_locations,
        ..
    } = location;

    let name = if !tooltip.is_empty() {
        tooltip
    } else {
        display_name
    };

    let func_name = if name.eq("Listed/NASDAQ") {
        &location_code
            .replace("STK.", "")
            .replace("ETF.EQ.", "")
            .replace("ETF.FI.", "")
    } else {
        name
    };

    all_location.push(format!(
        r#"    ("{}", {}, "{}"),"#,
        name.to_string(),
        to_snake_case(func_name),
        location_code
    ));

    for next_location in child_locations {
        recursion_over_location(next_location, all_location);
    }
}

struct ParsedParameters {
    // todo: add comment for Keys
    all_filters: Result<HashMap<String, Filter>, ScannerParametersError>,
    all_instruments: Result<Vec<Instrument>, ScannerParametersError>,
    all_locations: Result<HashMap<String, Location>, ScannerParametersError>,
    all_scan_types: Result<HashMap<String, ScanType>, ScannerParametersError>,
}

#[allow(clippy::single_match)]
fn parse_xml() -> Result<ParsedParameters, ScannerParametersError> {
    let path = Path::new("resources/resp_scan_param_rust.xml");
    let xml_content = fs::read_to_string(path).unwrap();

    let mut reader = Reader::from_str(&xml_content);
    reader.config_mut().trim_text(true);

    let mut all_filters: Result<HashMap<String, Filter>, ScannerParametersError> =
        Err(ScannerParametersError::InitState);
    let mut all_instruments: Result<Vec<Instrument>, ScannerParametersError> =
        Err(ScannerParametersError::InitState);
    let mut all_locations: Result<HashMap<String, Location>, ScannerParametersError> =
        Err(ScannerParametersError::InitState);
    let mut all_scan_types: Result<HashMap<String, ScanType>, ScannerParametersError> =
        Err(ScannerParametersError::InitState);

    let location_start = BytesStart::new("LocationTree");

    loop {
        match reader.read_event() {
            Err(e) => return Err(ScannerParametersError::QuickXml(e)),
            Ok(Event::Eof) => {
                break;
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"InstrumentList" => {
                // <InstrumentList varName="fullInstrumentList">

                let attr = e
                    .attributes()
                    .map(|a| String::from_utf8(a.unwrap().value.into_owned()).unwrap())
                    .collect::<Vec<_>>()[0]
                    .clone();
                if attr.eq("fullInstrumentList") {
                    all_instruments = parse_instruments(&mut reader);
                }
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"LocationTree" => {
                // <LocationTree varName="locationTree">

                let inner_text = reader
                    .read_text(location_start.to_end().to_owned().name())
                    .unwrap();
                let mut reader = Reader::from_str(&inner_text);
                all_locations = parse_locations(&mut reader);
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"ScanTypeList" => {
                // <ScanTypeList varName="scanTypeList">
                // At present, there is only one ScanTypeList.
                all_scan_types = parse_scan_types(&mut reader);
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"FilterList" => {
                // <FilterList varName="filterList">
                let attr = e
                    .attributes()
                    .map(|a| String::from_utf8(a.unwrap().value.into_owned()).unwrap())
                    .collect::<Vec<_>>()[0]
                    .clone();
                if attr.eq("filterList") {
                    all_filters = parse_filters(&mut reader);
                }
            }
            _ => (),
        }
    }

    Ok(ParsedParameters {
        all_instruments,
        all_locations,
        all_scan_types,
        all_filters,
    })
}

#[allow(clippy::single_match)]
fn parse_instruments(
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<Instrument>, ScannerParametersError> {
    // <InstrumentList varName="fullInstrumentList">

    let mut name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut type_ = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut sec_type = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut nscan_sec_type = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut filters = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut group = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut short_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut cloud_scan_not_supported = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut feature_codes = std::borrow::Cow::Borrowed(EMPTY_STR);

    let mut all_instruments: Vec<Instrument> = vec![];

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::End(e)) if e.name().as_ref() == b"InstrumentList" => {
                return Ok(all_instruments);
            }

            Ok(Event::End(e)) if e.name().as_ref() == b"Instrument" => {
                all_instruments.push(Instrument {
                    name: name.clone().into_owned(),
                    type_: type_.clone().into_owned(),
                    sec_type: sec_type.clone().into_owned(),
                    nscan_sec_type: nscan_sec_type.clone().into_owned(),
                    filters: filters.clone().split(",").map(|e| e.to_string()).collect(),
                    group: group.clone().into_owned(),
                    short_name: short_name.clone().into_owned(),
                    cloud_scan_not_supported: cloud_scan_not_supported
                        .clone()
                        .into_owned()
                        .parse::<bool>()
                        .unwrap_or(false),
                    feature_codes: feature_codes.clone().into_owned(),
                });

                name = std::borrow::Cow::Borrowed(EMPTY_STR);
                type_ = std::borrow::Cow::Borrowed(EMPTY_STR);
                sec_type = std::borrow::Cow::Borrowed(EMPTY_STR);
                nscan_sec_type = std::borrow::Cow::Borrowed(EMPTY_STR);
                filters = std::borrow::Cow::Borrowed(EMPTY_STR);
                group = std::borrow::Cow::Borrowed(EMPTY_STR);
                short_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                cloud_scan_not_supported = std::borrow::Cow::Borrowed(EMPTY_STR);
                feature_codes = std::borrow::Cow::Borrowed(EMPTY_STR);
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"name" => {
                    name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"type" => {
                    type_ = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"secType" => {
                    sec_type = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"nscanSecType" => {
                    nscan_sec_type = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"filters" => {
                    filters = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }

                b"group" => {
                    group = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"shortName" => {
                    short_name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"cloudScanNotSupported" => {
                    cloud_scan_not_supported = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"featureCodes" => {
                    feature_codes = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[allow(clippy::single_match)]
fn parse_locations(
    reader: &mut Reader<&[u8]>,
) -> Result<HashMap<String, Location>, ScannerParametersError> {
    // <LocationTree varName="locationTree">
    let mut display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut short_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut raw_price_only = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut location_code = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut instrument = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut route_exchange = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut delayed_only = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut access = std::borrow::Cow::Borrowed(EMPTY_STR);

    let mut all_locations: HashMap<String, Location> = HashMap::new();
    let mut deep = 0;
    let mut not_completed_locations: HashMap<usize, Location> = HashMap::with_capacity(26);

    let child_locations = vec![];
    let mut location_tree_end_right_now = false;

    loop {
        match reader.read_event()? {
            Event::Eof => {
                return Ok(all_locations);
            }
            Event::Start(e) if e.name().as_ref() == b"Location" => {
                //
            }
            Event::End(e) if e.name().as_ref() == b"Location" => {
                if location_tree_end_right_now {
                    location_tree_end_right_now = false;
                    continue;
                }

                let location = Location {
                    access: access.clone().into_owned(),
                    short_name: short_name.clone().into_owned(),
                    tooltip: tooltip.clone().into_owned(),
                    delayed_only: delayed_only.clone().into_owned(),
                    raw_price_only: raw_price_only.clone().into_owned(),
                    display_name: display_name.clone().into_owned(),
                    location_code: location_code.clone().into_owned(),
                    instrument: instrument.clone().into_owned(),
                    route_exchange: route_exchange.clone().into_owned(),
                    child_locations: child_locations.clone(),
                };

                if let Some(l) = not_completed_locations.get_mut(&deep) {
                    l.child_locations.push(location.clone());
                } else if deep == 0 {
                    all_locations.insert(location.instrument.clone(), location.clone());
                }

                display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                short_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
                raw_price_only = std::borrow::Cow::Borrowed(EMPTY_STR);
                location_code = std::borrow::Cow::Borrowed(EMPTY_STR);
                instrument = std::borrow::Cow::Borrowed(EMPTY_STR);
                route_exchange = std::borrow::Cow::Borrowed(EMPTY_STR);
                delayed_only = std::borrow::Cow::Borrowed(EMPTY_STR);
                access = std::borrow::Cow::Borrowed(EMPTY_STR);
            }

            Event::Start(e) if e.name().as_ref() == b"LocationTree" => {
                deep += 1;

                let location = Location {
                    access: access.clone().into_owned(),
                    short_name: short_name.clone().into_owned(),
                    tooltip: tooltip.clone().into_owned(),
                    delayed_only: delayed_only.clone().into_owned(),
                    raw_price_only: raw_price_only.clone().into_owned(),
                    display_name: display_name.clone().into_owned(),
                    location_code: location_code.clone().into_owned(),
                    instrument: instrument.clone().into_owned(),
                    route_exchange: route_exchange.clone().into_owned(),
                    child_locations: child_locations.clone(),
                };

                not_completed_locations.insert(deep, location);

                display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                short_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
                raw_price_only = std::borrow::Cow::Borrowed(EMPTY_STR);
                location_code = std::borrow::Cow::Borrowed(EMPTY_STR);
                instrument = std::borrow::Cow::Borrowed(EMPTY_STR);
                route_exchange = std::borrow::Cow::Borrowed(EMPTY_STR);
                delayed_only = std::borrow::Cow::Borrowed(EMPTY_STR);
                access = std::borrow::Cow::Borrowed(EMPTY_STR);
            }
            Event::End(e) if e.name().as_ref() == b"LocationTree" => {
                location_tree_end_right_now = true;

                if let Some(location) = not_completed_locations.remove(&deep) {
                    deep -= 1;

                    if deep == 0 {
                        all_locations.insert(location.instrument.clone(), location);
                    } else if let Some(location2) = not_completed_locations.get_mut(&(deep)) {
                        location2.child_locations.push(location)
                    }
                };
            }
            Event::Start(e) => match e.name().as_ref() {
                b"displayName" => {
                    display_name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"shortName" => {
                    short_name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"tooltip" => {
                    tooltip = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"nscanSecType" => {
                    raw_price_only = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"locationCode" => {
                    location_code = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"instruments" => {
                    instrument = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"routeExchange" => {
                    route_exchange = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"delayedOnly" => {
                    delayed_only = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"access" => {
                    access = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[allow(clippy::single_match)]
fn parse_scan_types(
    reader: &mut Reader<&[u8]>,
) -> Result<HashMap<String, ScanType>, ScannerParametersError> {
    let mut display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut scan_code = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut instruments = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut search_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut access = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut location_filter = std::borrow::Cow::Borrowed(EMPTY_STR);

    let mut all_scan_types = HashMap::with_capacity(450);

    loop {
        match reader.read_event()? {
            Event::Eof => {}
            Event::End(e) if e.name().as_ref() == b"ScanTypeList" => {
                return Ok(all_scan_types);
            }
            Event::End(e) if e.name().as_ref() == b"ScanType" => {
                all_scan_types.insert(
                    scan_code.clone().into_owned(),
                    ScanType {
                        display_name: display_name.clone().into_owned(),
                        scan_code: scan_code.clone().into_owned(),
                        instruments: instruments
                            .clone()
                            .split(",")
                            .map(|e| e.to_string())
                            .collect(),
                        search_name: search_name.clone().into_owned(),
                        access: access.clone().into_owned(),
                        location_filter: location_filter
                            .clone()
                            .split(":")
                            .map(|e| e.to_string())
                            .collect(),
                    },
                );

                display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                scan_code = std::borrow::Cow::Borrowed(EMPTY_STR);
                instruments = std::borrow::Cow::Borrowed(EMPTY_STR);
                search_name = std::borrow::Cow::Borrowed(EMPTY_STR);
                access = std::borrow::Cow::Borrowed(EMPTY_STR);
                location_filter = std::borrow::Cow::Borrowed(EMPTY_STR);
            }

            Event::Start(e) => match e.name().as_ref() {
                b"displayName" => {
                    display_name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"scanCode" => {
                    scan_code = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"instruments" => {
                    instruments = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"searchName" => {
                    search_name = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"access" => {
                    access = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                b"locationFilter" => {
                    location_filter = reader
                        .read_text(e.name())
                        .expect("Cannot decode text value");
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[allow(clippy::single_match)]
fn parse_filters(
    reader: &mut Reader<&[u8]>,
) -> Result<HashMap<String, Filter>, ScannerParametersError> {
    let mut filter_id = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut filter_category = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut filter_access = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut filter_abstract_fields = Vec::new();

    let mut abst_field_parameter_type =
        ScannerFieldType::NotYetImplementedOrInitState("".to_string());
    let mut abst_field_code = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut abst_field_code_not = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut abst_field_display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut abst_field_tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut abst_field_separator = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut abst_field_combo_values = Vec::new();

    let mut combo_value_code = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut combo_value_display_name = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut combo_value_tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
    let mut combo_value_default = std::borrow::Cow::Borrowed(EMPTY_STR);

    let mut inside_filter = false;
    let mut inside_abstract_field = false;
    let mut inside_combo_values = false;

    let mut all_filters = HashMap::with_capacity(280); // 242 + 37 1

    // TODO
    // +TripleComboFilter
    // +cover all FieldType

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"FilterList" => {
                    return Ok(all_filters);
                }
                _ => {}
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"RangeFilter" | b"SimpleFilter" => {
                    //add <TripleComboFilter> ?
                    inside_filter = true;
                    loop {
                        match reader.read_event()? {
                            Event::Start(e) => match e.name().as_ref() {
                                b"id" if inside_filter => {
                                    filter_id = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"category" if inside_filter => {
                                    filter_category = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"access" if inside_filter => {
                                    filter_access = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                // ─── AbstractField ───────────────────────────────────────────────────
                                b"AbstractField" => {
                                    // if filter_id == "ISSUER_COUNTRY_CODE" {
                                    //     let x = e
                                    //         .attributes()
                                    //         .map(|a| {
                                    //             String::from_utf8(a.unwrap().value.into_owned())
                                    //                 .unwrap()
                                    //         })
                                    //         .collect::<Vec<_>>()[0]
                                    //         .clone();
                                    //     println!("X:{:?}", x);
                                    // }

                                    inside_abstract_field = true;

                                    abst_field_parameter_type = e
                                        .attributes()
                                        .map(|a| {
                                            ScannerFieldType::from(
                                                String::from_utf8(a.unwrap().value.into_owned())
                                                    .unwrap()
                                                    .as_str(),
                                            )
                                        })
                                        .collect::<Vec<_>>()[0]
                                        .clone();
                                }
                                b"code" if inside_abstract_field && !inside_combo_values => {
                                    abst_field_code = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"codeNot" if inside_abstract_field && !inside_combo_values => {
                                    abst_field_code_not = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"displayName" if inside_abstract_field && !inside_combo_values => {
                                    abst_field_display_name = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"tooltip" if inside_abstract_field && !inside_combo_values => {
                                    abst_field_tooltip = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"separator" if inside_abstract_field && !inside_combo_values => {
                                    abst_field_separator = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"varName" if inside_abstract_field && !inside_combo_values => {
                                    // unimplemented
                                }

                                // ─── ComboValues ───────────────────────────────────────────────────
                                b"ComboValues" if inside_abstract_field => {
                                    inside_combo_values = true;
                                }

                                b"code" if inside_combo_values => {
                                    combo_value_code = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }

                                b"displayName" if inside_combo_values => {
                                    combo_value_display_name = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"tooltip" if inside_combo_values => {
                                    combo_value_tooltip = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                b"default" if inside_combo_values => {
                                    combo_value_default = reader
                                        .read_text(e.name())
                                        .expect("Cannot decode text value");
                                }
                                _ => {}
                            },
                            Event::End(e) => {
                                match e.name().as_ref() {
                                    // ─── Filter ──────────────────────────────────────────────────────────
                                    b"RangeFilter" | b"SimpleFilter" | b"TripleComboFilter" => {
                                        inside_filter = false;

                                        all_filters.insert(
                                            filter_id.clone().into_owned(),
                                            Filter {
                                                id: filter_id.clone().into_owned(),
                                                category: filter_category.clone().into_owned(),
                                                access: filter_access.clone().into_owned(),
                                                abstract_fields: filter_abstract_fields.clone(),
                                            },
                                        );

                                        filter_id = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        filter_category = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        filter_access = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        filter_abstract_fields.clear();

                                        break;
                                    }
                                    // ─── AbstractField ───────────────────────────────────────────────────
                                    b"AbstractField" if inside_abstract_field => {
                                        inside_abstract_field = false;

                                        filter_abstract_fields.push(AbstractField {
                                            parameter_type: abst_field_parameter_type.clone(),
                                            code: abst_field_code.clone().into_owned(),
                                            code_not: abst_field_code_not.clone().into_owned(),
                                            display_name: abst_field_display_name
                                                .clone()
                                                .into_owned(),
                                            tooltip: abst_field_tooltip.clone().into_owned(),
                                            separator: abst_field_separator.clone().into_owned(),
                                            combo_values: abst_field_combo_values.clone(),
                                        });

                                        abst_field_parameter_type =
                                            ScannerFieldType::NotYetImplementedOrInitState(
                                                EMPTY_STR.to_string(),
                                            );
                                        abst_field_code = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        abst_field_code_not = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        abst_field_display_name =
                                            std::borrow::Cow::Borrowed(EMPTY_STR);
                                        abst_field_tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        abst_field_separator =
                                            std::borrow::Cow::Borrowed(EMPTY_STR);
                                        abst_field_combo_values.clear();
                                        // println!("{:?}", f);
                                    }

                                    // ─── ComboValues ───────────────────────────────────────────────────
                                    b"ComboValue" => {
                                        abst_field_combo_values.push(ComboValue {
                                            code: combo_value_code.clone().into_owned(),
                                            display_name: combo_value_display_name
                                                .clone()
                                                .into_owned(),
                                            tooltip: combo_value_tooltip.clone().into_owned(),
                                            default: combo_value_default.clone().into_owned(),
                                        });

                                        combo_value_code = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        combo_value_display_name =
                                            std::borrow::Cow::Borrowed(EMPTY_STR);
                                        combo_value_tooltip = std::borrow::Cow::Borrowed(EMPTY_STR);
                                        combo_value_default = std::borrow::Cow::Borrowed(EMPTY_STR);
                                    }
                                    b"ComboValues" if inside_abstract_field => {
                                        inside_combo_values = false;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
