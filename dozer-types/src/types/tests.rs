use crate::types::{field_test_cases, DozerDuration, DozerPoint, Field, TimeUnit};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use ordered_float::OrderedFloat;
use rust_decimal::Decimal;

#[test]
fn data_encoding_len_must_agree_with_encode() {
    for field in field_test_cases() {
        let bytes = field.encode_data();
        assert_eq!(bytes.len(), field.data_encoding_len());
    }
}

#[test]
fn test_as_conversion() {
    let field = Field::UInt(1);
    assert!(field.as_uint().is_some());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_some());
    assert!(field.as_null().is_none());

    let field = Field::Int(1);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_some());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_some());
    assert!(field.as_null().is_none());

    let field = Field::U128(1);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_some());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_some());
    assert!(field.as_null().is_none());

    let field = Field::I128(1);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_some());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_some());
    assert!(field.as_null().is_none());

    let field = Field::Float(OrderedFloat::from(1.0));
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_some());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Boolean(true);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_some());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::String("".to_string());
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_some());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Text("".to_string());
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_some());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Binary(vec![]);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_some());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Decimal(Decimal::from(1));
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_some());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Timestamp(DateTime::from(Utc.timestamp_millis_opt(0).unwrap()));
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_some());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Date(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_some());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Bson(vec![]);
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_some());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Point(DozerPoint::from((0.0, 0.0)));
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_some());
    assert!(field.as_duration().is_none());
    assert!(field.as_null().is_none());

    let field = Field::Duration(DozerDuration(
        std::time::Duration::from_nanos(123_u64),
        TimeUnit::Nanoseconds,
    ));
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_duration().is_some());
    assert!(field.as_null().is_none());

    let field = Field::Null;
    assert!(field.as_uint().is_none());
    assert!(field.as_int().is_none());
    assert!(field.as_u128().is_none());
    assert!(field.as_i128().is_none());
    assert!(field.as_float().is_none());
    assert!(field.as_boolean().is_none());
    assert!(field.as_string().is_none());
    assert!(field.as_text().is_none());
    assert!(field.as_binary().is_none());
    assert!(field.as_decimal().is_none());
    assert!(field.as_timestamp().is_none());
    assert!(field.as_date().is_none());
    assert!(field.as_bson().is_none());
    assert!(field.as_point().is_none());
    assert!(field.as_null().is_some());
}

#[test]
fn test_to_conversion() {
    let field = Field::UInt(1);
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Int(1);
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::U128(1);
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::I128(1);
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Float(OrderedFloat::from(1.0));
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Boolean(true);
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::String("".to_string());
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Text("".to_string());
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Binary(vec![]);
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_some());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Decimal(Decimal::from(1));
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Timestamp(DateTime::from(Utc.timestamp_millis_opt(0).unwrap()));
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_some());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Date(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_some());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Bson(vec![]);
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_none());
    assert!(field.to_text().is_none());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_some());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Point(DozerPoint::from((0.0, 0.0)));
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_none());
    assert!(field.to_text().is_none());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_some());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_none());

    let field = Field::Duration(DozerDuration(
        std::time::Duration::from_nanos(123_u64),
        TimeUnit::Nanoseconds,
    ));
    assert!(field.to_uint().is_none());
    assert!(field.to_int().is_none());
    assert!(field.to_u128().is_none());
    assert!(field.to_i128().is_none());
    assert!(field.to_float().is_none());
    assert!(field.to_boolean().is_none());
    assert!(field.to_string().is_none());
    assert!(field.to_text().is_none());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_none());
    assert!(field.to_timestamp().unwrap().is_none());
    assert!(field.to_date().unwrap().is_none());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_some());
    assert!(field.to_null().is_none());

    let field = Field::Null;
    assert!(field.to_uint().is_some());
    assert!(field.to_int().is_some());
    assert!(field.to_u128().is_some());
    assert!(field.to_i128().is_some());
    assert!(field.to_float().is_some());
    assert!(field.to_boolean().is_some());
    assert!(field.to_string().is_some());
    assert!(field.to_text().is_some());
    assert!(field.to_binary().is_none());
    assert!(field.to_decimal().is_some());
    assert!(field.to_timestamp().unwrap().is_some());
    assert!(field.to_date().unwrap().is_some());
    assert!(field.to_bson().is_none());
    assert!(field.to_point().is_none());
    assert!(field.to_duration().is_none());
    assert!(field.to_null().is_some());
}
