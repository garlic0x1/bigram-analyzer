# bigram-analyzer  
Determine if a string is cleartext or not by bigram analysis.  
First you must have a corpus to generate a matrix of bigram occurrence scores from, then you can test strings against the matrix to guess if it is cleartext or not.  

# help  
```
$ bigram-analyzer help

bigram-analyzer 0.1.0

USAGE:
    bigram-analyzer [OPTIONS] <CORPUS> <SUBCOMMAND>

ARGS:
    <CORPUS>    local file or URL to generate matrix with

OPTIONS:
    -h, --help       Print help information
    -m, --matrix     load from matrix file (much faster than corpus)
    -V, --version    Print version information

SUBCOMMANDS:
    clear          print cleartext words from stdin
    hash           print hashed/encoded words from stdin
    help           Print this message or the help of the given subcommand(s)
    matrix         generate matrix file to use later without having to reread corpus (it goes to
                       stdout, pipe it to a file)
    probability    print the probability of a word's existence

```
