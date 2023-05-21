use proc_macro::{token_stream, TokenStream, TokenTree};

use crate::helpers::{expect_end, expect_punct, expect_string_ascii};

#[derive(Debug, Default)]
struct DriverInfo {
    drv_name: String,
    name: String,
    compatible: String,
}

impl DriverInfo {
    fn parse(it: &mut token_stream::IntoIter) -> Self {
        let mut info = DriverInfo::default();

        const EXPECTED_KEYS: &[&str] = &["drv_name", "name", "compatible"];
        let mut seen_keys = Vec::new();

        loop {
            let key = match it.next() {
                Some(TokenTree::Ident(ident)) => ident.to_string(),
                Some(_) => panic!("Expected Ident or end"),
                None => break,
            };

            if seen_keys.contains(&key) {
                panic!(
                    "Duplicated key \"{}\". Keys can only be specified once.",
                    key
                );
            }

            assert_eq!(expect_punct(it), ':');

            match key.as_str() {
                "drv_name" => info.drv_name = expect_string_ascii(it),
                "name" => info.name = expect_string_ascii(it),
                "compatible" => info.compatible = expect_string_ascii(it),
                _ => panic!(
                    "Unknown key \"{}\". Valid keys are: {:?}.",
                    key, EXPECTED_KEYS
                ),
            }

            assert_eq!(expect_punct(it), ',');
            seen_keys.push(key);
        }

        expect_end(it);

        let mut ordered_keys: Vec<&str> = Vec::new();
        for key in EXPECTED_KEYS {
            if seen_keys.iter().any(|e| e == key) {
                ordered_keys.push(key);
            }
        }

        if seen_keys != ordered_keys {
            panic!(
                "Keys are not ordered as expected. Order them like: {:?}.",
                ordered_keys
            );
        }

        info
    }
}

pub(crate) fn driver(ts: TokenStream) -> TokenStream {
    let mut it = ts.into_iter();
    let info = DriverInfo::parse(&mut it);

    format!(
        "
    #[used]
    #[link_section = \".init_calls\"]
    static {upper_drv_name}_ENTRY: CallEntry = CallEntry {{
        init_fn: {drv_name}_init_fn,
    }};

    fn {drv_name}_init_fn() -> Driver<'static> {{
        Driver::info(\"{name}\", \"{compatible}\")
    }}

    ",
        upper_drv_name = info.drv_name.to_uppercase(),
        drv_name = info.drv_name,
        name = info.name,
        compatible = info.compatible
    )
    .parse()
    .expect("Error parsing formatted string into token stream.")
}

// #![no_std]

// use drv_common::{CallEntry, Driver};
