use crate::output;
use crate::pipeline::aggregation::tests::aggregation_tests_utils::{
    delete_exp, delete_field, get_date_field, get_decimal_field, get_ts_field, init_input_schema,
    init_processor, insert_exp, insert_field, update_exp, update_field, DATE16, DATE4, DATE8,
    FIELD_100_FLOAT, FIELD_100_INT, FIELD_200_FLOAT, FIELD_200_INT, FIELD_50_FLOAT, FIELD_50_INT,
    ITALY, SINGAPORE,
};
use dozer_core::dag::dag::DEFAULT_PORT_HANDLE;
use dozer_core::storage::transactions::SharedTransaction;
use dozer_types::types::FieldType::{Date, Decimal, Float, Int, Timestamp};
use std::collections::HashMap;

#[test]
fn test_min_aggregation_float() {
    let schema = init_input_schema(Float, "MIN");
    let (processor, tx) = init_processor(
        "SELECT Country, MIN(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
        HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
    )
    .unwrap();

    // Insert 100 for segment Italy
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    let mut inp = insert_field(ITALY, FIELD_100_FLOAT);
    let mut out = output!(processor, inp, tx);
    let mut exp = vec![insert_exp(ITALY, FIELD_100_FLOAT)];
    assert_eq!(out, exp);

    // Insert another 100 for segment Italy
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = insert_field(ITALY, FIELD_100_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_100_FLOAT, FIELD_100_FLOAT)];
    assert_eq!(out, exp);

    // Insert 50 for segment Singapore
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0

        Singapore, 50.0
        ---------------
        MIN = 50.0
    */
    inp = insert_field(SINGAPORE, FIELD_50_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![insert_exp(SINGAPORE, FIELD_50_FLOAT)];
    assert_eq!(out, exp);

    // Update Singapore segment to Italy
    /*
        Italy, 100.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(SINGAPORE, ITALY, FIELD_50_FLOAT, FIELD_50_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![
        update_exp(ITALY, ITALY, FIELD_100_FLOAT, FIELD_50_FLOAT),
        delete_exp(SINGAPORE, FIELD_50_FLOAT),
    ];
    assert_eq!(out, exp);

    // Update Italy value 100 -> 200
    /*
        Italy, 200.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(ITALY, ITALY, FIELD_100_FLOAT, FIELD_200_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_FLOAT, FIELD_50_FLOAT)];
    assert_eq!(out, exp);

    // Delete 1 record (200)
    /*
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = delete_field(ITALY, FIELD_200_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_FLOAT, FIELD_50_FLOAT)];
    assert_eq!(out, exp);

    // Delete another record (50)
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = delete_field(ITALY, FIELD_50_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_FLOAT, FIELD_100_FLOAT)];
    assert_eq!(out, exp);

    // Delete last record
    /*
        -------------
        MIN = Null
    */
    inp = delete_field(ITALY, FIELD_100_FLOAT);
    out = output!(processor, inp, tx);
    exp = vec![delete_exp(ITALY, FIELD_100_FLOAT)];
    assert_eq!(out, exp);
}

#[test]
fn test_min_aggregation_int() {
    let schema = init_input_schema(Int, "MIN");
    let (processor, tx) = init_processor(
        "SELECT Country, MIN(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
        HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
    )
    .unwrap();

    // Insert 100 for segment Italy
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    let mut inp = insert_field(ITALY, FIELD_100_INT);
    let mut out = output!(processor, inp, tx);
    let mut exp = vec![insert_exp(ITALY, FIELD_100_INT)];
    assert_eq!(out, exp);

    // Insert another 100 for segment Italy
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = insert_field(ITALY, FIELD_100_INT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_100_INT, FIELD_100_INT)];
    assert_eq!(out, exp);

    // Insert 50 for segment Singapore
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0

        Singapore, 50.0
        ---------------
        MIN = 50.0
    */
    inp = insert_field(SINGAPORE, FIELD_50_INT);
    out = output!(processor, inp, tx);
    exp = vec![insert_exp(SINGAPORE, FIELD_50_INT)];
    assert_eq!(out, exp);

    // Update Singapore segment to Italy
    /*
        Italy, 100.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(SINGAPORE, ITALY, FIELD_50_INT, FIELD_50_INT);
    out = output!(processor, inp, tx);
    exp = vec![
        update_exp(ITALY, ITALY, FIELD_100_INT, FIELD_50_INT),
        delete_exp(SINGAPORE, FIELD_50_INT),
    ];
    assert_eq!(out, exp);

    // Update Italy value 100 -> 200
    /*
        Italy, 200.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(ITALY, ITALY, FIELD_100_INT, FIELD_50_INT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_INT, FIELD_50_INT)];
    assert_eq!(out, exp);

    // Delete 1 record (200)
    /*
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = delete_field(ITALY, FIELD_200_INT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_INT, FIELD_50_INT)];
    assert_eq!(out, exp);

    // Delete another record (50)
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = delete_field(ITALY, FIELD_50_INT);
    out = output!(processor, inp, tx);
    exp = vec![update_exp(ITALY, ITALY, FIELD_50_INT, FIELD_50_INT)];
    assert_eq!(out, exp);

    // Delete last record
    /*
        -------------
        MIN = Null
    */
    inp = delete_field(ITALY, FIELD_100_INT);
    out = output!(processor, inp, tx);
    exp = vec![delete_exp(ITALY, FIELD_50_INT)];
    assert_eq!(out, exp);
}

