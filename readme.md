# Truth or Dare Bot

A simple Truth or Dare bot with a database backend.

## Project Structure

- `bot/` - Bot source code
- `database/` - Database models, schema, and migrations

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://diesel.rs/guides/getting-started/) 
- SQLite (used as the database backend)


## Setup

1. **Clone the repository**

   ```sh
   git clone https://github.com/Yvonne-Aizawa/truth-or-dare-bot.git
   cd truth-or-dare-bot
   ```

2. **Set up environment variables**

   Copy the example environment file and edit as needed:

   ```sh
   cp .env.example .env
   ```

3. **Build and run the bot**

   ```sh
   cd bot
   cargo run
   # or cargo run --release (will take longer)
   ```
## on updates

in the repo
```sh
git pull
cargo build
```

## running as a service 

move the binary (in /target/release/tod_bot) and the .env to somewhere else make sure they are in the same folder

```toml
[Unit]
Description=This is a truth or dare discord bot
After=network.target
Wants=network-online.target

[Service]
Restart=always
Type=simple
ExecStart=/your/path/here
Environment=

[Install]
WantedBy=multi-user.target

```

## Help

To see all available commands and their descriptions, use the `/help` command in your Discord server after inviting the bot.  
The bot will display a list of commands such as:

- `/add_truth` — Add a new truth question.
- `/add_dare` — Add a new dare challenge.
- `/get_truth` — Get a random truth.
- `/get_dare` — Get a random dare.
- `/list_truths` — List truths (admin only).
- `/list_dares` — List dares (admin only).
- `/accept` — Accept a truth or dare (admin only).
- `/delete` — Delete a truth or dare (admin only).

If you need further assistance, please open an issue on the repository or contact the maintainer.

## Notes

- Database file: `database.db`
- Edit `.env` to configure database connection and bot settings.

---

For more details, see the source files in [bot/](bot/) and [database/](database/).