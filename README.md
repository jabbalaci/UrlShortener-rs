# Url Shortener (Rust)

A URL shortener command-line app. for [bit.ly](https://bitly.com/).
Implemented in Rust, using bit.ly's API v4.

It was tested under Linux and Windows.

You can use it in interactive and non-interactive modes.
In interactive mode, you have the possibility to copy the shortened URL
to the clipboard.

## Interactive mode

```
$ urlshortener
Long URL: https://google.com

https://bit.ly/2R9zFOR

# expanded from shortened URL: https://google.com (matches)

Copy shortened URL to clipboard [Yn]? y
# copied
```

## Non-interactive mode

```
$ urlshortener https://google.com
https://bit.ly/2R9zFOR

# expanded from shortened URL: https://google.com (matches)
```

## Pre-requisites

For this to work, you need an access token from bit.ly. Don't worry, it's free.
After registration you can generate one for yourself. Then, add it as an
environment variable called `BITLY_ACCESS_TOKEN`. For instance, under Linux
add the following line to the end of your `~/.bashrc` file:

```bash
export BITLY_ACCESS_TOKEN="..."
```

Under Linux, the copy to clipboard functionality requires the Linux
command `xsel`. You can install it via your package manager.

## Installation

If you have the Rust compiler, you can install it directly
from crates.io using the command `cargo`:

    $ cargo install bitly-urlshortener

## Related projects

* I used [pyshorteners](https://github.com/ellisonleao/pyshorteners/blob/master/pyshorteners/shorteners/bitly.py) to figure out how to call bit.ly's API v4.

* I have a [Nim implementation](https://github.com/jabbalaci/UrlShortener-nim) but that one uses an older API of bit.ly (v3).

* I have a [C# implementation](https://github.com/jabbalaci/UrlShortener-cs) too, which uses
the newer API of bit.ly (v4).
