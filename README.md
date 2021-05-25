# Rusty Anna

## About

 This is a WIP chatbot for the [Livechan](https://github.com/emgram769/livechan-js/)-based chats, specifically made for the [kotchan](https://kotchan.fun/chat/int) - the only that's still somewhat alive. The idea isn't new by any means - there are multiple ( [1](https://github.com/emgram769/anna), [2](https://github.com/cnsr/SadBot), [3](https://github.com/slavking/anna3) and others ) implementations - one of them is by yours truly. The initial implementation is called Anna, thus the repository name.

 Annacoding has reached the point at which everything works like shit in any of the versions and since the livechan maintainer is as good as fucking dead, any implementation has to overcome unintentional obstacles like the one post per 3 seconds limit which is an utterly retarded decision. There used to be an option to do that using a cookie but that doesnt seem to be working lol.

This implementation uses a `MessageQueue` system which is kinda broken as of right now as `POST`ing a message to the API doesn't return the expected response but instead returns an error.


## Setting up commands

Two planned ways of setting up commands are as specified in the `commands.example.yml` file.

Basic markup:

```yaml
commands:
    command_name:
        description: "command description"
        # it's planned that the regex will be evaluated
        # dynamically against each message
        regex: "^\.regex$"
        # option 1.
        # list of possible replies
        replies:
            - "hello"
            - "hi"
            - "test successful"
        # option 2.
        # you will have to manually add the matching execution
        # to the command checker (which is a TODO at the moment)
        execute: "execute_in_this_case"
```

Pseudocode command checker (very pseudo):

```rust
fn get_command_reply(command: Command, message_text: String) -> String {
    match command.execute {
        Some(executor) => {
            match executor {
                String::from("execute_in_this_case") => {
                    // parse against regex
                    return "Example return value";
                }
                _ => {
                    return "Unknown command";
                }
            }
        },
        _ => {
            // it's presumed we'll be checking the command.replies here
            return "No executor specified";
        }
    }
}
```


## TODO:

 - [x] Working message retrieval
 - [x] Somewhat working message posting
 - [ ] Inbound message processing
 - [ ] Fix tripcodes not working
 - [ ] Properly working message posting from the queue
 - [ ] Loading some of the bot commands from a `commands.yml` file
 - [ ] Docker
