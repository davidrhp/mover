use crate::config::tests::test_config;

use super::*;

#[test]
fn list_action_ok() {
    let cfg = test_config();
    let loc = cfg.locations.get(0).unwrap();
    let expected_name = &loc.name;
    let expected_url = format!("{}", loc.url);
    let mut writer = Vec::new();

    assert!(list_action(&mut writer, &cfg).is_ok());

    find_subslice(&writer, expected_url.as_bytes())
        .unwrap_or_else(|| panic!("list action did not write the name '{}' to its writer. \
            Writer content: \n{}\n", expected_name, std::str::from_utf8(&writer).unwrap()));

    find_subslice(&writer, expected_url.as_bytes())
        .unwrap_or_else(|| panic!("list action did not write the url '{}' to its writer. \
            Writer content: \n{}\n", expected_url, std::str::from_utf8(&writer).unwrap()));
}


fn find_subslice<'a>(slice: &'a [u8], subslice: &[u8]) -> Option<&'a [u8]> {
    slice.windows(subslice.len())
        .find(|window| *window == subslice)
}

