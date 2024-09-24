use crate::todo::Todo;
use crate::shared::date::Date;

/*
    Matches / Has
*/

// This macro stops us from having to write an iter & collector for each and every function :)
macro_rules! filter_vec {
    ($todo_vec:expr, $filter_code:block) => {
        $todo_vec.to_owned().into_iter().filter( $filter_code ).collect()
    };
}

pub fn priority(todo_vec: &Vec<Todo>, priority: char) -> Vec<Todo>
{	
	filter_vec!(todo_vec, { |todo| todo.priority.is_some_and(|p| p == priority ) })
}

pub fn due_date(todo_vec: &Vec<Todo>, due_date: Date) -> Vec<Todo>
{
	let date_string = due_date.to_string();
	filter_vec!(todo_vec, {|todo| todo.tags.tag_value("due").is_ok_and(|due| due == date_string) })
}

pub fn creation_date(todo_vec: &Vec<Todo>, creation_date: Date) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.creation_date == Some(creation_date) })
}

// TODO: Filter by completion date
// THEN: make a test for it

pub fn project(todo_vec: &Vec<Todo>, project: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.tags.matches_project(project) })
}

pub fn context(todo_vec: &Vec<Todo>, context: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.tags.matches_context(context) })
}

pub fn has_tag(todo_vec: &Vec<Todo>, tag: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, { |todo| todo.tags.has_tag(tag) })
}

pub fn complete(todo_vec: &Vec<Todo>) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.complete })
}

pub fn incomplete(todo_vec: &Vec<Todo>) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| !todo.complete })
}





#[cfg(test)]
mod test
{
	use super::*;
	use crate::Todo;
	use crate::shared::date::Date;

	fn make_todo_vec() -> Vec<Todo>
	{
		vec![
			// Priority.  Creation Date.
			Todo::from("x 2016-05-20 2016-04-30 description description +project1 @context1"),
			Todo::from("x (A) 2016-05-24 2016-04-19 description description"),
			// Due Date.  Priority
			Todo::from("(C) description description +project2 @context2 @context3 due:1969-07-20"),
			// Creation Date.  
			Todo::from("description +project3 @context3"),
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
}

