# Netwide Compiler
Execute your code on the cloud

## Installation:
```shell
$ cargo install --git "https://github.com/Pranjal-Patel/netwide-compiler"
```

## Usage:
```shell
$ nwc
Execute your code on the cloud

Usage: nwc [OPTIONS] <language>

Arguments:
  <language>  Language of the code type list for getting a list of languages

Options:
  -f, --file <file path>  [required] Input code file
  -t, --target <target>   Target
      --list-targets      List all the available targets for a specific language
  -h, --help              Print help
  -V, --version           Print version
```

##### How to execute some code:
Here's the rust code to print `Hello World`
```rs
fn main() {
    println!("Hello World");
}
```

To run:
<br>
syntax: `nwc <lang> -f <file path>`
```shell
$ nwc rust -f main.rs
Program output =====
Hello World
=====
```

## Misc:
#### To get a the list of available languages
```shell
$ nwc list
List of languages:
...
```

#### Available targets for a specific language
```shell
$ nwc javascript --list-targets
List of targets for 'javascript':
nodejs-16.14.0
nodejs-14.16.1
nodejs-12.22.1
nodejs-10.24.1
spidermonkey-88.0.0
```