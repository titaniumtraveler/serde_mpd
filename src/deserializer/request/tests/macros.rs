macro_rules! test_deserialize {
    (
        $(#[$meta:meta])?
        $name:ident;
        $deserializer:expr;
        $deserializer_input:expr;
        ($deserializer_input_type:ty, $input_type:ty, $state_type:ty)

        $(
            $input:expr;
            $scratch:expr;
            $state:expr;
            $result:expr;
        )*
    ) => {
        $($meta:meta)?
        #[allow(unused)]
        #[test]
        fn $name() {
            use crate::read::SliceDebug;
            use serde::de::Deserialize;
            let mut input: $deserializer_input_type = $deserializer_input;
            let mut deserializer = $deserializer(input);
            let mut result;
            $(
                result = Deserialize::deserialize(&mut deserializer);
                assert_eq!(result, $result);
                let scratch: &[u8] = $scratch;
                assert_eq!(SliceDebug(scratch), SliceDebug(deserializer.scratch.as_slice()));
                let input: $input_type = $input;
                assert_eq!(SliceDebug(&*deserializer.read.src), SliceDebug(&*input));
                assert_eq!(deserializer.state, $state);
            )*
        }
    };
    (
        @debug
        $(#[$meta:meta])?
        $name:ident;
        $deserializer:expr;
        $deserializer_input:expr;
        ($deserializer_input_type:ty, $input_type:ty, $state_type:ty)

        $(
            $input:expr;
            $scratch:expr;
            $state:expr;
            $result:expr;
        )*
    ) => {
        $($meta:meta)?
        #[allow(unused)]
        #[test]
        fn $name() {
            use crate::read::SliceDebug;
            use serde::de::Deserialize;
            let mut input: $deserializer_input_type = $deserializer_input;
            let mut deserializer = $deserializer(input);
            let mut result;
            loop {
                result = Deserialize::deserialize(&mut deserializer);
                println!("{:?};", SliceDebug(&*deserializer.read.src));
                println!("{:?};", SliceDebug(&*deserializer.scratch));
                println!("{}::{:?};", core::any::type_name::<$state_type>(), deserializer.state);
                println!("{:?};", result);
                println!();
                if result.is_ok() { break; }
            }
            let mut input: $deserializer_input_type = $deserializer_input;
            let mut deserializer = $deserializer(input);
            $(
                result = Deserialize::deserialize(&mut deserializer);
                assert_eq!(result, $result);
                let scratch: &[u8] = $scratch;
                assert_eq!(SliceDebug(scratch), SliceDebug(deserializer.scratch.as_slice()));
                let input: $input_type = $input;
                assert_eq!(SliceDebug(&*deserializer.read.src), SliceDebug(&*input));
                assert_eq!(deserializer.state, $state);
            )*
        }
    };
    (@ignore $($ignore:tt)*) => {};
}

pub(super) use test_deserialize;
