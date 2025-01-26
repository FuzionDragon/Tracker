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

### Immediate

- Currently Mark breaks running on the same directory more than once, so for validation of this and some other functions which involved adding entries a Query sql function is needed

- Sqlite Databases are automatically created in the current directory, something for proper use will be annoying. So there is a need to bring back the Toml storage of a main database which the program checks each runtime for database actions.