#[test]
fn test_min_aggregation_decimal() {
    let schema = init_input_schema(Decimal, "MIN");
    let (processor, tx) = init_processor(
        "SELECT Country, MIN(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
        HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
    )
    .unwrap();

    // Insert 100 for segment Italy
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    let mut inp = insert_field(ITALY, &get_decimal_field(100));
    let mut out = output!(processor, inp, tx);
    let mut exp = vec![insert_exp(ITALY, &get_decimal_field(100))];
    assert_eq!(out, exp);

    // Insert another 100 for segment Italy
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = insert_field(ITALY, &get_decimal_field(100));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_decimal_field(100),
        &get_decimal_field(100),
    )];
    assert_eq!(out, exp);

    // Insert 50 for segment Singapore
    /*
        Italy, 100.0
        Italy, 100.0
        -------------
        MIN = 100.0

        Singapore, 50.0
        -------------
        MIN = 50.0
    */
    inp = insert_field(SINGAPORE, &get_decimal_field(50));
    out = output!(processor, inp, tx);
    exp = vec![insert_exp(SINGAPORE, &get_decimal_field(50))];
    assert_eq!(out, exp);

    // Update Singapore segment to Italy
    /*
        Italy, 100.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(
        SINGAPORE,
        ITALY,
        &get_decimal_field(50),
        &get_decimal_field(50),
    );
    out = output!(processor, inp, tx);
    exp = vec![
        update_exp(
            ITALY,
            ITALY,
            &get_decimal_field(100),
            &get_decimal_field(50),
        ),
        delete_exp(SINGAPORE, &get_decimal_field(50)),
    ];
    assert_eq!(out, exp);

    // Update Italy value 100 -> 200
    /*
        Italy, 200.0
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = update_field(
        ITALY,
        ITALY,
        &get_decimal_field(100),
        &get_decimal_field(200),
    );
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_decimal_field(50),
        &get_decimal_field(50),
    )];
    assert_eq!(out, exp);

    // Delete 1 record (200)
    /*
        Italy, 100.0
        Italy, 50.0
        -------------
        MIN = 50.0
    */
    inp = delete_field(ITALY, &get_decimal_field(200));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_decimal_field(50),
        &get_decimal_field(50),
    )];
    assert_eq!(out, exp);

    // Delete another record (50)
    /*
        Italy, 100.0
        -------------
        MIN = 100.0
    */
    inp = delete_field(ITALY, &get_decimal_field(50));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_decimal_field(50),
        &get_decimal_field(100),
    )];
    assert_eq!(out, exp);

    // Delete last record
    /*
        -------------
        MIN = 0.0
    */
    inp = delete_field(ITALY, &get_decimal_field(100));
    out = output!(processor, inp, tx);
    exp = vec![delete_exp(ITALY, &get_decimal_field(100))];
    assert_eq!(out, exp);
}

#[test]
fn test_min_aggregation_timestamp() {
    let schema = init_input_schema(Timestamp, "MIN");
    let (processor, tx) = init_processor(
        "SELECT Country, MIN(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
        HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
    )
    .unwrap();

    // Insert 100 for segment Italy
    /*
        Italy, 100
        -------------
        MIN = 100
    */
    let mut inp = insert_field(ITALY, &get_ts_field(100));
    let mut out = output!(processor, inp, tx);
    let mut exp = vec![insert_exp(ITALY, &get_ts_field(100))];
    assert_eq!(out, exp);

    // Insert another 100 for segment Italy
    /*
        Italy, 100
        Italy, 100
        -------------
        MIN = 100
    */
    inp = insert_field(ITALY, &get_ts_field(100));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_ts_field(100),
        &get_ts_field(100),
    )];
    assert_eq!(out, exp);

    // Insert 50 for segment Singapore
    /*
        Italy, 100
        Italy, 100
        -------------
        MIN = 100

        Singapore, 50
        -------------
        MIN = 50
    */
    inp = insert_field(SINGAPORE, &get_ts_field(50));
    out = output!(processor, inp, tx);
    exp = vec![insert_exp(SINGAPORE, &get_ts_field(50))];
    assert_eq!(out, exp);

    // Update Singapore segment to Italy
    /*
        Italy, 100
        Italy, 100
        Italy, 50
        -------------
        MIN = 50
    */
    inp = update_field(SINGAPORE, ITALY, &get_ts_field(50), &get_ts_field(50));
    out = output!(processor, inp, tx);
    exp = vec![
        update_exp(ITALY, ITALY, &get_ts_field(100), &get_ts_field(50)),
        delete_exp(SINGAPORE, &get_ts_field(50)),
    ];
    assert_eq!(out, exp);

    // Update Italy value 100 -> 200
    /*
        Italy, 200
        Italy, 100
        Italy, 50
        -------------
        MIN = 50
    */
    inp = update_field(ITALY, ITALY, &get_ts_field(100), &get_ts_field(200));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_ts_field(50),
        &get_ts_field(50),
    )];
    assert_eq!(out, exp);

    // Delete 1 record (200)
    /*
        Italy, 100
        Italy, 50
        -------------
        MIN = 50
    */
    inp = delete_field(ITALY, &get_ts_field(200));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_ts_field(50),
        &get_ts_field(50),
    )];
    assert_eq!(out, exp);

    // Delete another record (50)
    /*
        Italy, 100
        -------------
        MIN = 100
    */
    inp = delete_field(ITALY, &get_ts_field(50));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_ts_field(50),
        &get_ts_field(100),
    )];
    assert_eq!(out, exp);

    // Delete last record
    /*
        -------------
        MIN = 0
    */
    inp = delete_field(ITALY, &get_ts_field(100));
    out = output!(processor, inp, tx);
    exp = vec![delete_exp(ITALY, &get_ts_field(100))];
    assert_eq!(out, exp);
}

