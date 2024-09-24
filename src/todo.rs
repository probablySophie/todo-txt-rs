use crate::shared::date::Date;
use std::fmt;

use crate::tags::Tags;

#[derive(Default, Clone)]
pub struct Todo
{
	pub complete: bool,
	pub priority: Option<char>,
	pub creation_date: Option<Date>,
	pub finished_date: Option<Date>,
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

		let completion_date = if self.finished_date.is_some() {
			self.finished_date.unwrap().to_string() + " "
		} else {String::new()};

		let creation_date = if self.creation_date.is_some() {
			self.creation_date.unwrap().to_string() + " "
		} else {String::new()};

		let mut description = self.description.clone();
		// If there isn't a space at the end of the description
		// (and there is actually a description)
		if ! description.ends_with(' ') && ! description.is_empty()
		{
			description += " "; // tack one on
		}

		let tags = self.tags.to_string();
		
		write!(f, "{complete_string}{priority_string}{completion_date}{creation_date}{description}{tags}")
	}
}
impl Todo
{
	pub fn complete(&mut self) -> Result<(), ()>
	{
		if self.complete
		{
			return Err(())
		}
		// Else

		// TODO: If addons::on_complete(task), then continue, else, its actually not complete
		
		self.complete = true;

		if let Ok(today) = Date::today()
		{
			self.finished_date = Some(today);
		}
		else
		{
			return Err(())
		}
		Ok(())
		
}

	#[must_use]
	pub fn from(string: &str) -> Todo
	{
		if string.len() < 2
		{
			return Todo::default();
		}
		
		// Using the specifications from:
		// https://github.com/todotxt/todo.txt

		// defaulting everything to negative or None
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
				// yes :)
				finished_date = Some(parsed_date);
				index += 11; // include the space after the date
			}
		}
		
		// Is there a creation date?
		if let Ok(parsed_date) = Date::from(&string[index..index+10])
		{
			creation_date = Some(parsed_date);

			index += 11; // including the space
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
				finished_date,
				description,
				tags
			}
		//)
	}
}


fn get_priority(priority_slice: &str) -> Result<char, &str>
{
    if priority_slice.len() != 3
    {
        return Err("Given string is incorrect length.  Length must be 3")
    }

    if ! (priority_slice.starts_with("(") && priority_slice.ends_with(")") )
    {
        return Err("Priority must be in the form '(X)'")
    }

	let priority_char = priority_slice.chars().nth(1);
	if priority_char.is_none() {return Err("???")}
	let priority_char = priority_char.unwrap();

	// TODO: Check if [1] is a letter
    if priority_char.is_alphanumeric()
    {
        return Ok(priority_char)
    }

    Err("Priority needs to be alphanumeric")
}

fn get_description(description_string: &str) -> String
{
		// Split the description by spaces
		let items = description_string.split(' ');
		let mut i = items.clone().count(); // count how many items are in the split

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
			i -= 1; // decrement i because I couldn't make enumerate behave
		}
		// return
		items
			.take(i)                      // Get the first i number of items
			.map(|s| {s.to_owned() + " "})// for each item, tack on a space
			.collect::<String>()          // collect the items into a String
			.trim_end()                   // remove any trailing whitespace
			.to_string()                  // we want it as a String again
}


#[cfg(test)]
mod test
{
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
}
