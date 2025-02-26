# Ideas

Verb-Noun model. Like Kubectl, Docker, and Powershell.

CLI installed as "cs".

```
cs list
cs get
cs exec
cs set
```

## Location

```
cs list location // Lists all saved locations
cs add location --name <name> --longitude <longitude> --latitude <latitude> // Adds a location to the database
cs set location <location> // Sets the current location
cs get location // Gets the current location
```

## Weather

```
cs list weather // Lists the weather at all locations
cs get weather // Gets the weather at the current location
```

## Time

```
cs list time // Lists the time at all locations
cs get time // Gets the current time at the current location
```

## Teammember

```
cs list teammember // Lists all teammembers
cs add teammember --name <name> --email <email> --tz <timezone> // Adds a teammember
cs get teammember --name <name> // Gets a teammember by their name
```

## Task

```
cs list task // Lists all tasks
cs add task --name <name> --description <description> --due <due> // Adds a task
cs get task --name <name> // Gets a task by its name
```

## Link

```
cs list link // Lists all links
cs add link --name <name> --url <url> // Adds a link
cs get link --name <name> // Gets a link by its name
```

## Note

```
cs list note // Lists all notes
cs add note --name <name> --content <content> // Adds a note
cs get note --name <name> // Gets a note by its name
```

## Book

```
cs list book // Lists all books
cs add book --name <name> --author <author> --isbn <isbn> // Adds a book
cs get book --name <name> // Gets a book by its name
```

## Exec

### Create Gitignore

Looks at the current directory and creates a .gitignore file based on the files in the directory.

```
cs exec create-gitignore
```

### Create README

Looks at the current directory and creates a README.md file based on the files in the directory.

```
cs exec create-readme
```

### Slug

Converts a string to a slug.

```
cs exec slug --string "This is a string"
```

### UUID

Generates a UUID.

```
cs exec uuid
```

### Random Int

Generates a random number.

```
cs exec random-int --min 0 --max 100
```

### Create Svelte Page

Creates a Svelte page.

```
cs exec create-svelte-page --name <name>
```

### Autocommit

Adds all changes to Git, sends the diff to an LLM to generate a commit message, and commits the changes.

```
cs exec autocommit
```

### Nomenclator

Generates a name for a project.

```
cs exec nomenclator
```

