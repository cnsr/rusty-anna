# Rusty Anna

## About

 This is a WIP chatbot for the [Livechan](https://github.com/emgram769/livechan-js/)-based chats, specifically made for the [kotchan](https://kotchan.fun/chat/int) - the only that's still somewhat alive. The idea isn't new by any means - there are multiple ( [1](https://github.com/emgram769/anna), [2](https://github.com/cnsr/SadBot), [3](https://github.com/slavking/anna3) and others ) implementations - one of them is by yours truly. The initial implementation is called Annal, thus the repository name.

 Annacoding has reached the point at which everything works like shit in any of the versions and since the livechan maintainer is as good as fucking dead, any implementation has to overcome unintentional obstacles like the one post per 3 seconds limit which is an utterly retarded decision. There used to be an option to do that using a cookie but that doesnt seem to be working lol.

This implementation uses a `MessageQueue` system which is kinda broken as of right now as `POST`ing a message to the API doesn't return the expected response but instead returns an error.

## TODO:

 - [x] Working message retrieval
 - [x] Somewhat working message posting
 - [] Properly working message posting from the queue
 - [] Loading bot commands from `commands.yml` file
 - [] Docker