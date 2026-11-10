#[derive(Debug, PartialEq, Eq)]
enum TimestampError {
    InvalidFormat,
    MinutesOutOfRange,
    SecondsOutOfRange,
}

fn parse_timestamp(_text: &str) -> Result<u32, TimestampError> {
    todo!()
}

fn main() {
    assert_eq!(parse_timestamp("00:00:00,000"), Ok(0));
    assert_eq!(parse_timestamp("00:00:00,001"), Ok(1));
    assert_eq!(parse_timestamp("00:00:00,999"), Ok(999));
    assert_eq!(parse_timestamp("00:00:01,000"), Ok(1_000));
    assert_eq!(parse_timestamp("00:01:00,000"), Ok(60_000));
    assert_eq!(parse_timestamp("01:00:00,000"), Ok(3_600_000));
    assert_eq!(parse_timestamp("01:23:45,600"), Ok(5_025_600));
    assert_eq!(parse_timestamp("99:59:59,999"), Ok(359_999_999));

    for text in ["00:60:00,000", "00:99:00,000", "00:60:60,000"] {
        assert_eq!(
            parse_timestamp(text),
            Err(TimestampError::MinutesOutOfRange)
        );
    }
    for text in ["00:00:60,000", "00:00:99,000"] {
        assert_eq!(
            parse_timestamp(text),
            Err(TimestampError::SecondsOutOfRange)
        );
    }
    for text in [
        "",
        "0:00:00,000",
        "000:00:00,000",
        "00:0:00,000",
        "00:00:0,000",
        "00:00:00,00",
        "00:00:00,0000",
        "00:00:00.000",
        "00-00:00,000",
        "00:00-00,000",
        "+1:00:00,000",
        "-1:00:00,000",
        "00:+1:00,000",
        "00:00:00,+01",
        "aa:00:00,000",
        "00:00:00,00x",
        " 00:00:00,000",
        "00:00:00,000 ",
        "00:00:00,000\n",
        "00:60:00,00x",
        "é:00:00,000",
        "００:00:00,000",
    ] {
        assert_eq!(parse_timestamp(text), Err(TimestampError::InvalidFormat));
    }

    println!("timestamps: ok");
}
