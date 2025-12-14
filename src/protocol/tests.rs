// src/protocol/tests.rs
use super::*;

#[test]
fn test_checksum_simple() {
    assert_eq!(checksum(&[0x8C, 0x00, 0x00, 0x02, 0x01]), 0x8F);
}

#[test]
fn test_checksum_wraps() {
    // Sum exceeds 255, should wrap
    assert_eq!(checksum(&[0xFF, 0xFF]), 0xFE);
}

#[test]
fn test_build_control_packet_power_on() {
    let packet = build_control_packet(0x00, &[0x01]);
    // Header=0x8C, Category=0x00, Function=0x00, Length=0x02, Data=0x01, Checksum
    assert_eq!(packet[0], 0x8C);
    assert_eq!(packet[1], 0x00);
    assert_eq!(packet[2], 0x00);
    assert_eq!(packet[3], 0x02);
    assert_eq!(packet[4], 0x01);
    assert_eq!(packet[5], checksum(&packet[0..5]));
}

#[test]
fn test_build_query_packet_power() {
    let packet = build_query_packet(0x00);
    assert_eq!(packet[0], 0x83);
    assert_eq!(packet[1], 0x00);
    assert_eq!(packet[2], 0x00);
    assert_eq!(packet[3], 0xFF);
    assert_eq!(packet[4], 0xFF);
    assert_eq!(packet[5], checksum(&packet[0..5]));
}

#[test]
fn test_response_code_try_from() {
    assert_eq!(ResponseCode::try_from(0x00).unwrap(), ResponseCode::Success);
    assert_eq!(
        ResponseCode::try_from(0x01).unwrap(),
        ResponseCode::LimitOverMax
    );
    assert_eq!(
        ResponseCode::try_from(0x02).unwrap(),
        ResponseCode::LimitOverMin
    );
    assert_eq!(
        ResponseCode::try_from(0x03).unwrap(),
        ResponseCode::Canceled
    );
    assert_eq!(
        ResponseCode::try_from(0x04).unwrap(),
        ResponseCode::ParseError
    );
    assert!(ResponseCode::try_from(0x05).is_err());
}
