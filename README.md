# Nosey Parker: Find secrets in textual data

Nosey Parker is a command-line tool that finds secrets and sensitive information in textual data. It is useful both for offensive and defensive security testing.

**Key features:**
- It supports scanning files, directories, and the entire history of Git repositories
- It uses regular expression matching with a set of 60 patterns chosen for high signal-to-noise based on experience and feedback from offensive security engagements
- It groups matches together that share the same secret, further emphasizing signal over noise
- It is fast: it can scan at hundreds of megabytes per second on a single core, and is able to scan 100GB of Linux kernel source history in less than 5 minutes on an older MacBook Pro

This open-source version of Nosey Parker is a reimplementation of part of the internal version in use at Praetorian, which has additional machine learning capabilities. Read more in blog posts [here](https://www.praetorian.com/blog/nosey-parker-ai-secrets-scanner-release/) and [here](https://www.praetorian.com/blog/six-months-of-finding-secrets-with-nosey-parker/).


## Building from source

**NOTE:** Currently only x86_64 is supported due to the dependency on Hyperscan, which only supports x86_64.


**1. Install the [Hyperscan](https://github.com/intel/hyperscan) library and headers for your system**

On macOS using Homebrew:
```
brew install hyperscan pkg-config
```

On Ubuntu 22.04:
```
apt install libhyperscan-dev pkg-config
```

**2. Install the Rust toolchain**

Recommended approach: install from <https://rustup.rs>

**3. Build using [Cargo](https://doc.rust-lang.org/cargo/)**

```
cargo build --release
```
This will produce a binary at `target/release/noseyparker`.

## Docker Usage

**NOTE:** Currently only x86_64 is supported due to the dependency on Hyperscan, which only supports x86_64.

1. Build the Docker image: 

```
docker build -t noseyparker . 
```

2. Run Docker image with mounted volume:

```
docker run -v `pwd`:/opt/ noseyparker
```

## Usage quick start


### The datastore
Most Nosey Parker commands use a _datastore_.
This is a special directory that Nosey Parker uses to record its findings and maintain its internal state.
A datastore will be implicitly created by the `scan` command if needed.
You can also create a datastore explicitly using the `datastore init -d PATH` command.


### Scanning filesystem content for secrets
Nosey Parker has built-in support for scanning files, recursively scanning directories, and scanning the entire history of Git repositories.

For example, if you have a Git clone of [CPython](https://github.com/python/cpython) locally at `cpython.git`, you can scan its entire history with the `scan` command.
Nosey Parker will create a new datastore at `np.cpython` and saves its findings there.
```
$ noseyparker scan --datastore np.cpython cpython.git
Found 28.30 GiB from 18 plain files and 427,712 blobs from 1 Git repos [00:00:04]
Scanning content  ████████████████████ 100%  28.30 GiB/28.30 GiB  [00:00:53]
Scanned 28.30 GiB from 427,730 blobs in 54 seconds (538.46 MiB/s); 4,904/4,904 new matches

 Rule                      Distinct Groups   Total Matches
───────────────────────────────────────────────────────────
 PEM-Encoded Private Key             1,076           1,192
 Generic Secret                        331             478
 netrc Credentials                      42           3,201
 Generic API Key                         2              31
 md5crypt Hash                           1               2

Run the `report` command next to show finding details.
```

You can specify multiple inputs to scan at once in any combination of the supported input types (files, directories, and Git repos).


### Summarizing findings
Nosey Parker prints out a summary of its findings when it finishes
scanning.  You can also run this step separately:
```
$ noseyparker summarize --datastore np.cpython

 Rule                      Distinct Groups   Total Matches
───────────────────────────────────────────────────────────
 PEM-Encoded Private Key             1,076           1,192
 Generic Secret                        331             478
 netrc Credentials                      42           3,201
 Generic API Key                         2              31
 md5crypt Hash                           1               2
```


### Reporting detailed findings
To see details of Nosey Parker's findings, use the `report` command.
This prints out a text-based report designed for human consumption:
```
$ noseyparker report --datastore np.cpython
Finding 1/1452: Generic API Key
Match: QTP4LAknlFml0NuPAbCdtvH4KQaokiQE
Showing 3/29 occurrences:

    Occurrence 1:
    Git repo: clones/cpython.git
    Blob: 04144ceb957f550327637878dd99bb4734282d07
    Lines: 70:61-70:100

        e buildbottest

        notifications:
          email: false
          webhooks:
            urls:
              - https://python.zulipchat.com/api/v1/external/travis?api_key=QTP4LAknlFml0NuPAbCdtvH4KQaokiQE&stream=core%2Ftest+runs
            on_success: change
            on_failure: always
          irc:
            channels:
              # This is set to a secure vari

    Occurrence 2:
    Git repo: clones/cpython.git
    Blob: 0e24bae141ae2b48b23ef479a5398089847200b3
    Lines: 174:61-174:100

        j4 -uall,-cpu"

        notifications:
          email: false
          webhooks:
            urls:
              - https://python.zulipchat.com/api/v1/external/travis?api_key=QTP4LAknlFml0NuPAbCdtvH4KQaokiQE&stream=core%2Ftest+runs
            on_success: change
            on_failure: always
          irc:
            channels:
              # This is set to a secure vari
...
```


### Getting help
Running the `noseyparker` binary without arguments prints top-level help and exits.
You can get abbreviated help for a particular command by running `noseyparker COMMAND -h`.
More detailed help is available with the `help` command.
For example:
```
$ noseyparker scan -h
Scan content for secrets

Usage: noseyparker scan [OPTIONS] --datastore <PATH> <INPUT>...

Arguments:
  <INPUT>...  Paths of inputs to scan

Options:
  -d, --datastore <PATH>  Use the specified datastore path [env: NP_DATASTORE=]
  -j, --jobs <N>          The number of parallel scanning jobs [default: 12]
  -r, --rules <PATH>      Path of custom rules to use
  -h, --help              Print help information (use `--help` for more detail)

Content Discovery Options:
      --max-file-size <MEGABYTES>  Do not scan files larger than the specified size [default: 100]
  -i, --ignore <FILE>              Path of a custom ignore rules file to use

Global Options:
  -v, --verbose...       Enable verbose output
      --color <MODE>     Enable or disable colored output [default: auto] [possible values: auto, never, always]
      --progress <MODE>  Enable or disable progress bars [default: auto] [possible values: auto, never, always]
```


## Contributing
Contributions are welcome, particularly new regex rules.
Developing new regex rules is detailed in a [separate document](docs/RULES.md).

If you are considering making significant code changes, please [open an issue](https://github.com/praetorian-inc/noseyparker/issues/new) first to start discussion.


## License
Nosey Parker is licensed under the [Apache License, Version 2.0](LICENSE-APACHE).

Any contribution intentionally submitted for inclusion in Nosey Parker by you, as defined in the Apache 2.0 license, shall be licensed as above, without any additional terms or conditions.
