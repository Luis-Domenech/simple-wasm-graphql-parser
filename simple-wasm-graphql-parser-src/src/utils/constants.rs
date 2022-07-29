use lazy_static::lazy_static;
use fancy_regex::Regex;
use crate::prelude::RegexPattern;

lazy_static! {
    pub static ref REGEX: RegexPattern = RegexPattern {
        global_match_words: Regex::new(r"(?m)\w+").unwrap(),
        global_match_comments: Regex::new(r"(?m)([^:]|^\s+)#.*$").unwrap(),
        global_match_description_with_quotes: Regex::new("(?m)[\"]{3,}[\\s\\S]*?[\"]{3,}").unwrap(),
        global_match_carriage_return_and_new_line: Regex::new(r"(?m)(\r\n)+").unwrap(),
        match_comments_before_type: Regex::new(r"([\s\S]*[#]+[^\n]+)").unwrap(),
        match_complete_type: Regex::new(r"[\w\[\]!]+").unwrap(),
        match_exclamations: Regex::new(r"[!]+").unwrap(),
        match_closing_brackets: Regex::new(r"(])+").unwrap(),
        match_brackets_and_exclamation: Regex::new(r"[\[\]!]+").unwrap(),
        match_description_beginning_end_quotes: Regex::new("(^\\s*[\"]+[\\s\\r\\n]*)|(\\s*[\"]+[\\s\\r\\n]*$)").unwrap(),
        match_comment_at_line_end: Regex::new(r"[#].*$").unwrap(),
        match_all_directives_with_parenthesis: Regex::new("(@{1}\\w+\\s*[(]+[\\s\\r\\n\\w:\"{$}]+[)]+)").unwrap(),
        match_directive_name: Regex::new(r"@\w+").unwrap(),
        match_whitespace: Regex::new(r"\s+").unwrap(),
        match_on_word: Regex::new(r"(\s*(on)+\s+)+").unwrap(),
        match_first_word: Regex::new(r"[^\s][\w]+").unwrap(),
        match_word: Regex::new(r"[\w]+").unwrap(),
        match_single_quote_description: Regex::new("(?<=\\s)[\"][\\w+\\s+]+[\"](?:\\s)").unwrap(),
        match_all_parenthesis: Regex::new(r"[)()]+").unwrap(),
        match_field_name: Regex::new(r"\w+(?=[\s\r\n:])").unwrap(),
        match_directive_argument_value: Regex::new("(?<=:\\s)[\"\\[\\]\\s\\S]+").unwrap()
    };

}

pub const PACKAGE_NAME: &str = "wasm-graphql-parser";
pub const PRETTY_PACKAGE_NAME: &str = "WASM GraphQL Parser";