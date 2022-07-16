# wcr (wc in Rust)
wc implemented in Rust as part of reading command line rust book. This repo uses the latest version of clap which has
quite a different API compared to the API used in the book.

This program supports the following capabilities:

```shell
wcr 1.0.0
sanjayts

USAGE:
    wcr [OPTIONS] [FILE]...

ARGS:
    <FILE>...    Input files [default: -]

OPTIONS:
    -c, --bytes      print the byte counts
    -h, --help       Print help information
    -l, --lines      print the newline counts
    -m, --chars      print the character counts
    -V, --version    Print version information
    -w, --words      print the word counts
```

# Debugging

To squash corner case bugs related to formatting, I used the `od` tool. An example below:

```shell
# Reference output
>> cat tests/expected/atlamal.txt.stdin.out | od -ac
0000000   sp  sp  sp  sp  sp  sp  sp   4  sp  sp  sp  sp  sp  sp   2   9
                                       4                           2   9
0000020   sp  sp  sp  sp  sp   1   7   7  nl                            
                               1   7   7  \n                            
0000031

# My output
>> cargo run -- < tests/inputs/atlamal.txt | od -ac
0000000   sp  sp  sp  sp  sp  sp  sp   4  sp  sp  sp  sp  sp  sp   2   9
                                       4                           2   9
0000020   sp  sp  sp  sp  sp   1   7   7  sp  nl                        
                               1   7   7      \n                        
0000032
```
Have you noticed the extra space at the end? :)

# TODO

* Add support for grapheme count instead of simple char counts
* Support `--max-line-length` and `--files0-from=FILE`
* Use criterion to generate perf benchmarks for large files/lines

# Reference

* https://doc.rust-lang.org/book/ch08-02-strings.html
* https://docs.oracle.com/cd/E88353_01/html/E37839/wc-1.html