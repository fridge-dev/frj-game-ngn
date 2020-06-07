Read more about better input prompting:

* https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
* https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_1.rs
* http://dcuddeback.github.io/termios-rs/termios/

```
//////////////////////////////////////////////////
 _______________________  _______________________
/                       \/                       \
|                       ||                       |
|      +---------+      ||      +---------+      |
|      | X X X X |      ||      | o o o o |      |
|      +=========+      ||      | o o o o |      |
|      |         |      ||      | o o o o |      |
|      |         |      ||      | o o o o |      |
|      |         |      ||      |         |      |
|      | o   o o |      ||      |         |      |
|      | o o o o |++++  ||      |         |      |
|    --| o o o o |+     ||      |         |      |
|     -| o o o o |++    ||      |         |      |
|     -| o o o o |+     ||      +=========+      |
|    --| o o o o |      ||      | 0 0 0 0 |      |
|      +---------+      ||      +---------+      |
|          ^            ||                       |
\_______________________/\_______________________/
//////////////////////////////////////////////////

- on left for each correct peg in incorrect slot
+ on right for each correct peg in correct slot

1) Press Left/Right arrow to move cursor (above)
2) Press 0-8 to select a color (below)
3) Press enter to place peg

.-------. .-------. *---+---*
| Clear | |  Red  | | Orang |
.-------. .-------. *---+---*
   [0]       [1]       [2]

.-------. .-------. .-------.
| Yelow | | Green | | Blue  |
.-------. .-------. .-------.
   [3]       [4]       [5]

.-------. .-------. .-------.
| Purpl | | Pink  | | Brown |
.-------. .-------. .-------.
   [6]       [7]       [8]
```
