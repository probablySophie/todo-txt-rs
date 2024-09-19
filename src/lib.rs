
mod shared;

mod todo;
mod tags;

/*
   The Todo.txt documentation: https://github.com/todotxt/todo.txt
  
  TODO: Spin off our own super simple shared-rs library
  * Include Date, Vector
*/


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
   TODO: Testing
   
   * input:  Valid Todo item string
   * action: Input into the program & then output back out
   * result: The same string

   
*/
