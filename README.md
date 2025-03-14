# kilit-cli

🇹🇷 [Türkçe](README.tr.md) 

This is a cli based password manager written in Rust.


## USAGE
```bash
$ kilit-cli [OPTIONS]
```
## OPTIONS
```txt
-c, --conf <conf>        Set the store file path
                         	(Example usage: "./kilit -c "$HOME/.mypassfile"") and don't use ~ for
                         home dir it crashes the tool [default: $HOME/.kilit]
-h, --help               Print help information
-p, --prompt <prompt>    Instead of opening a new shell the commands will be written using this
                         argument
                         	(Example usage kilit -c "~/.adana" -p "go passwd list name bgc"
                                           kilit -c "~/.adana" -p "create passwd") 
-V, --version            Print version information
```
