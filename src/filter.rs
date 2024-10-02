use crate::todo::Todo;
use crate::shared::date::Date;

#[path = "tests/filter_test.rs"] mod test;

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
