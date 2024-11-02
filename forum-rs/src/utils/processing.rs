use crate::globals;
use crate::utils;

use rayon::prelude::*;
/// Text cleaning function, can be changed.
///
/// Note that RE is static and should be changed above
///
/// * `text` - A string reference
///
/// # Example
///
/// ```
/// // This clearly does nothing, but just shows the usage
/// let text = "hello world".to_string();
/// let cleaned_text = clean_text(&text);
/// assert_eq!(cleaned_text, "hello world");
/// ```
fn clean_text(text: String) -> String {
    let cleaned_text = globals::RE.replace_all(&text, " ");
    let cleaned_text = globals::RE2.replace_all(&cleaned_text, " ");
    cleaned_text.trim().to_string()
}

pub fn process(
    thread_id: String,
    content: Vec<String>,
    forum_name: String,
    use_sentencepiece: bool,
) -> utils::writer::ThreadPost {
    let content: Vec<String> = content
    .into_par_iter()
    .with_min_len(100)
    .map(clean_text)
    .collect();
    let content = content.join("\n");
    let length: usize = match use_sentencepiece {
        true => globals::TOKENIZER
            .get()
            .unwrap()
            .encode(content.as_str(), false)
            .unwrap()
            .len(),
        false => content.split_whitespace().count(),
    };
    utils::writer::ThreadPost {
        length,
        raw_content: content,
        thread_id,
        source: forum_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let test_cases = vec![
            // Test dashes
            ("hello--world", "hello world"),
            ("multiple---dashes", "multiple dashes"),
            ("normal-dash", "normal-dash"), // single dash should remain
            // Test equals signs
            ("title==heading", "title heading"),
            ("multiple===equals", "multiple equals"),
            ("single=equals", "single=equals"), // single equals should remain
            // Test multiple spaces
            ("too    many    spaces", "too many spaces"),
            ("normal spaces", "normal spaces"),
            ("tabs\t\tand    spaces", "tabs and spaces"),
            // Test URLs
            ("check http://example.com here", "check here"),
            ("https://website.com/path", ""),
            ("mixed http://url.com and text", "mixed and text"),
            // Test @ mentions
            ("hello @username world", "hello world"),
            ("@user1 @user2 text", "text"),
            ("email@domain.com text", "text"),
            // Test hashtags
            ("#Hashtag5, #Hashtag2, #Hashtag  ", ""),
            // Test combinations
            ("@user http://example.com  ---separator", "separator"),
            (
                "complex   http://example.com   @user   case",
                "complex case",
            ),
            (
                "shouldn't #SPAM https://spam.com remove someone@spam.com this",
                "shouldn't remove this",
            ),
            // Edge cases
            ("", ""),    // empty string
            ("   ", ""), // only spaces
            ("---", ""), // only dashes
            ("===", ""), // only equals
        ];

        for (input, expected) in test_cases {
            let result = utils::processing::clean_text(input.to_string());
            println!("{} -> {}", input, result);
            assert_eq!(
                result, expected,
                "Failed on input: '{}'\nExpected: '{}'\nGot: '{}'",
                input, expected, result
            );
        }
    }
}
