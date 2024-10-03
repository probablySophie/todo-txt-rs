
#[cfg(test)]
use super::*;

#[test]
fn due_date()
{
	//
}

#[cfg(test)]
fn creation_date_vec() -> Vec<Todo>
{
	crate::bulk_create(
"1969-07-15 Land on the moon +Apollo
2021-03-22 Feed the baby Lizard some crickets @lizarrrd
2020-11-04 Buy some crickets for the baby Lizard @lizarrrd")
}

#[test]
fn creation_date_ascending()
{
	let todo_vec = creation_date_vec();
	let sorted_asc  = super::creation_date(&todo_vec, Sort::Ascending );

	assert_eq!(crate::flatten_vec(sorted_asc ),
"1969-07-15 Land on the moon +Apollo
2020-11-04 Buy some crickets for the baby Lizard @lizarrrd
2021-03-22 Feed the baby Lizard some crickets @lizarrrd");
}

#[test]
fn creation_date_descending()
{
	let todo_vec = creation_date_vec();
	let sorted_desc = super::creation_date(&todo_vec, Sort::Descending);
	assert_eq!(crate::flatten_vec(sorted_desc),
"2021-03-22 Feed the baby Lizard some crickets @lizarrrd
2020-11-04 Buy some crickets for the baby Lizard @lizarrrd
1969-07-15 Land on the moon +Apollo");
}

#[cfg(test)]
fn priority_vec() -> Vec<Todo>
{
	crate::bulk_create(
"Do some work +dailyTasks
(A) Make a coffee +dailyTasks
(B) Work on +dailyTasks
(C) Go to bed +dailyTasks")
}

#[test]
fn priority_ascending()
{
	let todo_vec = priority_vec();
	let sorted_asc  = super::priority(&todo_vec, Sort::Ascending );

	assert_eq!(crate::flatten_vec(sorted_asc ), 
"Do some work +dailyTasks
(C) Go to bed +dailyTasks
(B) Work on +dailyTasks
(A) Make a coffee +dailyTasks");

}

#[test]
fn priority_descending()
{
	let todo_vec = priority_vec();
	let sorted_desc = super::priority(&todo_vec, Sort::Descending);
	
	assert_eq!(crate::flatten_vec(sorted_desc), 
"(A) Make a coffee +dailyTasks
(B) Work on +dailyTasks
(C) Go to bed +dailyTasks
Do some work +dailyTasks");
}
