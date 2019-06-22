# delinhere
A Neovim plugin to autodetect bracket pairs and map common verbs. Made in Rust
because I was bored. I still have the original delinhere at bezze/delinhere.
It's a bit buggy, but should work for vim.

## Motivation

If you find yourself inside a function's arguments and you would like to delete
them, the correct vim "phrase" would be ´di)´, which translates to "delete in
parentheses". If you would like to delete the parentheses too, ´da)´, which
translates to "delete around parentheses". This holds true for other structures,
like ´yi]´ will yank everything inside paired square brackets.

So the motivation behind this simple plugin is: I hate finding the correct
bracket type. First, my brain has to identify if I'm inside a curly
bracket or a parentheses. Then I have to find the correct key in my keyboard.
Annoying. I almost always just need the closest bracket pair, no matter it's
type. So this plugin does that. It finds the _closest bracket_ and uses that one.

## Mappings:

    * Delete in here:       ´dih´
    * Delete around here:   ´dah´
    * Change in here:       ´cih´
    * Change around here:   ´cah´
    * Yank in here:         ´yih´
    * Yank around here:     ´yah´
    * Select in here:       ´vih´
    * Select around here:   ´vah´

## Installation

The project is very young and installation a bit tedious. The easiest way would
be to use your pluggin manager of choice, then go to the downloaded folder and
run ´´´$ cargo build´´´. You could compile for release, but with minor editions.
The important thing is that the ´´´s:bin´´´ variable in
plugin/neovim-delinhere.vim point to the right binary.

## Todos and future

* Create a decent build script for common plugin managers.
* Tidy up the logging systems (it currently sucks. Hard. I'm all ears).
* Improved parsing: support for "<", xml tags and perhaps comments.
* Argument manipulation (in progress).
