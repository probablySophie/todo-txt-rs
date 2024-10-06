use crate::todo::Todo;
use crate::shared::date::Date;

#[path = "tests/sort_test.rs"] mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Sort
{
    Ascending,
    Descending,
}


/// Sort a `todo_vec` by the `due_date` tag and return a new `Vec<Todo>`
#[must_use] pub fn due_date(todo_vec: &[Todo], sort_by: Sort) -> Vec<Todo>
{
    let mut new_todo_vec = todo_vec.to_vec();
    new_todo_vec.sort_by( |todo_a, todo_b|
        if sort_by == Sort::Descending
        {
            match (todo_a.tags.tag_value("due"), todo_b.tags.tag_value("due"))
            {
                (Ok(a),  Ok(b) ) => 
                {
                    match (Date::from(&a), Date::from(&b))
                    {
                        (Ok(c) , Ok(d) ) => {          c.cmp(&d)          },
                        (Ok(_) , Err(_)) => { std::cmp::Ordering::Greater },
                        (Err(_), Ok (_)) => { std::cmp::Ordering::Less    },
                        (Err(_), Err(_)) => { std::cmp::Ordering::Equal   },
                    }
                },
                (Ok(_)  , Err(())) => { std::cmp::Ordering::Greater },
                (Err(()), Ok (_) ) => { std::cmp::Ordering::Less    },
                (Err(()), Err(())) => { std::cmp::Ordering::Equal   },
            }
        }
        else
        {
            match (todo_a.tags.tag_value("due"), todo_b.tags.tag_value("due"))
            {
                (Ok(a),  Ok(b) ) => 
                {
                    match (Date::from(&a), Date::from(&b))
                    {
                        (Ok(c) , Ok(d) ) => {          d.cmp(&c)          },
                        (Ok(_) , Err(_)) => { std::cmp::Ordering::Less    },
                        (Err(_), Ok (_)) => { std::cmp::Ordering::Greater },
                        (Err(_), Err(_)) => { std::cmp::Ordering::Equal   },
                    }
                },
                (Ok(_)  , Err(())) => { std::cmp::Ordering::Less    },
                (Err(()), Ok (_) ) => { std::cmp::Ordering::Greater },
                (Err(()), Err(())) => { std::cmp::Ordering::Equal   },
            }            
        }
    );
    new_todo_vec
}

/// Sort a `todo_vec` by their `creation_date` and return a new `Vec<Todo>`
#[must_use] pub fn creation_date(todo_vec: &[Todo], sort_by: Sort) -> Vec<Todo>
{
    let mut new_todo_vec = todo_vec.to_vec();

    new_todo_vec.sort_by(|todo_a, todo_b|
    {
        if sort_by == Sort::Ascending
        {
            match (todo_a.creation_date, todo_b.creation_date)
            {
                (Some(a), Some(b)) => {         a.cmp(&b)           },
                (Some(_), None   ) => { std::cmp::Ordering::Greater },
                (None   , Some(_)) => { std::cmp::Ordering::Less    },
                (None   , None   ) => { std::cmp::Ordering::Equal   },
            }
        }
        else
        {
            match (todo_a.creation_date, todo_b.creation_date)
            {
                (Some(a), Some(b)) => {         b.cmp(&a)           },
                (Some(_), None   ) => { std::cmp::Ordering::Less    },
                (None   , Some(_)) => { std::cmp::Ordering::Greater },
                (None   , None   ) => { std::cmp::Ordering::Equal   },
            }
        }
    });

    new_todo_vec
}

/// Sort a `todo_vec` by their `priority` and return a new `Vec<Todo>`
#[must_use] pub fn priority(todo_vec: &[Todo], sort_by: Sort) -> Vec<Todo>
{
    let mut new_todo_vec = todo_vec.to_vec();

    new_todo_vec.sort_by(|todo_a, todo_b|
    {
        if sort_by == Sort::Descending
        {
            match (todo_a.priority, todo_b.priority)
            {
                (Some(a), Some(b)) => {          a.cmp(&b)          },
                (Some(_), None   ) => { std::cmp::Ordering::Less    },
                (None   , Some(_)) => { std::cmp::Ordering::Greater },
                (None   , None   ) => { std::cmp::Ordering::Equal   },
            }
        }
        else
        {
            match (todo_a.priority, todo_b.priority)
            {
                (Some(a), Some(b)) => {          b.cmp(&a)          },
                (Some(_), None   ) => { std::cmp::Ordering::Greater },
                (None   , Some(_)) => { std::cmp::Ordering::Less    },
                (None   , None   ) => { std::cmp::Ordering::Equal   },
            }
        }
    });

    new_todo_vec
}

// TODO: Sort by completion date
//       & make matching #[test]s

// TODO: Sort by a date range
//       & make matching #[test]s
