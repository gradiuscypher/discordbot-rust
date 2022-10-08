# discord-interactions
This portion of the Discord bot handles all of the Slash commands and other [Discord Application interactions](https://discord.com/developers/docs/interactions/application-commands) that don't require an active connection to Discord.

# How does it work
There are a few parts that do most of the heavy lifting in this setup.

## [interactions.rs](discordbot-rust/discord-interactions/src/bin/interactions.rs)
This is where the HTTP listener is set up to receive and respond to Discord interactions. We've set up a single route, `/interact`, and the `handle_interaction` function found in `lib.rs` is what parses this interaction and sends back an HTTP response to Discord.

## [security.rs](discordbot-rust/discord-interactions/src/security.rs)
This code provides `verify_discord_message` and `SignatureValidationError`. This is what `lib.rs` uses to validate whether the Application command came from Discord or not, as [required by Discord](https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization).

## [lib.rs](discordbot-rust/discord-interactions/src/lib.rs)
This is the file where the command parsing starts, in particular, the `handle_interaction` function. This function has a `match` on the type of interaction coming in and then treats it accordingly. Each of these branches then calls the appropriate handler from [commands::command_parser](discordbot-rust/discord-interactions/src/commands/command_parser.rs)

## [command_parser.rs](discordbot-rust/discord-interactions/src/commands/command_parser.rs)
This file has a collection of parsers for each Interaction type. When you add new commands, you also add the command name to the appropriate Interaction parsing method. When that command name is provided, the `match` block will match that and will run the related function.

# How to set up the bot
TODO

# How to create a new command
TODO

# Current Work
* docs for how create new commands

# Up Next
* docs for how to host this bot on various services

# Backlog (Unsorted)
