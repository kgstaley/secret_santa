# Secret Santa CLI
CLI tool intended to automate pulling names out of a hat and matching Secret Santa participants.

Works by reading contents from a local `.txt` file (newline delimited) and iterating over the collection of participants to determine Secret Santa matches.

# How to run
```
cargo run -- <file_path>
```

# Examples

Your `.txt` file may look something like this:
```
# secret_santa.txt
Bob
Janet
Sally
Joe
Will
Susie
```
And you would run the tool like so: 
```
cargo run -- /User/username/PATH/TO/FILE.txt
```

# todo
- write some tests