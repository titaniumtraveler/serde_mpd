use crate::{
    deserializer::request::{DeserializerState, RequestDeserializer},
    read::{InteruptSlice, Slice},
    Error,
};
use serde::{de::IgnoredAny, Deserialize};

#[test]
fn ignore_any_slice() {
    let inputs: &[&[u8]] = &[
        b"ACK ...error message\n",
        b"OK\n",
        b"entry1: 1\nentry2: 2\nentry2: 3\nOK\n",
    ];
    for src in inputs {
        let mut deserializer = RequestDeserializer::from_read(Slice { src });
        assert_eq!(Deserialize::deserialize(&mut deserializer), Ok(IgnoredAny));
        assert_eq!(
            deserializer,
            RequestDeserializer {
                read: Slice { src: b"" },
                scratch: Vec::new(),
                state: DeserializerState::None
            }
        );
    }
}

#[test]
fn ignore_any_interrupt() {
    macro_rules! run_tests {
        ($input:expr => $expected_outputs:expr;) => {
            let input: &mut [&[u8]] = $input;
            let expected_outputs: &[Result<_, Error>] = $expected_outputs;

            let mut deserializer = RequestDeserializer::from_read(InteruptSlice { src: input });
            let mut outputs = Vec::with_capacity(expected_outputs.len());
            loop {
                let result = Deserialize::deserialize(&mut deserializer);
                match result {
                    Err(Error::Pending) => {
                        outputs.push(result);
                        continue;
                    }
                    result @ (Ok(_) | Err(_)) => {
                        outputs.push(result);
                        break;
                    }
                }
            }
            assert_eq!(expected_outputs, &outputs);
            assert_eq!(
                deserializer,
                RequestDeserializer {
                    read: InteruptSlice { src: &mut [b""] },
                    scratch: Vec::new(),
                    state: DeserializerState::None
                }
            );
        };
        ($($input:expr => $expected_outputs:expr;)*) => {

            $(run_tests!{$input => $expected_outputs;})*

        }
    }
    use Error::Pending;
    run_tests! {
        &mut [b"ACK ...error message\n"] => &[Ok(IgnoredAny)];
        &mut [b"ACK", b"", b" ...", b"error message\n"]
            => &[Err(Pending), Err(Pending), Err(Pending), Ok(IgnoredAny)];
        &mut [b"OK\n"] => &[Ok(IgnoredAny)];
        &mut [b"entry1: 1\nentry2: 2\nentry2: 3\nOK\n"]
            => &[Ok(IgnoredAny)];
        &mut [ b"", b"entry1", b": 1", b"\ne", b"ntry2: 2\nentry", b"2: 3\nO", b"K\n"]
            => &[Err(Pending), Err(Pending), Err(Pending), Err(Pending), Err(Pending), Err(Pending), Ok(IgnoredAny)];
    }
}
