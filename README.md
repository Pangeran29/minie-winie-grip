# MINIE WINIE GRIP (it's just a grep but with RUST 😎)

In Rust Docs (https://doc.rust-lang.org/book/ch12-00-an-io-project.html) there is a project building a command line program that performs things like what grep does, searching for some string with some pattern that is provided. To make this project better, I decide to make a more complex system. this program will search into some string pattern but rather than searching it in a file this minie winie grip will search into a whole directory that provides. Also, this program will return the reference to the related string, think of it like a string searcher in VsCode (ctrl + shift + f). cool right 😎

![alt text](/asset/image.png)
![alt text](/asset/image-1.png)

# TO DO
1. exclude search in some diretory (binary/else)

# AVAILABLE COMMAND
IGNORE_CASE=1 cargo run -- <search> <dir>
*** IGNORE_CASE=* for specifying to search with case sensitive or insensitive
