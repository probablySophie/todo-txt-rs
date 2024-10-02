use std::fmt;

#[path = "tests/tags_test.rs"] mod test;

#[derive(Default, Clone)]
pub struct Tags
{
	pub project: Vec<String>,
	pub context: Vec<String>,
	pub custom: Vec<(String, String)>,
}

impl fmt::Display for Tags
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        // We can discard the project & context as that is included in the description
        // So we're only actually printing the custom tags

        let tag_string: String = self.custom.clone()
            .into_iter()
            .map(|(tag, value)| { tag + ":" + &value + " " }) // Stringify the tag:value
            .collect::<String>()
        	.trim_end()
        	.to_string();
        
        write!(f, "{tag_string}")
    }
}

impl Tags
{
    pub fn matches_project(&self, project: &str) -> bool
    {
        for item in self.project.clone()
        {
            if item == project
            {
                return true
            }
        }
        // Else
        false
    }
    pub fn matches_context(&self, context: &str) -> bool
    {
        for item in self.context.clone()
        {
            if item == context
            {
                return true
            }
        }
        // Else
        false
    }
    pub fn has_tag(&self, tag: &str) -> bool
    {
        for (custom_tag, _value) in self.custom.clone()
        {
            if custom_tag == tag
            {
                return true
            }
        }
        // Else
        false
    }
    pub fn tag_value(&self, tag: &str) -> Result<String, ()>
    {
        for (custom_tag, value) in self.custom.clone()
        {
            if custom_tag == tag
            {
                return Ok(value)
            }
        }
        // Else
        Err(())
    }
    
    
	pub fn from(string: &str) -> Tags
	{
		let mut tags = Tags::default();

		let items = string.split(' ');

		for item in items
		{
			// sexy little bit of safety
			if !item.is_empty()
			{
				// Check the first char, the string isn't empty so there'll 100% be one
				match item.chars().next().unwrap()
				{
					// +ProjectTag
					'+' => {
						tags.project.push(item[1..].to_string());
					},
					// @contextTag
					'@' => {
						tags.context.push(item[1..].to_string());
					},
					// Else
					_ => {
						let colon_index = item.find(':');

						// Is the item a tag?
						if colon_index.is_some()
						{
							let colon_index = colon_index.unwrap();

							let pre_string = item[0..colon_index].to_string();
							let post_string = item[colon_index+1..].to_string();	

							// ✨ safety ✨
							if pre_string.is_empty() || post_string.is_empty()
							{
								continue // skip if either pre:post is empty
							}

							tags.custom.push(
								(
									pre_string,
									post_string
								)
							);
						}
					}
				}
			}
		}
		tags // return tags
	}
}

