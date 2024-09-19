use crate::shared::date::Date;
use std::fmt;

use crate::tags::Tags;


pub struct Todo
{
	complete: bool,
	priority: Option<char>,
	creation_date: Option<Date>,
	finished_date: Option<Date>,
	description: String,
	tags: Tags,
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
		if ! description.ends_with(' ')
		{
			description += " "; // tack one on
		}

		let tags = self.tags.to_string();
		
		write!(f, "{complete_string}{priority_string}{completion_date}{creation_date}{description}{tags}")
	}
}
impl Todo
{
	pub fn from(string: &str) -> Todo
	{
		// Using the specifications from:
		// https://github.com/todotxt/todo.txt

		let mut string_chars = string.chars();

		// defaulting everything to negative or None
		let mut priority: Option<char> = None;
		let mut creation_date: Option<Date> = None;
		let mut finished_date: Option<Date> = None;
		let mut description: String = String::new();
		let mut tags = Tags::default();

		let mut index = 0;

		// Is the task item complete?
		let complete = string[0..2] == *"x ";
		if complete 
		{ 
			index += 2;
		};

		// Does the task item have a priority?
		// INFO: If the task is complete, then it almost certainly won't have a priority...
		//       any more, but we'll check anyway, just incase
		if string_chars.nth(index).unwrap() == '(' && string_chars.nth(index+2).unwrap() == ')'
		{
			// If yes, set it
			priority = string_chars.nth(index+1);
			index += 4; // including the space
		}

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

		// Split the description by spaces
		let items = string[index .. ].split(' ');
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

		description = items
			.take(i) // Get the first i number of items
			.map(|s| {s.to_owned() + " "}) // for each item, give them their space back
			.collect::<String>(); // collect the items into a String

		// Get tags from the description & custom tags
		tags = Tags::from( &string [index .. ] );
		
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

