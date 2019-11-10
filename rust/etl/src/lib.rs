use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(value, letters)| {
            letters
                .iter()
                .map(move |letter| (letter.to_ascii_lowercase(), *value))
        })
        .collect()
}
