# rush - a simple shell in Rust

## Introduction

One of the things developers use every day is a shell. It comes in many flavors like bash, fish or zsh. Depending on your background, writing your own shell from scratch might either sound intimidating or pointless. We like to believe that it can be a fun way to gain more hands-on Rust experience. If you want to play with concepts like I/O, error handling, and syscalls we invite you to participate in this workshop. Who knows, it could also be an opportunity to start your first mid-size Rust project?

This is a workshop about writing your own bash-like [shell](https://multicians.org/shell.html) in Rust. It will show you the basics, but it's by no means feature-complete.

A shell is a really big project. We can not cover everything.
Take it as an excuse to learn Rust rather than a guide to write the perfect shell.

## Who's the target audience?

This workshop is intended for *intermediate* Rust programmers that already understand basic Rust concepts and have some programming experience. We’ll explain the rest as we go along.
When we were learning Rust and already knew the basic concepts, we always wanted to see a mid-size system being built to get an understanding for how the concepts are put into practice. We think this could be a learning boost for people that are in the same situation like we were.

[Santiago Pastorino](https://santiagopastorino.com) is the [WyeWorks](https://wyeworks.com) co-founder, Ruby on Rails core team alumni and Rust Developer. He is currently part of the Rust compiler NLL WG and [Rust Latam](https://rustlatam.org) organizer.
[Matthias Endler](https://matthias-endler.de ) is a Backend Engineer at trivago and runs [Hello Rust Show](https://hello-rust.show).


### Necessary tools

* [rustc](https://rustup.rs/)
* [cargo](https://github.com/rust-lang/cargo)
* [git](https://git-scm.com/)

## Structure

Every commit adds one new feature or refactors the code in a significant way.
You can `git checkout <commit_id>` to see how the code looks like after each step.

You can always check your implementation by running `./validate`.

(Validate script was adapted from the majestic [build-your-own-shell](https://github.com/tokenrove/build-your-own-shell) by Julian Squires.)

## Features we cover

1. Running single commands
2. Concatenating commands with semicolon
3. Shell-builtins (cd)
4. Pipes
5. Ideas for extending your shell
   - readline
   - handling control signals (ctrl+c, ctrl+d)
   - redirection
   - command completion
   - adding more builtins
   - use a grammar for parsing
   - Hints for the workshop

### Section 0 - Check Rust installation and version

- rustc -v
- try running Rust Edition 2018 preview

### Block 1 - Running single commands

- Try to run a single command and print the output to the stdout.
- Hint: Look for APIs in the standard library to do that.
- Bonus: Try to make the code as idiomatic as you can.
- Bonus: Write a unit test to make sure your shell works

### Block 2 - Concatenating commands

- Try to run two commands in sequence and print all output in sequence
  to stdout
- Bonus: Write an [integration test](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)

### Block 3 - Builtins

- What would be a nice way to handle builtins?
- Implement a builtin of your choice (e.g. exit, echo, cd, history,
  for,...)
- Bonus: Think about why \`cd\` must be a builtin

### Block 4 - Pipes

- Implement pipes, which are a way to feed the output of one command into another one.
	Syntax:

	```shell
	command1 | command2
	```

- Think about ways to make command representation more idiomatic.

### Block 5 - Bring your own features (e.g. "where the fun is at")!

* Add readline support
* Handle control signals (ctrl+c, ctrl+d)
* Redirection
* Command completion
* Implement more shell-builtins
* Use a grammar for parsing
* Add more tests
