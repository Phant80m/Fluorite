#!/bin/bash

# Set your Discord token
export DISCORD_TOKEN="Enter your bots token here"

cargo build --release
clear
echo '[info:] Starting Discord bot'
./target/release/discord

