# NAME

corre - Execute shell scripts embedded within text

# SYNOPSIS

**corre** [*OPTIONS*]\...

# DESCRIPTION

Executes shell scripts nested within text as marked by some delimiter, and then
replaces the shell script with its STDOUT.

## AS AN EXECUTABLE

By default, the input corre reads from is the STDIN and the output is printed
to the STDOUT.  These can be redirected via file redirection or the `-i` and
`-o` flags.

The default delimiter used to mark an embedded script is the opening tag "!(("
and the closing tag "))!".  The delimiter can be changed with the `-d` flag.

The embedded scripts are called from the directory that corre is called from.

## AS A LIBRARY

Corre exports only a single function:

```rs
corre::run_embedded_scripts(
    text: String,       // The input text
    opening_tag: &str,  // The opening tag that marks a script start
    closing_tag: &str,  // The closing tag that marks a script end
    recur: bool,        // Whether or not to recur on script outputs
) -> Result<String, PopenError>
```

Simply pass the input text and the delimiters, and corre will return the
processed output text.

# INSTALL

Requires the rust toolchain and the cargo CLI.

    cargo install corre

# OPTIONS

-i, -\-input
: The input file.  Defaults to the STDIN.

-o, -\-output
: The output file.  Defaults to the STDOUT.

-d, -\-delimiters
: Overrides the default delimiters "!(( ))!".  Separate the opening and
  closing tags with a space.

-r, -\-recur
: Recursively operate on the script outputs.

-h, -\-help
: Prints a help menu.

-V, -\-version
: Prints the current version of the program.

# EXAMPLES

## PARTIAL FILES

Corre can be used to include partial files in other files as such:

```md
<!-- ./par/main.md -->

[[[cat ./par/main.md]]]
```

```md
<!-- ./par/partial.md -->

[[[cat ./par/partial.md]]]
```

```
corre -i ./par/main.md -o ./par/processsed_main.md
```

```md
<!-- ./par/processsed_main.md -->

[[[cargo run -- -i ./par/main.md]]]
```

It may be useful to pass the `-r` flag to operate recursively on script outputs
if you plan to have multiple layers of inclusion.  Otherwise, you would have
to call corre yourself before calling `cat`.

## DIRECTORY LISTINGS

```
[[[cat ./par/directory.html]]]
```

```
[[[cargo run -- -i ./par/directory.html]]]
```

# BUGS

1. The characters "<" and ">" cannot be used as a part of a delimiter.

# SEE ALSO

- [Source code](https://github.com/ctwiebe23/corre)
- [Online README](https://ctwiebe23.github.io/corre)
- [CHANGELOG](https://ctwiebe23.github.io/corre/changelog)
