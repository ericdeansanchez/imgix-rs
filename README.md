# Unofficial imgix crate

[![Build Status](https://travis-ci.org/ericdeansanchez/imgix-rs.svg?branch=master)](https://travis-ci.org/ericdeansanchez/imgix-rs)

## Overview

The imgix-rs crate provides functionality for programmatically constructing imgix-urls. This crate is a work in progress and **is not currently ready for production use**.

This crate seeks to be explicit and _correct_. It is also currently a side-project (and unofficial). Right now, the project structure looks like this:
```text
.
└──  src
   ├── bin
   └── imgix
```

Where the `bin/` directory represents the cli portion of the project and the `imgix/` directory represents the crate that the cli and users use. The project may be split into separate crates in the future: one for the cli and one for the crate the cli depends on.

For a more detailed description of this project's architecture [read this](Architecture.md).

## Contents

- [Unofficial imgix crate](#unofficial-imgix-crate)
  - [Overview](#overview)
  - [Contents](#contents)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Contributing](#contributing)
  - [Prerequisites](#prerequisites)
  - [Clone](#clone)
  - [Build](#build)
  - [Test](#test)
  - [Read](#read)
    - [Publicly](#publicly)
    - [Locally](#locally)
  - [Run](#run)

## Requirements

imgix-rs is written in Rust and currently only depends on `clap`.

## Installation

Coming soon! For now, you can `git clone` this repository.

## Contributing

Contributions are welcome! No contribution is too small––bug fix, a new feature, feature-request, or a typo fix––all are welcome.

* contribution [template]()
* issue [template]()

## Prerequisites

imgix-rs is written in Rust so make sure you have [Rust installed](https://www.rust-lang.org/tools/install.)


## Clone

Clone the repository:

```bash
$ git clone https://github.com/ericdeansanchez/imgix-rs.git
```

## Build

cd into the repository and run:

```bash
$ cargo build
```

## Test

Ensure the tests pass on your system (please open an issue if they do not):

```bash
$ cargo test
```

## Read

### Publicly 
The published docs can be found [here](https://docs.rs/imgix/0.1.0/imgix/struct.Url.html) at docs.rs.

### Locally
Copy & paste the command below in your terminal. Make sure `--no-deps` is passed otherwise you'll build documentation for all/any dependencies.

```bash
$ cargo doc --no-deps --open
```

## Run
Calling `run` without arguments
```bash
$ cargo run
```
leads to the cli help message.