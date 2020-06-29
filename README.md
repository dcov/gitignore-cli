# gitignore CLI
A CLI tool to manage .gitignore files in your project.

## Setup
Set $GITIGNORE_HOME. It should contain <file_stem>.gitignore files that will be used to generate the .gitignore file.

###### Example
```bash
// Clone the github/gitignore repo
git clone https://github.com/github/gitignore

// Set $GITIGNORE_HOME to the path of the repo.
export GITIGNORE_HOME=path/to/gitignore_repo
```


## Usage
```bash
gitignore [flags] <file_stems>
```
### <file_stems>
*If generating the .gitignore file:* The <file_stem>.gitignore files in $GITIGNORE_HOME to use when generating the
.gitignore file.

*If removing from the .gitignore file:* The <file_stem> named blocks to remove from the generated .gitignore file.

###### Example
```bash
// Generate a .gitignore file with the contents of $GITIGNORE_HOME/Rust.gitignore
gitignore rust

// Remove the $GITIGNORE_HOME/Rust.gitignore block from the .gitignore file
gitignore -r rust
```

### [flags]

#### *-h, --help*
Print help information

#### *-V, --version*
Print version number

#### *-c*
Generate the .gitignore file in the current directory.

The default behavior is to search for the directory where the '.git' directory lives, and generate, or modify, the
.gitignore file there.

#### *-r*
The <file_stems> arguments will be used to remove any existing blocks with those names.

By default <file_stems> arguments are used to generate/modify blocks with those names.

#### *-l*
List the current block names in the generated .gitignore file.

#### *-s*
Sync the exisiting .gitignore blocks with the source $GITIGNORE_HOME files.

This is useful if you've updated the source $GITIGNORE_HOME files, and want to update the generated .gitignore file.
