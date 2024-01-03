# Write Yourself A Shell in Rust

![Course banner](assets/banner.png)

## Introduction

One of the things developers use every day is a [shell](https://multicians.org/shell.html). It comes in many flavors like `bash`, `fish` or `zsh`. Depending on your background, writing your own shell from scratch might either sound intimidating or pointless. We like to believe that it can be a fun way to gain more hands-on Rust experience. If you want to play with concepts like I/O, error handling, and syscalls we invite you to participate in this workshop. Who knows, it could also be an opportunity to start your first mid-size Rust project?

> [!NOTE]
> Shells are very complex and we will only implement a basic subset of their functionality.
> Take this workshop as an excuse to learn Rust rather than a guide for how to write the perfect shell.

## Who's the Target Audience?

This workshop is intended for *intermediate* Rust programmers who already understand basic Rust concepts and have some programming experience. We’ll explain the rest as we go along.
When we were learning Rust, we always wanted to see a mid-size system being built to get an understanding of how the concepts are put into practice. We think this could be a learning boost for people who are in the same situation as we were.

### Necessary Tools

* [rust](https://www.rust-lang.org/tools/install)
* [git](https://git-scm.com/)

## Structure

Use `src/main.rs` to start writing your code.
If you get stuck, check out the [examples](/examples) folder, which contains working source code for each block.
We recommend to try it yourself first and only refer to the example code in case you run into issues.

You can always check your implementation by running `./validate`.
(Validate script was adapted from the majestic [build-your-own-shell](https://github.com/tokenrove/build-your-own-shell) by Julian Squires.)

## Features We Cover

1. Running single commands
2. Concatenating commands with semicolon
3. Shell-builtins (`cd`)
4. Pipes
5. Ideas for extending your shell
   - readline
   - handling control signals (<kbd>Ctrl</kbd> + <kbd>c</kbd>, <kbd>Ctrl</kbd> + <kbd>d</kbd>)
   - redirection
   - command completion
   - adding more builtins
   - use a grammar for parsing
   - Hints for the workshop

## Blocks

Here are the individual blocks of work that we will cover:

* Block 0 - Check Rust Installation and Version
* Block 1 - Running Single Commands
* Block 2 - Concatenating Commands
* Block 3 - Shell Builtins
* Block 4 - Shell History Support
* Block 5 - Pipes
* Block 6 - Bring your own features!

## Show And Tell!

We are curious to see what you have built. If you want to share your shell with
us, please send us a link to your repository. We will add it to the list below.

We'd be happy to hear your answers to the following questions:

- What did you learn?
- What was easy?
- What was hard?
- Would you do anything differently?
- What would you like to learn next?

## Closing Words

If you enjoyed this workshop, please share it with your friends and colleagues.
It would greatly help us if you could tweet/toot about it or share it on
[Reddit](https://www.reddit.com/r/rust/) or [LinkedIn](https://www.linkedin.com/).
Thanks!

You might also want to [subscribe to our newsletter](https://corrode.dev/blog/) for
future workshops and other Rust content.

If you are looking for professional Rust training, please get in touch with us
at [corrode.dev](https://corrode.dev/).
