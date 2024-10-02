#![allow(dead_code)]

mod shared;

mod todo;
pub use todo::*;
mod tags;

//mod sort;
mod filter;
//mod addons;

#[path = "tests/main.rs"] mod test;

/*
   The Todo.txt documentation: https://github.com/todotxt/todo.txt
*/
#[must_use] // the caller of this function MUST USE the returned result
pub fn bulk_create(string: &str) -> Vec<todo::Todo>
{
    let mut todo_vec = Vec::new();
    let lines = if string.contains("\r\n") // If windows...
                {
                    string.split("\r\n") // search for the carriage return
                }
                else // else
                {
                    #[allow(clippy::single_char_pattern)] // shut up clippy
                    string.split("\n") // be normal
                };

    for line in lines
    {
        // ignore empty lines
        if ! line.is_empty()
        {
            todo_vec.push(todo::Todo::from(line));
        }
    }

    todo_vec // return to_return
}


#[must_use]
pub fn flatten_vec(todo_vec: Vec<todo::Todo>) -> String
{
    let mut string = String::new();
    
    for todo in todo_vec
    {
        string += &(todo.to_string() + "\n");
    }

    string // return string
}
