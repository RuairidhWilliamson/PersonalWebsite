# Recgit

February 2024

Rust, CLI

---

## Background

I use git for almost all my projects. Even if it is just me working on a project, I will use git to synchronize it on GitHub/GitLab. I do this because I am not very good at taking backups and I want some protection in case of hard drive failure. This has lead me to a problem where I have many repositories checked out locally but in different states of synchronization with their remotes. I would like a tool to scan all of these repositories for changes that are only present on my local system and have not been pushed to remote. This tool should also tell me exactly what I only have a local copy of so that I can rectify it.

## Recgit

Recgit is a CLI tool written in Rust I made to solve this problem. Named recgit because of "recursive git". It has a very simple command line interface with only one command, `recgit status`.

The status command walks the directory tree looking for git repositories. When it finds one it and checks if the repository has any unsynced changes. It then prints these results to standard output for the user, highlighting where code is not synchronized.

[![asciicast](https://asciinema.org/a/Syj6rYTvEgVbOeK63T8RxXj3b.svg)](https://asciinema.org/a/Syj6rYTvEgVbOeK63T8RxXj3b)

## Initial Implementation

Git is quite complicated so my first pass at this was to run the git CLI and parse its output. The most obvious command is `git status`. This lists files that are changed in the working directory compared to HEAD and if the checked out branch is in sync with the tracking branch. Git status by default is not very easy to parse but using `git status --porcelain=v2` gives a more verbose but well defined output. (`v2` here represents the format version so we can rely on this not breaking in future git versions). See [https://git-scm.com/docs/git-status#\_porcelain_format_version_2](https://git-scm.com/docs/git-status#_porcelain_format_version_2) for the documentation of this format.

We parse this format but it doesn't give us all the information we want. Adding `--branch` gives information about the checked out branch and its upstream tracking branch. Adding `--show-stash` gives information about the stash. Tracking if their are changes in the stash is important as these are easy to forget about.

All together this gives us `git status --branch --show-stash --porcelain=v2`. Parsing this into a struct is fairly simple. Code is available here [`src/simple_status.rs`](https://github.com/RuairidhWilliamson/recgit/blob/main/src/simple_status.rs).

Writing tests for this was important. The tests creates a git repository in `/tmp` and slowly adds commits, files and branches. It runs recgit at each stage asserting the status is as we expect and the method `has_unsynced_changes` is correct. To mock a remote repository for the test it creates a second repository in `/tmp` and sets the remote of the first repository to the second using the `file://` protocol.

This test makes an important discovery about this method though. Only the checked out branch is inspected. Any changes not synced on other branches are invisible to `git status`. This is maybe ok for some use cases but it is common to have multiple feature branches locally that are not synced. It is important recgit can handle this case.

## Gitoxide Implementation

To address the different branches issue I switched from using the git CLI to using a git library. This will give easier programmatic access to git without parsing complex structures. In rust there is [git2](https://github.com/rust-lang/git2-rs) which is a wrapper around the C [libgit2](https://libgit2.org/) library. There is also [gitoxide](https://github.com/GitoxideLabs/gitoxide) a pure rust implementation of git. Gix is incomplete as of writing but provides enough for my use case. I don't think it makes much of a difference choosing gitoxide vs git2 so I chose gitoxide.

Firstly I still want to make use of `git status --branch --show-stash --porcelain=v2` as it works and rewriting it using gitoxide wouldn't actually bring much benefit. Using gitoxide I find all the remote branches and their head commits. Then going through each local branch I check if there is a corresponding remote branch with the same commit. Any local branches without a remote are collected into a list and indicate that the branch is unsynced.

Using the same testing method as before we get the expected results and fix the case when a branch not checked out is unsynced. There are still some false positives and false negatives that I plan on fixing in the future.

Try it out here

[Recgit](https://github.com/RuairidhWilliamson/recgit)
