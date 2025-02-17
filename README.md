# Tracker

A solution to track ongoing tasks or projects in the command line.

Uses a Sqlite database to store your trackers and your default editor to add and remove tasks.

---

### Setup

Simply run tracker in the terminal to get started.

You will need to have EDITOR defined with your favourite text editor in your environment as a prequisit.

The database will be created and stored in ~/.local/share/tracker/tracker.db.

### Usage

Due to limitations of Rust programs only being able to affect its own environment and not the users shell, scripts or certain other commands are needed for.

This also helps with flexibility of using the tool in scripts.

This example uses awk and fzf, allowing interactive picking of marked directories to cd into.

```bash
!/path/to/shell

cd "$(tracker marked | awk -F'|' '{ print $2 }' | fzf)"
```
