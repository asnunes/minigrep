# minigrep

Minigrep is a Rust CLI application to match strings in a given text file. It's one of the projects of "The Rust Programming Language" book by Steve Klabnik and Carol Nichols.

The application expects two arguments: the string to look for and the file where it should search:

```
# my_file.txt
I should write a line here, but what should I say?
I should also write another one because it is two results, but.... This line will match?
And this?
```


```bash
$ minigrep but my_file.txt

I should write a line here, but what should I say?
I should also write another one because it is two results, but.... This line will match?
```

It also accepts case insensitive searches. Just set CASE_INSENSITIVE env variable:

```bash
$ CASE_INSENSITIVE=1 minigrep "and this" my_file.txt

And this?
```

# building

Make sure you have Rust installed (available at https://www.rust-lang.org/). To compile just clone this repository and run the following command:

```bash
cargo build --release
```