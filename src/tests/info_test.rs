
#[cfg(test)]
use super::*;

#[cfg(test)]
fn make_test_vec() -> Vec<Todo>
{
	crate::bulk_create("
x Task one description +Project1 @context1 @context2
Task two description +Project1 @context2
(A) Task three description +Project2 @context3
2020-01-01 Task four description +Project3 @context3
")
}

#[test]
fn get_projects()
{
	let todo_vec = make_test_vec();
	let mut flattened = String::new();

	for project in super::get_projects(&todo_vec)
	{
		flattened += &(project + ", ");
	}
	
	assert_eq!(flattened, "Project1, Project2, Project3, ");
}

#[test]
fn get_contexts()
{
	let todo_vec = make_test_vec();
	let mut flattened = String::new();

	for context in super::get_contexts(&todo_vec)
	{
		flattened += &(context + ", ");
	}
	
	assert_eq!(flattened, "context1, context2, context3, ");
}
