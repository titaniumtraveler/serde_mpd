use crate::{
    deserializer::request::{
        tests::macros::test_deserialize, DeserializerState, RequestDeserializer,
    },
    read::{InteruptSlice, Slice},
    Error::Pending,
};
use serde::de::IgnoredAny;

test_deserialize! {
    ignore_any_ack;
    |src| RequestDeserializer::from_read(Slice { src });
    b"ACK ...error message\n";
    (_, &[u8], DeserializerState)

    b"";
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
test_deserialize! {
    ignore_any_ok_unit;
    |src| RequestDeserializer::from_read(Slice { src });
    b"OK\n";
    (_, &[u8], DeserializerState)

    b"";
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
test_deserialize! {
    ignore_any_ok_entries;
    |src| RequestDeserializer::from_read(Slice { src });
    b"entry1: 1\nentry2: 2\nentry2: 3\nOK\n";
    (_, &[u8], DeserializerState)

    b"";
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}

test_deserialize! {
    ignore_any_interrupt_ack1;
    |src| RequestDeserializer::from_read(InteruptSlice { src });
    &mut [b"ACK", b"", b" ...", b"error message\n"];
    (&mut [&[u8]], &[&[u8]], DeserializerState)

    &[b"", b" ...", b"error message\n"];
    b"ACK";
    DeserializerState::None;
    Err(Pending);

    &[b" ...", b"error message\n"];
    b"ACK";
    DeserializerState::None;
    Err(Pending);

    &[b"error message\n"];
    b"";
    DeserializerState::IgnoreAnyAck;
    Err(Pending);

    &[b""];
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
test_deserialize! {
    ignore_any_interrupt_ok_unit1;
    |src| RequestDeserializer::from_read(InteruptSlice { src });
    &mut [b"OK\n"];
    (&mut [&[u8]], &[&[u8]], DeserializerState)

    &[&[]];
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
test_deserialize! {
    ignore_any_interrupt_ok_entries1;
    |src| RequestDeserializer::from_read(InteruptSlice { src });
    &mut [b"entry1: 1\nentry2: 2\nentry2: 3\nOK\n"];
    (&mut [&[u8]], &[&[u8]], DeserializerState)

    &mut [&[]];
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
test_deserialize! {
    ignore_any_ack_interrupt3;
    |src| RequestDeserializer::from_read(InteruptSlice { src });
    &mut [ b"", b"entry1", b": 1", b"\ne", b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"];
    (&mut [&[u8]], &[&[u8]], DeserializerState)

    &[b"entry1", b": 1", b"\ne", b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"];
    b"";
    DeserializerState::None;
    Err(Pending);

    &[b": 1", b"\ne", b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"];
    b"";
    DeserializerState::None;
    Err(Pending);

    &[b"\ne", b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"];
    b"";
    DeserializerState::None;
    Err(Pending);

    &[b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"];
    b"";
    DeserializerState::None;
    Err(Pending);

    &[b"2: 3\nO", b"K\n"];
    b"";
    DeserializerState::None;
    Err(Pending);

    &[b"K\n"];
    b"O";
    DeserializerState::None;
    Err(Pending);

    &[b""];
    b"";
    DeserializerState::None;
    Ok(IgnoredAny);
}
