# Unofficial imgix crate

[![Build Status](https://travis-ci.org/ericdeansanchez/imgix-rs.svg?branch=master)](https://travis-ci.org/ericdeansanchez/imgix-rs)

## Overview

The imgix-rs crate provides functionality for programmatically constructing imgix-urls.

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
  - [Run](#run)

## Requirements

imgix-rs is written in Rust and currently only depends on `clap`.

## Installation

Coming soon! For now, you can `git clone` this repository.

# Contributing

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

These are the docs you really want. Copy & paste the command below in your 
terminal. Make sure `--no-deps` is passed otherwise you'll build documentation 
for all/any dependencies.

```bash
$ cargo doc --no-deps --open
```

## Run