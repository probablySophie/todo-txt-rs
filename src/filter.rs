use crate::todo::Todo;
use crate::shared::date::Date;
use crate::shared::trigrams::get_similarity;

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

/// Filter a `Vec<Todo>` by a given priority
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn priority(todo_vec: &Vec<Todo>, priority: char) -> Vec<Todo>
{	
	filter_vec!(todo_vec, { |todo| todo.priority.is_some_and(|p| p == priority ) })
}

/// Filter a `Vec<Todo>` by a given due date
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn due_date(todo_vec: &Vec<Todo>, due_date: Date) -> Vec<Todo>
{
	let date_string = due_date.to_string();
	filter_vec!(todo_vec, {|todo| todo.tags.tag_value("due").is_ok_and(|due| due == date_string) })
}

/// Filter a `Vec<Todo>` by a given creation date
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn creation_date(todo_vec: &Vec<Todo>, creation_date: Date) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.creation_date == Some(creation_date) })
}

// TODO: Filter by completion date
// THEN: make a test for it

/// Filter a `Vec<Todo>` by a given project name
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn project(todo_vec: &Vec<Todo>, project: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.tags.matches_project(project) })
}

/// Filter a `Vec<Todo>` by a given context item name
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn context(todo_vec: &Vec<Todo>, context: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.tags.matches_context(context) })
}

/// Filter a `Vec<Todo>` for Todo items that have a given tag identifier
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn has_tag(todo_vec: &Vec<Todo>, tag: &str) -> Vec<Todo>
{
	filter_vec!(todo_vec, { |todo| todo.tags.has_tag(tag) })
}

/// Filter a `Vec<Todo>` for only complete items
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn complete(todo_vec: &Vec<Todo>) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| todo.complete })
}

/// Filter a `Vec<Todo>` for only incomplete items
/// Returns a new filtered `Vec<Todo>`
#[must_use] pub fn incomplete(todo_vec: &Vec<Todo>) -> Vec<Todo>
{
	filter_vec!(todo_vec, {|todo| !todo.complete })
}


/// Perform a trigram search on the input `Vec<Todo>`'s descriptions
/// The search text does not need to be capitalised
/// `min_assurity` should be a percentage number between 0 - 100
///
/// # Errors
///
/// - If unable to find ANY `Todo` items that fit the search text within the given `min_assurity`
pub fn fuzzy_search(todo_vec: &[Todo], search_text: &str, num_items: usize, min_assurity: i32) -> std::io::Result<Vec<Todo>>
{
	let mut closest: Vec<(usize, i32)> = Vec::new();

	for (i, todo_item) in todo_vec.iter().enumerate()
	{
		let similarity = get_similarity(&todo_item.description, search_text);

		// Does the similarity fall within our allowable range?
		if min_assurity < similarity
		{
			continue // no. Skip
		}
		// If closest is empty, then what we have is by default, the closest
		if closest.is_empty()
		{
			closest.push( (i, similarity) );
			continue
		}
		// Make sure there is a last (there obviously is if we've reached this point .-.)
		if let Some(lowest_similarity) = closest.last()
		{
			// If our new guy is more similar than the lowest current similarity
			// OR we have fewer items to return than the caller asked for
			if similarity > lowest_similarity.1 || closest.len() < num_items
			{
				// Add our new guy
				closest.push( (i, similarity) );

				// Sort by similarity
				closest.sort_by(|(_,a), (_,b)| {a.cmp(b) } );

				// If we NOW have more items then was requested
				if closest.len() > num_items
				{
					closest.remove(0); // Murder the least close
				}
			}
		}
	}

	// If we've gotten this far and we haven't found anything then throw a tantrum
	if closest.is_empty()
	{
		return Err(
			std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Unable to find a Todo with the searched string"
			)
		)
	}
	// Else we're happy and will want to return a Vec<Todo>
	let mut return_vec = Vec::new();
	// For each of the closest items
	for (i, _) in closest
	{
		// Add a copy of the real Todo item to our return_vec
		return_vec.push( todo_vec[i].clone() );
	}

	Ok(return_vec)
}
