# Tracker

A solution to track ongoing tasks or projects in the command line.

Uses a Sqlite database to store your trackers and your default editor to add and remove tasks.

---

### Usage

Simply run tracker in the terminal to get started.

You will need to have EDITOR defined with your favourite text editor in your environment as a prequisit.

The database will be created and stored in ~/.local/share/tracker/tracker.db.

---

### Todos

- Ability to use multiple trackers with expected features (add, delete, list)

- Shell scripts to allow integration with cronjobs (for my personal use case)

- Output tracker in different data formats (CSV, JSON, TOML)

- Configuration to allow change for record schema

- Independant configuration for each created tracker
