pub mod dictionary_profile;
pub mod ssf_format;

pub fn clean_string(input: String) -> String {
    input
        .replace(" ", "")
        .replace("\n", "")
}

pub fn remove_whitespace_suffix(mut input: String) -> String {
    input = input.replace("\n", "");
    while input.ends_with(' ') {
        input.pop();
    }
    input
}
