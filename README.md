# fxbot

This simple discord bot replaces links to posts containing https://twitter.com or https://x.com with links to https://fxtwitter.com, which fixes embeds!

## How to use
There are two ways to use `fxbot`:

1. You can use the official `fxbot` by [adding it to your instance](https://getfx.bot) 

I, personally, host the bot with the same code found in this repo. If there are any problems with the bot, please reach out to me by [pinging me on twitter](https://twitter.com/d0nutptr)

2. If you want to host it yourself, [create a discord bot token](https://www.writebots.com/discord-bot-token/) and launch the app as follows:
```
cargo build --release

DISCORD_TOKEN=<discord bot token> ./target/release/fxbot
```

Then add your bot to your own discord instance.
