//! # TODO-txt
//! 
//! TODO-txt is a rust crate that follows the [todo.txt](https://github.com/todotxt/todo.txt) standard for creating and saving TODO items.
//! This crate has some additional functionality on top of the base standard aimed at making implementing the crate into an app a far simpler process, including filtering, sorting, and more.

#![allow(dead_code)]

mod shared;

mod todo;
pub use todo::*;
mod tags;

mod info;
mod sort;
mod filter;
//mod addons;

#[path = "tests/lib_tests.rs"] mod test;

/*
   The Todo.txt documentation: https://github.com/todotxt/todo.txt
*/
#[must_use] // the caller of this function MUST USE the returned result
/// Makes a `Vec<Todo>` from a given &str.
/// The input &str is assumed to be multi-line and will be split as such
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
/// Creates a `String` from a given `Vec<Todo>`
/// This function is intended to be used for exporting & saving between active sessions
pub fn flatten_vec(todo_vec: Vec<todo::Todo>) -> String
{
    let mut string = String::new();
    
    for todo in todo_vec
    {
        string += &(todo.to_string() + "\n");
    }
    // Remove the last newline :)
    string.pop();
    
    string // return string
}
