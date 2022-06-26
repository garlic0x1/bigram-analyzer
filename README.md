# bigram-analyzer  
Determine if a string is cleartext or not by bigram analysis.  
First you must have a corpus to generate a matrix of bigraph occurrence scores from, then you can test strings against the matrix to guess if it is cleartext or not.  

# help  
```
$ bigram-analyzer help
bigram-analyzer 0.1.0

USAGE:
    bigram-analyzer <CORPUS> <SUBCOMMAND>

ARGS:
    <CORPUS>    local file or URL to generate matrix with

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    clear     print cleartext words from stdin
    hash      print hashed/encoded words from stdin
    help      Print this message or the help of the given subcommand(s)
    matrix    print occurrence matrix
```

options for clear and hash subcommands:  
```
OPTIONS:
    -h, --help
            Print help information

    -o, --occurrences-max <OCCURRENCES_MAX>
            n rare bigraphs to be encoded [default: 1]

    -s, --score-min <SCORE_MIN>
            minimum occurence score for "common bigraph" [default: 10]

    -u, --unique
            only print unique results

    -V, --version
            Print version information
```
