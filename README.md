# todo-txt-rs

Aiming to follow the [`todo.txt` standard](https://github.com/todotxt/todo.txt), but in Rust.  



****

Make `todo_txt_rs::core`?  
And then have further crates like `todo_txt_rs::ADD_ON_NAME`??  


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
- [ ] At some point spin `Date` and `shared::trigrams` off into their own library

#### Planned Frontends

- [ ] Make a separate Rust TUI for this
- [ ] And a desktop app?  
	- [ ] Using [Iced](https://github.com/iced-rs/iced)?
	- [ ] Or WASM & [Tauri](https://github.com/tauri-apps/tauri)?
- [ ] Syncing (or actually this really shouldn't be something this crate does?)
	- [ ] Something something NextCloud syncing
	- [ ] Google syncing?  That can't be too hard... Right...?
	- [ ] OneDrive?
- [ ] Make an Obsidian plugin?
- [ ] NextCloud web-app?
	- [ ] WASM?
- [ ] Firefox new tab page?!
	- [ ] WASM?
- [ ] A KanBan board using a tag? `kanban:topic,x,y`?
	- [ ] WASM?
- [ ] A todo roulette wheel to pick a task from a given set of projects or tasks

