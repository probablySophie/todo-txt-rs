#[cfg(test)]

#[test]
fn empty_todo()
{
    use crate::Todo;

    let todo = Todo::from("");
    
    assert_eq!(todo.to_string(), "");
}

// TODO: more testing of the Todo struct

/*
   TODO: Testing
   
   * input:  Valid Todo item string
   * action: Input into the program & then output back out
   * result: The same string

   
*/

