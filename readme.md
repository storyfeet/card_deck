# Card Deck

The intent of this module is to provide methods for managing a deck and discard pile from a deck of generic cards.

The expectation is that cards will moved into and out of the deck, copy and clone are not used internally.


## Changes

### in v0.1.9

Fixed bug in card creation (All Spades)
Enabled from_str and Display for PCard in playing cards
Added tests for card creation

### in v0.1.8 

PartialEq on deck
push_discards now takes an iterator


### in v0.1.7
A playing cards module that works well with this deck.

### in v0.1.5

Added "dig\_for" and "dig\_all" methods for grabbing the first/all cards that match a filter respectively.
