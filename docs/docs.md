# Rustentials docs 

Aim of this project is to provide lightweight, performant and secure password manager. 

Currently the front-end has been done using Cli but the codebase has such structure that it wouldn't be a huge
chore to tie it to some gui.

## Architecture 

Front-end of application is Cli based, implemented using `Clap` crate.

Back-end is using `Sqlite` as database and `Argon2` for master password hashing.

TODO -> Talk about how actual passwords are encrypted.

## Codebase 

Codebase is split in couple of main directories:

- core -> Reusable code containing core app algorithms 
- repository -> Database related code
    - queries -> Contains Sql query code 
- ui -> Cli related code 
  - commands -> Implementation of available commands
- modes -> Contains main structs used in app

## How to run 

Developer: 
I strongly suggest you use `watch` crate, it will watch for changes in code and then rebuild the project.

There are two ways of running the app: 
1) Cargo run -- arg1 arg2 ... 
2) Navigate to target **Rustentials/target/debug** there should be Rustentials app which you can run, in dev builds 
database should be in that folder aswell.

Note:
Once database is generated you can check content of it on some online tools by just taking `rustentials.db` and dropping it in some [online viewer](https://sqliteviewer.app/). 

### Tests 

Rustentials has working tests for testing database services and authentication of passwords.

`/scripts` folder contains script to run tests, change the permission on it and run it.

## Git actions 

`Pre pull/push to Main branch` has action that runs the tests and confirms that all is good before merge/push!