#[test]
fn test_min_aggregation_date() {
    let schema = init_input_schema(Date, "MIN");
    let (processor, tx) = init_processor(
        "SELECT Country, MIN(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
        HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
    )
    .unwrap();

    // Insert 2015-10-08 for segment Italy
    /*
        Italy, 2015-10-08
        ------------------
        MIN = 2015-10-08
    */
    let mut inp = insert_field(ITALY, &get_date_field(DATE8));
    let mut out = output!(processor, inp, tx);
    let mut exp = vec![insert_exp(ITALY, &get_date_field(DATE8))];
    assert_eq!(out, exp);

    // Insert another 2015-10-08 for segment Italy
    /*
        Italy, 2015-10-08
        Italy, 2015-10-08
        -----------------
        MIN = 2015-10-08
    */
    inp = insert_field(ITALY, &get_date_field(DATE8));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_date_field(DATE8),
        &get_date_field(DATE8),
    )];
    assert_eq!(out, exp);

    // Insert 2015-10-04 for segment Singapore
    /*
        Italy, 2015-10-08
        Italy, 2015-10-08
        -------------
        MIN = 2015-10-08

        Singapore, 2015-10-04
        -------------
        MIN = 2015-10-04
    */
    inp = insert_field(SINGAPORE, &get_date_field(DATE4));
    out = output!(processor, inp, tx);
    exp = vec![insert_exp(SINGAPORE, &get_date_field(DATE4))];
    assert_eq!(out, exp);

    // Update Singapore segment to Italy
    /*
        Italy, 2015-10-08
        Italy, 2015-10-08
        Italy, 2015-10-04
        -------------
        MIN = 2015-10-04
    */
    inp = update_field(
        SINGAPORE,
        ITALY,
        &get_date_field(DATE4),
        &get_date_field(DATE4),
    );
    out = output!(processor, inp, tx);
    exp = vec![
        update_exp(ITALY, ITALY, &get_date_field(DATE8), &get_date_field(DATE4)),
        delete_exp(SINGAPORE, &get_date_field(DATE4)),
    ];
    assert_eq!(out, exp);

    // Update Italy value 100 -> 200
    /*
        Italy, 2015-10-16
        Italy, 2015-10-08
        Italy, 2015-10-04
        -------------
        MIN = 2015-10-04
    */
    inp = update_field(
        ITALY,
        ITALY,
        &get_date_field(DATE8),
        &get_date_field(DATE16),
    );
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_date_field(DATE4),
        &get_date_field(DATE4),
    )];
    assert_eq!(out, exp);

    // Delete 1 record (2015-10-16)
    /*
        Italy, 2015-10-08
        Italy, 2015-10-04
        -------------
        MIN = 2015-10-04
    */
    inp = delete_field(ITALY, &get_date_field(DATE16));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_date_field(DATE4),
        &get_date_field(DATE4),
    )];
    assert_eq!(out, exp);

    // Delete another record (2015-10-04)
    /*
        Italy, 2015-10-08
        -------------
        MIN = 2015-10-08
    */
    inp = delete_field(ITALY, &get_date_field(DATE4));
    out = output!(processor, inp, tx);
    exp = vec![update_exp(
        ITALY,
        ITALY,
        &get_date_field(DATE4),
        &get_date_field(DATE8),
    )];
    assert_eq!(out, exp);

    // Delete last record
    /*
        -------------
        MIN = 0
    */
    inp = delete_field(ITALY, &get_date_field(DATE8));
    out = output!(processor, inp, tx);
    exp = vec![delete_exp(ITALY, &get_date_field(DATE8))];
    assert_eq!(out, exp);
}