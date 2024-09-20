
use crate::todo::Todo;

// TODO: Lua plugins?!?!

/*
    t:YYYY-MM-DD    Tasks with a future starting date
    due:YYYY-MM-DD  Due date
    repeats:1day    Recreate the task on completion
    completion:2of5 Tasks that must be done multiple times
    requires:abc123 Tasks that cannot be completed before a specific other task
    id:abc213       An ID so other tasks can target it
*/

pub fn on_completion(todo: Todo) -> Todo
{
    // TODO: 

    todo // return todo
}

pub fn on_creation(todo: Todo) -> Todo
{
    // TODO:

    todo // return todo
}

