# todo-txt-rs

Aiming to follow the [`todo.txt` standard](https://github.com/todotxt/todo.txt), but in Rust.  



****

#### The Plan

- [ ] Get the base stuff *working*  
- [ ] `Todo.complete()` & have it pull in the date of completion
- [ ] Bulk import!  (take a text block, break by lines, read each in)
- [ ] Add some fun add-ons & tags
    - [ ] Future tasks! `t:YYYY-MM-DD` (future starting date)
	- [ ] Repeating tasks! `repeats:1day`
	- [ ] Due date! `due:YYYY-MM-DD`
	- [ ] Task that must be done multiple times `completion:2of5`
	- [ ] ID numbers `id:abc123` (hash the text?)
	- [ ] Task blockers `requires:abc123` (requires task with `id` be complete first)
- [ ] Default (built-in) sort options (ascending & descending)
    - [ ] By due date
	- [ ] By creation date
	- [ ] Only tasks matching a specific project (or multiple)
	- [ ] Only tasks matching a specific context (or multiple)
	- [ ] Due today!! (or due on specific date)
	- [ ] Only complete (or incomplete) tasks
	- [ ] By priority
	- [ ] By whether or not it has a specific tag
- [ ] Multi-file 
    - [ ] (store `origin:filename`, save back to where they came from)
	- [ ] Sort into files by Project
	- [ ] Pruning (move completed tasks into a *done* file)
- [ ] Organisation-y things
	- [ ] List the different projects
	- [ ] List the different contexts
- [x] Learn how to properly use the built in Rust test suite  
- [ ] At some point spin Date off into its own library

#### Planned Frontends

- [ ] Make a separate Rust TUI for this
- [ ] Something something NextCloud syncing
- [ ] Google syncing?  That can't be too hard... Right...?
- [ ] Make an Obsidian plugin?
- [ ] NextCloud web-app?

