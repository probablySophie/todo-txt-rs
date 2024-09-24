#[cfg(test)]
use crate::Todo;
#[cfg(test)]
use crate::shared::date::Date;


#[test]
fn empty_todo()
{
    let todo = Todo::from("");
    
    assert_eq!(todo.to_string(), "");
}

#[test]
fn full_todo()
{
	let from_str: &str = "x (R) 2024-03-14 2000-01-01 Test description +project @context tag:onetwo";
	
	let todo = Todo::from(from_str);

	assert!(todo.complete);
	assert!(todo.priority == Some('R'));
	
	assert!(todo.completion_date.is_some_and(|d| d == Date::from("2024-03-14").unwrap()));
	assert!(todo.creation_date.is_some_and(|d| d == Date::from("2000-01-01").unwrap()));

	assert!(todo.tags.matches_project("project"));
	assert!(todo.tags.matches_context("context"));
	
	assert!(todo.tags.has_tag("tag"));
	assert_eq!(todo.tags.tag_value("tag"), Ok("onetwo".to_string()));

	assert_eq!(todo.to_string(), from_str);
	
}

// TODO: more testing of the Todo struct

/*
   TODO: Testing
   
   * input:  Valid Todo item string
   * action: Input into the program & then output back out
   * result: The same string

   
*/

