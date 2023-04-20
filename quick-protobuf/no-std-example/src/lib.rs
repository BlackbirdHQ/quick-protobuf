#![no_std]


// extern crate alloc;

// use alloc::string::String;
// use alloc::string::ToString;

mod protos;

#[test]
pub fn round_trip() {
    use crate::protos::no_std::*;
    use quick_protobuf::{deserialize_from_slice, serialize_into_slice, sizeofs, MessageWrite};
    
    
    let message = NoStdMessage {
        // num: String::from("value"),
        num: heapless::String::from("value"),
        nums: heapless::Vec::from_slice(&[10, 15, 63]).unwrap(),
        message: Some(protos::no_std::EmbeddedMessage {
            val: -1337,
            e: protos::no_std::MyEnum::Val1,
        }),
        messages: heapless::Vec::from_slice(&[protos::no_std::EmbeddedMessage {
            val: 1337,
            e: protos::no_std::MyEnum::Val0,
        },
        protos::no_std::EmbeddedMessage {
            val: 12345,
            e: protos::no_std::MyEnum::Val1,
        }])
        .unwrap(),
    };

    let mut buf = [0u8; 1024];
    let len = message.get_size();
    serialize_into_slice(&message, &mut buf).unwrap();

    let read_message = deserialize_from_slice(&buf[..sizeofs::sizeof_len(len)]).unwrap();
    assert_eq!(message, read_message);
}
