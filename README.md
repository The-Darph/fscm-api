# Fascism Tracker API

> A collaborative account of every significant fascist event enacted by the current U.S. administration until it either ends or it ends us...

## Purpose

The purpose of this API is to provide a backend to pull data related to these fascists acts. It serves as an archive
and a way to search, sort, and filter these things so people can get an idea of the scale and speed of what is unfolding.

## Build & Run

__Make sure you configure everything first otherwise the app may build and run
but then panic later. See [Configuration](#configuration) section below__

1. Have Rust installed.
2. Run `cargo build -p cli` from the root of the project.
3. Update your `.env` file
4. Run `./target/debug/cli server`

Okay so maybe running from the debug folder isn't the best idea but it's the best we got
right now. Also, you may want to add some sort of watcher to the process if you run this 
on a server so that it starts on reboot and comes back after a crash. There's still
a ways to go before this project is stable in my eyes.

## Contributing

For simplicity and security, all contributions will happen via database
seeds. You write a database seed using standard SQL statements.

![Database diagram](./docs/fscm-api-db.png)

### Branches

The `master` branch is what ends up in production. It is all that will exist at first 
until the project is mature enough for a very basic deploy.

__`develop`__ is the branch where all pull requests will be merged into and tested

__`feature_`__ branches are branches you create and will submit when you want to create 
a pull request. 

To contribute to the code itself or even add an event through a seed, follow these steps:

1. Clone the project
2. Switch to the `develop` branch (`git checkout develop` or `git checkout -b develop && git pull origin develop`)
3. Create your own branch off of that and give it an appropriate name like `event_20250505` for events and `feat_<whatever_here>` for features
4. Push your branch to your repository and then open a pull request. Simple.

### What counts as a "fascist event"

We're tracking two main types of events: Fascism and events leading to World War III or similar wars.

## Configuration

1. Make sure to fill out the .env file. For your API key you'll want to choose a strong password then run it through SHA-256 hashing that way you don't have to send the hash in clear text every time you use the HTML SAP to create new events
2. In /cli/api/mod.rs you will need to update the allows URLs that are accepted for CORS requests. You should only use two. One for production and the other for development but you do what you want. It's your security.
3. In the `docs` folder you'll find CSV files full of events you can fill your DB with. These items are already part of the database migrations. To add more, simply create *new* CSV files (I export them using Numbers on Mac) and then have ChatGPT or something help you turn those into SQL queries since there will be a lot of data to process by hand. __DO NOT__ add more to CSV files that have already been processed. The existing CSVs are for archive purposes only and if you ever need to nuke your database and start over.

## Deploy

This app has never been deployed but here are the notes for deploy written as 
I develop it so when it comes time to write a deploy script I remember:

1. Make sure Rust is installed on the server along with Cargo's `binstall` (no, that is not a type)
2. Install dependencies: `cargo build`
3. Run migrations with `diesel migration run` (testing the up/down migration at once to rebuild a database requires running `diesel migration redo`)
