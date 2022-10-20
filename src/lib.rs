use dictionary_profile::NEWLINE;

pub mod dictionary_profile;
pub mod sff_format;

pub fn clean_string(input: String) -> String {
    input
        .replace(" ", "")
        .replace(NEWLINE, "")
        .replace("\t", "")
}

pub fn remove_whitespace_suffix(mut input: String) -> String {
    input = input.replace(NEWLINE, "")
            .replace("\t", "");
    while input.ends_with(' ') {
        input.pop();
    }
    input
}
