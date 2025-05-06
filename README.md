# Fascism Tracker API

> A collaborative account of every significant fascist event enacted by the current U.S. administration until it either ends or it ends us...

## Purpose

The purpose of this API is to provide a backend to pull data related to these fascists acts. It serves as an archive
and a way to search, sort, and filter these things so people can get an idea of the scale and speed of what is unfolding.

## Contributing

For simplicity and security, all contributions will happen via database
seeds. You write a database seed using standard SQL statements.

### What counts as a "fascist event"

We're tracking two main types of events: Fascism and events leading to World War III or similar wars.

## Deploy

This app has never been deployed but here are the notes for deploy written as 
I develop it so when it comes time to write a deploy script I remember:

1. Make sure Rust is installed on the server along with Cargo's `binstall` (no, that is not a type)
2. Install dependencies: `cargo build`
3. Run migrations with `diesel migration run` (testing the up/down migration at once to rebuild a database requires running `diesel migration redo`)
