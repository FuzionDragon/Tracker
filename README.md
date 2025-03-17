# Tracker

A solution to track ongoing tasks or projects in the command line.

Uses a Sqlite database to store your projects and your default editor to add and remove tasks.

Simple by design to allow integration with scripts.

---

### Setup

Simply run tracker in the terminal to get started.

You will need to have EDITOR defined with your favourite text editor in your environment as a prequisit.

The database will be created and stored in ~/.local/share/tracker/tracker.db.

### Usage

Due to limitations of Rust programs only being able to affect its own environment and not the users shell, scripts or certain other commands are needed for more environmental functionality.

With that in mind the tool has been made with simple outputs which helps with flexibility of using the tool in scripts.

Some scripts found in the project root which I use.

You can have an interactive picking of project directories to cd into.

```bash
!/path/to/shell

cd "$(tracker dirs | awk -F'|' '{ print $2 }' | fzf)"
```

You could also just jump to the hooked project directory.

```bash
!/path/to/shell

cd "$(tracker hooked | awk -F'|' '{ print $2 }')"
```

### Commands

Edit tasks in default text editor $EDITOR

Edit,

Lists all projects 

List,

Clears all projects 

Clear,

Adds current project directory to the tracker
If there is no hooked directory, then the marked directory becomes the next jump location

Mark {
name: Option<String>,
},

By default, sets default jump directory regardless of any newly marked directories

Hook {
name: Option<String>,
},

Prints hooked name and directory, if there is no hooked project then it will print the last
marked project.
If there are neither then it will print nothing.

Hooked,

Unhooks the currently hooked project, otherwise does nothing

Unhook,

Prints both hooked and marked projects

Special,

Prints all projects with a non-empty directory field

Dirs,
