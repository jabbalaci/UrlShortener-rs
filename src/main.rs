mod bitly;
mod clipboard;
mod config;

use std::env;
use std::process;

use crate::config as cfg;

use colored::Colorize;
use jabba_lib::jconsole;

/// Removes the trailing `'/'` character from the expanded URL if
/// the original URL didn't have a trailing `'/'` character.
fn trim_trailing_slash_if_necessary<'a>(long_url: &str, expanded_url: &'a str) -> &'a str {
    if !long_url.ends_with('/') && expanded_url.ends_with('/') {
        &expanded_url[..expanded_url.len() - 1]
    } else {
        expanded_url
    }
}

/// Verifies if the API key is available.
///
/// It tries to read the API key from the environment variable `BITLY_ACCESS_TOKEN`.
/// If the API key is not available, then the program terminates.
fn check_api_key() {
    if cfg::API_KEY.is_empty() {
        let msg = r#"
Error: your bit.ly access token was not found.
Tip: put it in the environment variable called BITLY_ACCESS_TOKEN
Tip: on the home page of bit.ly you can generate one for free.
"#
        .trim();
        eprintln!("{}", msg);
        process::exit(1);
    }
}

/// Returns a tuple of two elements. The first element is a URL.
/// The second element is a bool, which shows how the URL was
/// obtained. `true` means interactive input. `false` means command-line input.
///
/// Takes the first command-line argument.
/// If no command-line argument was given, then it reads the URL from the keyboard.
fn read_url() -> (String, bool) {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        (jconsole::input("Long URL: "), true)
    } else {
        (args[0].to_owned(), false)
    }
}

/// Asks the user if (s)he wants to copy the short URL to the clipboard.
fn ask_to_copy_to_clipboard(url: &str) {
    println!();
    let resp = jconsole::input("Copy shortened URL to clipboard [Yn]? ");
    if resp == "" || resp == "y" || resp == "Y" {
        clipboard::set_text(url);
        println!("# copied")
    } else {
        println!("# no");
    }
}

/// Prints an error message and terminates the program.
fn print_error_and_exit(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

/// Entry point.
fn main() {
    check_api_key(); // may exit

    let (long_url, is_interactive) = read_url();
    if long_url.is_empty() {
        print_error_and_exit("Provide a valid URL");
    }
    let url_id = bitly::shorten(&long_url);
    if url_id.is_err() {
        print_error_and_exit("Error: couldn't process the response from bit.ly");
    }
    let url_id = url_id.unwrap();
    let short_url = format!("https://{url_id}");
    println!();
    println!("{}", short_url.bold());
    println!();
    let expanded_url = bitly::expand(&url_id);
    if expanded_url.is_err() {
        print_error_and_exit("Error: couldn't process the response from bit.ly");
    }
    let expanded_url = expanded_url.unwrap();
    let expanded_url = trim_trailing_slash_if_necessary(&long_url, &expanded_url);
    println!(
        "# expanded from shortened URL: {} {}",
        expanded_url,
        if long_url == expanded_url {
            format!("{}", "(matches)".green().bold())
        } else {
            format!("{}", "(does NOT match)".red().bold())
        }
    );
    if is_interactive {
        ask_to_copy_to_clipboard(&short_url);
    }
}
