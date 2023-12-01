<div align="center">
    <img src="./assets/aoc-banner.jpeg" title="Advent of Code Banner" alt="Advent of Code Banner" width='300' align="right"/>

  <h1 align="center">Advent of Code</h1>

  [Advent of Code](https://adventofcode.com/2023/about) is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like, created by [Eric Wastl](http://was.tl/).

  I love the aspect of Advent of Code and want to participate it in every year. I started late, but intend on retroactively going back and completing available puzzles from previous years eventually.

  <img src="https://img.shields.io/github/stars/Toxocious/AdventOfCode?style=for-the-badge&logo=appveyor" />
  <a href="https://visitorbadge.io/status?path=https%3A%2F%2Fgithub.com%2FToxocious%AdventOfCode">
    <img src="https://api.visitorbadge.io/api/visitors?path=https%3A%2F%2Fgithub.com%2FToxocious%AdventOfCode&countColor=%2337d67a" />
  </a>
  <br />

  <a href="https://discord.gg/Km6btPhs" target="_blank">
    <img src="https://discord.com/api/guilds/1002005327555862620/widget.png?style=banner2" alt="Discord Banner" />
  </a>
</div>



# Table of Contents
- [Table of Contents](#table-of-contents)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Required Dependencies](#required-dependencies)
  - [Installation](#installation)
  - [Available Commands](#available-commands)
    - [Generate New Day Template](#generate-new-day-template)
    - [Code Runner and Watcher](#code-runner-and-watcher)
    - [Code Linting](#code-linting)
- [License](#license)



# Getting Started
## Prerequisites
I've opted to use Rust for my Advent of Code runs, so you'll need to install it on your machine.

You can find the correct installation procedure for your machine on the [Install Rust](https://www.rust-lang.org/tools/install) page of the official website.

## Required Dependencies
Be sure to install the required dependencies.
```sh
$ cargo install cargo-generate
$ cargo install just
```

**just** - Used to simplify the available commands for ease-of-use
**cargo-generate** - Used to generate new day scaffolding from the provided template

## Installation
Clone the repository with the following command in your terminal or download it through GitHub's website.

```bash
$ git clone https://github.com/Toxocious/Advent-Of-Code.git
```

After cloning the repo onto your machine, open a terminal into the repository.
Creating New Day Directory
From here, you can choose which year's puzzles to work on or run.

## Available Commands
I've set up this repository in such a way that generating new days and running each day's code is very easy.

### Generate New Day Template
```sh
$ just make day-01
```
This will copy the template day code into `day-01`, which you can then edit and run later.

### Code Runner and Watcher
To run the code for an existing day, run:
```sh
$ just work day-01
```
This will run your code for a given day, run any tests for the day, and also run linting for the code inside of the directory as well.

It will also watch for changes and log all of this to the terminal when any file changes have happened.

### Code Linting
You can choose to only lint a given day by running:
```sh
$ just lint day-01
```


# License
This project is not licensed, meaning you can do whatever you want with the code that is inside of this repository.

Do consider and uphold the licenses of any and all dependencies that may lie in this repository.
