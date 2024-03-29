# discord-interactions
This portion of the Discord bot handles all of the Slash commands and other [Discord Application interactions](https://discord.com/developers/docs/interactions/application-commands) that don't require an active connection to Discord.

# How does it work
There are a few parts that do most of the heavy lifting in this setup.

## [interactions.rs](src/bin/interactions.rs)
This is where the HTTP listener is set up to receive and respond to Discord interactions. We've set up a single route, `/interact`, and the `handle_interaction` function found in `lib.rs` is what parses this interaction and sends back an HTTP response to Discord.

## [security.rs](src/security.rs)
This code provides `verify_discord_message` and `SignatureValidationError`. This is what `lib.rs` uses to validate whether the Application command came from Discord or not, as [required by Discord](https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization).

## [lib.rs](src/lib.rs)
This is the file where the command parsing starts, in particular, the `handle_interaction` function. This function has a `match` on the type of interaction coming in and then treats it accordingly. Each of these branches then calls the appropriate handler from `command_parser`

## [command_parser.rs](src/commands/command_parser.rs)
This file has a collection of parsers for each Interaction type. When you add new commands, you also add the command name to the appropriate Interaction parsing method. When that command name is provided, the `match` block will match that and will run the related function.

## [slash_cli.rs](src/bin/slash_cli.rs)
This is the tool that's used to install the Application commands to a guild or to an Application globally.

# How to set up the application
## Creating the Discord Application
TODO

## Installing the application to your server
TODO

## Installing your commands
TODO

# How to run the bot
TODO

# How to create a new command
* copy a template command or create your own
* add your `.rs` file to `src/commands/mod.rd`
* create a callback function
* create your create_command
* install your command

# Current Work
* docs for how create new commands
* add descriptions for roles in role selection

# Up Next (Sorted)
* docs for how to host this bot on various services
* repurpose `slash-cli.rs` to just `install-commands.rs` and leave `slash-cli.rs` as a CLI tool example
  * install specific commands rather than having a giant block of all the commands every time?

# Backlog (Unsorted)
* how to set up multiple branches for different purpose bots - possibly forking or maybe adding upstream and merge example?
* rename and refactor "test" directories to be examples and templates for similar commands
* rename the variables in the template commands to be more descriptive (eg: resp, cmd, etc)
