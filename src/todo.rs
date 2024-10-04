use crate::shared::date::Date;
use std::fmt;

#[path = "tests/todo_test.rs"] mod test;

use crate::tags::Tags;

#[derive(Default, Clone, Debug)]
pub struct Todo
{
	pub complete: bool,
	pub priority: Option<char>,
	pub creation_date: Option<Date>,
	pub completion_date: Option<Date>,
	pub description: String,
	pub tags: Tags,
}
impl fmt::Display for Todo
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let complete_string = if self.complete {"x "} else {""};

		let priority_string = if self.priority.is_some() {
			"(".to_owned() + &self.priority.unwrap().to_string() + ") "
		} else {String::new()};

		let completion_date = if self.completion_date.is_some() {
			self.completion_date.unwrap().to_string() + " "
		} else {String::new()};

		let creation_date = if self.creation_date.is_some() {
			self.creation_date.unwrap().to_string() + " "
		} else {String::new()};

		let mut description = self.description.clone();
		
		let tags = self.tags.to_string();
		
		// If there actually are tags and there isn't a space at the 
		// end of the description (and there actually is a description)
		if ! tags.is_empty() && ! description.ends_with(' ') && ! description.is_empty()
		{
			description += " "; // Tack one on
		}


		write!(f, "{complete_string}{priority_string}{completion_date}{creation_date}{description}{tags}")
	}
}
impl Todo
{
	/// Mark the `Todo` item as complete
	/// 
	/// # Errors
	/// - Will return `Err` if already complete, or if unable to get `Date::today()`
	pub fn complete(&mut self) -> Result<bool, &str>
	{
		if self.complete
		{
			return Err("Already Complete")
		}
		// Else

		// TODO: If addons::on_complete(task), then continue, else, its actually not complete
		
		self.complete = true;

		if let Ok(today) = Date::today()
		{
			self.completion_date = Some(today);
		}
		else
		{
			return Err("Unable to get Date::today()")
		}
		Ok(self.complete)
}

	/// Make a new `Todo` item from a given `&str`
	/// Breaks the string up as per the todo.txt standard
	/// The standard: `https://github.com/todotxt/todo.txt`
	#[must_use]
	pub fn from(string: &str) -> Todo
	{
		let string = string.trim();
		if string.len() < 2
		{
			return Todo::default();
		}
		
		// Using the specifications from:
		// https://github.com/todotxt/todo.txt

		// Defaulting everything to negative or None
		let mut creation_date: Option<Date> = None;
		let mut finished_date: Option<Date> = None;

		let mut index = 0;

		// Is the task item complete?
		let complete = string[0..2] == *"x ";
		// If yes, increment Index
		if complete { index += 2 };


		let priority: Option<char> = if let Ok(returned_priority) = get_priority(&string[index..index+3])
		{
			Some(returned_priority)
		}
		else
		{
			None
		};

		// And increment Index if we got a priority
		if priority.is_some() { index += 4 };


		// INFO: Based on the example description.svg from the docs, it goes complete -> priority -> completion date
		// If the task item is complete, is there a completion date?
		if complete
		{
			if let Ok(parsed_date) = Date::from(&string[index..index+10])
			{
				// Yes :)
				finished_date = Some(parsed_date);
				index += 11; // Include the space after the date
			}
		}
		
		// Is there a creation date?
		if let Ok(parsed_date) = Date::from(&string[index..index+10])
		{
			creation_date = Some(parsed_date);

			index += 11; // Including the space
		}

		// Technically everything left over is the description according to...
		// (the explainer image in) the docs, but that's kind of ugly and awful :(

		let description = get_description(&string[index .. ]);

		// Get tags from the description & custom tags
		let tags = Tags::from( &string [index .. ] );
		
		//Ok(
			Todo
			{
				complete,
				priority,
				creation_date,
				completion_date: finished_date,
				description,
				tags
			}
		//)
	}
}

/// Private function: get a priority char from a given 3 char slice
fn get_priority(priority_slice: &str) -> Result<char, &str>
{
    if priority_slice.len() != 3
    {
        return Err("Given string is incorrect length.  Length must be 3")
    }

    if ! (priority_slice.starts_with('(') && priority_slice.ends_with(')') )
    {
        return Err("Priority must be in the form '(X)'")
    }

	let priority_char = priority_slice.chars().nth(1);
	if priority_char.is_none() {return Err("???")}
	let priority_char = priority_char.unwrap();

    if priority_char.is_alphanumeric()
    {
        return Ok(priority_char)
    }

    Err("Priority needs to be alphanumeric")
}

/// Private function: Get a description string from a given (valid) Todo.txt input string
fn get_description(description_string: &str) -> String
{
		// Split the description by spaces
		let items = description_string.split(' ');
		let mut i = items.clone().count(); // Count how many items are in the split

		// Go backwards through the items
		for item in items.clone().rev()
		{
			// If the item ISN'T a tag
			if item.find(':').is_none()
			{
				// Break out of the for loop because we're done
				break;
			}
			// Else, it IS a tag, so keep going
			i -= 1; // Decrement i because I couldn't make enumerate behave
		}
		// Return
		items
			.take(i)                      // Get the first i number of items
			.map(|s| {s.to_owned() + " "})// For each item, tack on a space
			.collect::<String>()          // Collect the items into a String
			.trim_end()                   // Remove any trailing whitespace
			.to_string()                  // We want it as a String again
}

