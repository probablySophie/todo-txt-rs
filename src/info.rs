use crate::todo::Todo;

#[path = "tests/info_test.rs"] mod test;

/// From a `Vec<Todo>` get all the existing project names
/// Returns a `Vec<String>` containing the project names
/// This function is **inefficient** and is meant to be used *once* per session, please store the returned `Vec<String>` if you intend to use it again
pub fn get_projects(todo_vec: &Vec<Todo>) -> Vec<String>
{
	let mut projects: Vec<String> = Vec::new();

	// For each item in the input Vec
	for todo in todo_vec
	{
		// And each project the current item has
		for project in &todo.tags.project
		{
			// If the current project is in our projects list
			if projects.clone().into_iter().any(|p| p == project.clone() )
			{
				// It's already there.  Skip
			}
			else
			{
				projects.push(project.clone());
			}
		}
	}
	
	projects // Return projects
}

/// From a `Vec<Todo>` get all the existing context names
/// Returns a `Vec<String>` containing the context names
/// This function is **inefficient** and is meant to be used *once* per session, please store the returned `Vec<String>` if you intend to use it again
pub fn get_contexts(todo_vec: &Vec<Todo>) -> Vec<String>
{
	let mut contexts: Vec<String> = Vec::new();

	// For each item in the input Vec
	for todo in todo_vec
	{
		// And each project the current item has
		for context in &todo.tags.context
		{
			// If the current project is in our projects list
			if contexts.clone().into_iter().any(|c| c == context.clone() )
			{
				// It's already there.  Skip
			}
			else
			{
				contexts.push(context.clone());
			}
		}
	}
	
	contexts // Return projects
}

