#[cfg(test)]
use crate::filter::*;
#[cfg(test)]
use crate::Todo;
#[cfg(test)]
use crate::shared::date::Date;


#[cfg(test)]
/// Make a todo vec for use in filter testing
fn make_todo_vec() -> Vec<Todo>
{
	vec![
		// Priority.  Creation Date.
		Todo::from("x 2016-05-20 2016-04-30 Take Mr Cat to the vet! +project1 @context1"),
		Todo::from("x (A) 2016-05-24 2016-04-19 Learn how to drive and land a plane"),
		// Due Date.  Priority
		Todo::from("(C) Make sure the oven is turned off +project2 @context2 @context3 due:1969-07-20"),
		// Creation Date.  
		Todo::from("Become Napoleon and lose a land war in Russia +project3 @context3"),
	]
}

#[test]
fn test_priority()
{
	let todo_vec = make_todo_vec();
	assert!(priority(&todo_vec, 'C').len() == 1);
	assert!(priority(&todo_vec, 'B').is_empty());
}
#[test]
fn filter_due_date()
{
	let todo_vec = make_todo_vec();
	assert!(due_date(&todo_vec, Date::from("1969-07-20").unwrap()).len() == 1);
}
#[test]
fn filter_creation_date()
{
	let todo_vec = make_todo_vec();
	assert!(creation_date(&todo_vec, Date::from("2016-04-30").unwrap()).len() == 1);
}
#[test]
fn filter_project()
{
	let todo_vec = make_todo_vec();
	assert!(project(&todo_vec, "project1").len() == 1);
	assert!(project(&todo_vec, "project2").len() == 1);
	assert!(project(&todo_vec, "project3").len() == 1);
	assert!(project(&todo_vec, "project4").is_empty());
}
#[test]
fn filter_context()
{
	let todo_vec = make_todo_vec();
	assert!(context(&todo_vec, "context1").len() == 1);
	assert!(context(&todo_vec, "context2").len() == 1);
	assert!(context(&todo_vec, "context3").len() == 2);
	assert!(context(&todo_vec, "context4").is_empty());
}
#[test]
fn filter_has_tag()
{
	let todo_vec = make_todo_vec();
	assert!(has_tag(&todo_vec, "due").len() == 1);
	assert!(has_tag(&todo_vec, "fake").is_empty());
}
#[test]
fn filter_complete()
{
	let todo_vec = make_todo_vec();
	assert!(complete(&todo_vec).len() == 2);
}
#[test]
fn filter_incomplete()
{
	let todo_vec = make_todo_vec();
	assert!(incomplete(&todo_vec).len() == 2);
}
#[test]
fn fuzzy_search()
{
	let todo_vec =make_todo_vec();
	let item = super::fuzzy_search(&todo_vec, "land war", 3, 50).unwrap();
	assert_eq!(item.len(), 2);
	assert_eq!(item.first().unwrap().description, "Become Napoleon and lose a land war in Russia +project3 @context3".to_string());
}

