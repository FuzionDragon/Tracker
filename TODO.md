# Todo

### Long Term

- Ability to use multiple trackers with expected features (add, delete, list, update)
  -[ ] Add
  -[ ] Delete
  -[*] List
  -[ ] Update

- Shell scripts to allow integration with cronjobs (need to figure out what type of functionality)

- Output tracker in different data formats (CSV, JSON, TOML)

- Independant configuration for each created tracker

- Mark and Jump feature to navigate between active projects (need to figure out finer details)

- Add more interactivity

- Maybe another functionality to add is the ability to quickly store a directory temporarily (specifically deeply nested directories) which can then be used in other ways, like moving multiple different files to said directory.

- Subset of commands will be under 'track' which will hold commands for more immediate uses, whereas commands under 'tracker' will be for lesser used commands.

### Immediate

- Sqlite Databases are automatically created in the current directory, something for proper use will be annoying. So there is a need to bring back the Toml storage of a main database which the program checks each runtime for database actions.
