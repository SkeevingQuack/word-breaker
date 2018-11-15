## wordlist

Bulk handling of the wordlist. Functions may be moved into Rust eventually.

#### final.txt

List of words to be used by the game.

#### Moby/

Moby Word II word lists.

#### blacklist.txt

List of unwanted but otherwise viable words.

#### whitelist.txt

List of wanted and viable words not in the list.

#### clean.py

The script that turns `raw.txt`, `blacklist.txt`, and `whitelist.txt` into `final.txt`.
