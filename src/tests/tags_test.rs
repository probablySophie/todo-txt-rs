
#[cfg(test)]
use super::Tags;
	
#[test]
fn tags()
{				
	let test_tags = Tags::from("Test +string @123 one:two three:four");

	// Checking we got the values correctly
	assert_eq!(test_tags.project[0], "string");
	assert_eq!(test_tags.context[0], "123");
	assert_eq!(test_tags.to_string(), "one:two three:four");

	// Checking the project & context
	assert!(test_tags.matches_project("string"));
	assert!(! test_tags.matches_project("stringg"));
	assert!(test_tags.matches_context("123"));
	assert!(! test_tags.matches_context("1234"));

	// Checking if tags exist
	assert!(test_tags.has_tag("one"));
	assert!(! test_tags.has_tag("AAA"));

	// Getting tag values
	assert_eq!(test_tags.tag_value("three").unwrap(), "four");
	assert!(test_tags.tag_value("BBB").is_err());
}

#[test]
fn empty_tags()
{
	// Testing an empty string (if this crashes, then there's a problem)
	_ = Tags::from("");
}

