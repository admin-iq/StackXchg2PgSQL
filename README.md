# StackXchg2PgSQL

A tool for importing Stack Exchange data dumps into PostgreSQL using Rust and Diesel ORM.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Database Setup](#database-setup)
- [Working with Diesel Migrations](#working-with-diesel-migrations)
- [Schema Management](#schema-management)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Introduction

StackXchg2PgSQL is a utility that helps you import Stack Exchange data dumps into a PostgreSQL database. It uses the Diesel ORM for Rust to handle database operations.

## Installation

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Diesel CLI

### Install Rust

If you don't have Rust installed, you can install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install PostgreSQL

#### macOS

```bash
brew install postgresql
brew services start postgresql
```

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### Install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Clone and Build

```bash
git clone https://github.com/your-username/stackxchg2pgsql.git
cd stackxchg2pgsql
cargo build --release
```

## Usage

```bash
A command-line tool to load StackExchange XML dumps into a PostgreSQL database

Usage: stackxchg2pgsql --badges-path <BADGES_PATH> --comments-path <COMMENTS_PATH> --posts-path <POSTS_PATH> --post-history-path <POST_HISTORY_PATH> --post-links-path <POST_LINKS_PATH> --tags-path <TAGS_PATH> --users-path <USERS_PATH> --votes-path <VOTES_PATH> --database-url <DATABASE_URL>

Options:
      --badges-path <BADGES_PATH>              The path to the badges XML file
      --comments-path <COMMENTS_PATH>          The path to the comments XML file
      --posts-path <POSTS_PATH>                The path to the posts XML file
      --post-history-path <POST_HISTORY_PATH>  The path to the post history XML file
      --post-links-path <POST_LINKS_PATH>      The path to the post links XML file
      --tags-path <TAGS_PATH>                  The path to the tags XML file
      --users-path <USERS_PATH>                The path to the users XML file
      --votes-path <VOTES_PATH>                The path to the votes XML file
      --database-url <DATABASE_URL>            The database URL
  -h, --help                                   Print help
  -V, --version                                Print version
```

Example:

```bash
stackxchg2pgsql \
--badges-path /tmp/dataset/Badges.xml \
--comments-path /tmp/dataset/Comments.xml \
--posts-path /tmp/dataset/Posts.xml \
--post-history-path /tmp/dataset/PostHistory.xml \
--post-links-path /tmp/dataset/PostLinks.xml  \
--tags-path /tmp/dataset/Tags.xml \
--users-path /tmp/dataset/Users.xml \
--votes-path /tmp/dataset/Votes.xml \
--database-url postgres://postgres:postgres@172.17.0.3/stackexchange
```
```
Data loaded successfully into the database.
```

## Database Setup

### Configure Database Connection

Export the database URL as an environment variable.

```bash
export DATABASE_URL=postgres://username:password@localhost/stackoverflow
```

## Working with Diesel Migrations

Diesel migrations help manage your database schema. The project already includes initial migrations in the `migrations` directory.

### Run Migrations

To apply all pending migrations:

```bash
diesel migration run
```

### Redo the Latest Migration

If you need to revert and reapply the latest migration:

```bash
diesel migration redo
```

### Revert Migrations

To revert the latest migration:

```bash
diesel migration revert
```

To revert all migrations:

```bash
diesel migration revert --all
```

### List Migration Status

To see which migrations have been applied:

```bash
diesel migration list
```

## Troubleshooting

### Connection Issues

If you encounter database connection errors:

1. Verify your database credentials
2. Ensure PostgreSQL is running:
   ```bash
   pg_isready
   ```
3. Check PostgreSQL logs:
   ```bash
   tail -f /var/log/postgresql/postgresql-*.log
   ```

### Migration Errors

If migrations fail:

1. Check the error message carefully
2. Try running migrations with verbose output:
   ```bash
   diesel migration run --verbose
   ```
3. Inspect the `schema_migrations` table:
   ```sql
   SELECT * FROM __diesel_schema_migrations ORDER BY run_on DESC;
   ```

### Reset Database

To completely reset your database and run migrations from scratch:

```bash
diesel migration redo
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
