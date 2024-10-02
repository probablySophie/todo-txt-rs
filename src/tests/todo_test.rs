
#[cfg(test)]
use super::get_priority;

#[test]
fn empty_priority()
{
	assert!(get_priority("").is_err());
}
#[test]
fn long_priority()
{
	assert!(get_priority("(A)d").is_err());
}
#[test]
fn incorrect_form()
{
	assert!(get_priority("[B]").is_err());
}
#[test]
fn correct_priority()
{
	assert_eq!(get_priority("(D)"), Ok('D'));
}

#[test]
fn test_description()
{
	use super::get_description;

	// Empty Description
	assert_eq!(
		get_description(""), 
		String::new()
	);
	// Just text
	assert_eq!(
		get_description("Why hello there my friend"), 
		"Why hello there my friend".to_string()
	);
	// With two valid extra tags
	assert_eq!(
		get_description("Wow, what a nice dog plan:stealTheDog t:1969-07-20"), 
		"Wow, what a nice dog".to_string()
	);	
}
