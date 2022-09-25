// get a precompiled vector of english words
pub fn text() -> Vec<String> {
    let pre_compile_english_text = include_str!("../english-dictionary.txt");
    pre_compile_english_text
        .split(";")
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod english_dict_tests {
    use super::*;

    // test compile time text from english dict
    #[test]
    fn precompile_english_dictionary() {
        assert_eq!(text().len() > 0, true);
    }
}
