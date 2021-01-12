# NRP

Stupid "Name Rust Program" application.

## What does it do?

NRP is my stupid idea for naming projects in Rust.

The logic for naming is: You put a small description of your application -- for
example "Name Rust Programs". The application will try to find out a name by
selecting an adjective that starts with the same letter of every word, except
the last one; the last one will pick a metal/alloy for it.

For example, if you use "Name Rust Program" as a parameter, you can get
"normative random promethium".

## Usage

### `generate`

Use `generate` to generate a name. Example:

`nrp generate 'name rust program'`

### Managing adjectives and metals

To manage adjectives and metals, start with the topic you want to manage:
`adjectives` for managing adjectives and `metals` to manage metals.

Both accept an action over their collections: `list` will list the contents of
said group, `add <name>` will add a new name to the collection and `rm <name>`
will remove that name from the list.

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.
