#!/bin/bash

# Set your Discord token
export DISCORD_TOKEN="your_discord_token_here"

cargo build --release
clear
echo '[info:] Starting Discord bot'
./target/release/discord

