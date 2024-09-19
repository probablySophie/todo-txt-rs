#[cfg(test)]

#[test]
fn it_works()
{
    use crate::Todo;

    let todo = Todo::from("");
    
    assert_eq!(todo.to_string(), "");
}


/*
   TODO: Testing
   
   * input:  Valid Todo item string
   * action: Input into the program & then output back out
   * result: The same string

   
*/

