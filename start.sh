#!/bin/bash

# Set your Discord token
export DISCORD_TOKEN="MTExNjg4NjE4NzQ5MjI0NTU4NQ.GXIkDW.yiqQfYGQ_fBr2WG6hAFQiLLqhTY68Q854pY03E"

cargo build --release
clear
echo '[info:] Starting Discord bot'
./target/release/discord

