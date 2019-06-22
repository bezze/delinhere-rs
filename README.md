# delinhere
A vim pluggin to autodetect bracket pairs and map common verbs.

## Motivation

If you find yourself inside a function's arguments and you would like to delete
them, the correct vim "phrase" would be ´di)´, which translates to "delete in
parentheses". If you would like to delete the parentheses too, ´da)´, which
translates to "delete around parentheses". This holds true for other structures,
like ´yi]´ will yank everything inside paired square brackets.

So the motivation behind this simple pluggin is: I hate finding the correct
bracket type. First, my brain has to identify if I'm inside a curly
bracket or a parentheses. Then I have to find the correct key in my keyboard.
Annoying. I almost always just need the closest bracket pair, no matter it's
type. So this pluggin does that. It finds the _closest bracket_ and uses that one.

## Mappings:

    * Delete in here:       ´dih´
    * Delete around here:   ´dah´
    * Change in here:       ´cih´
    * Change around here:   ´cah´
    * Yank in here:         ´yih´
    * Yank around here:     ´yah´
    * Select in here:       ´vih´
    * Select around here:   ´vah´
