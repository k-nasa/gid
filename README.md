## gid (Grasp issue dependencies)

[![Actions Status](https://github.com/k-nasa/gid/workflows/CI/badge.svg)](https://github.com/k-nasa/gid/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/gid_cli.svg)](https://crates.io/crates/gid_cli)
[![GitHub license](https://img.shields.io/github/license/k-nasa/gid)](https://github.com/k-nasa/gid/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/k-nasa/gid?style=social)](https://github.com/k-nasa/gid/stargazers)

Help project managers and project owners with easy-to-understand views of github issue dependencies.

<img width="640" alt="スクリーンショット 2022-04-10 10 36 22" src="https://user-images.githubusercontent.com/23740172/162638576-77ce945e-1b46-43af-a824-44798067be55.png">

This tool can be used on github action or the command line.
Basically, it is supposed to be used in Github Action.


It relies on GitHub's builtin mermaid support. The output format may increase in the future.

Currently it only supports analysis with the [tracking feature of GitHub issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/about-task-lists).
Issue Description tasks and issue link analysis will be added in the future.

## DEMO

### [GitHub Actions](https://github.com/marketplace/actions/grasp-issue-dependencies)

Adding the following workflow will analyze the issue every minute and comment on the diagram. (This is an example of parsing an issue labeled "root".)

```yml
name: Comment gid
on:
  schedule:
    - cron: '* * * * *'

jobs:
  grasp_issue:
    runs-on: macOS-latest
    name: Grasp issue dependencies
    steps:
      - uses: actions/checkout@v3
      - uses: k-nasa/gid@main
        with:
          label: 'root' # Analyze the issue with the specified label
          github_token: ${{secrets.GITHUB_TOKEN}}
```

Comment by github actions.

<img width="934" alt="スクリーンショット 2022-04-10 0 20 35" src="https://user-images.githubusercontent.com/23740172/162580458-c81677c0-f171-4eda-8e8b-c9b9bff38691.png">

### CLI

Let's analyze [issue 1](https://github.com/k-nasa/gid/issues/1) as a trial.

```sh
gid -o k-nasa -r gid -i 1
```

This command outputs the following figure. (To be exact, the mermaid script that is the basis of the figure is output.)

```mermaid
graph LR

classDef CLOSED fill:#8256d0,color:#FFFFFF,stroke-width:0px;
classDef OPEN fill:#347d39,color:#FFFFFF,stroke-width:0px;

2 --> 4["Child 1"]:::CLOSED
2 --> 5["Child 2"]:::CLOSED
2 --> 6["Child 3"]:::CLOSED
7 --> 8["Child 2"]:::OPEN
7 --> 9["Child 1"]:::OPEN
9 --> 10["Grandchild1"]:::OPEN
9 --> 11["Grandchild2"]:::CLOSED
1 --> 2["DEMO 1"]:::CLOSED
1 --> 3["DEMO2"]:::OPEN
1 --> 7["DEMO3"]:::OPEN

click 4 href "https://github.com/k-nasa/gid/issues/4" _blank
click 5 href "https://github.com/k-nasa/gid/issues/5" _blank
click 6 href "https://github.com/k-nasa/gid/issues/6" _blank
click 8 href "https://github.com/k-nasa/gid/issues/8" _blank
click 9 href "https://github.com/k-nasa/gid/issues/9" _blank
click 10 href "https://github.com/k-nasa/gid/issues/10" _blank
click 11 href "https://github.com/k-nasa/gid/issues/11" _blank
click 2 href "https://github.com/k-nasa/gid/issues/2" _blank
click 3 href "https://github.com/k-nasa/gid/issues/3" _blank
click 7 href "https://github.com/k-nasa/gid/issues/7" _blank
```

## Usage

```sh
gid 0.1.0
k-nasa <htilcs1115@gmail.com>
Issue graphical tool

USAGE:
    gid --organization <ORGANIZATION> --repository <REPOSITORY> --issue-number <ISSUE_NUMBER>

OPTIONS:
    -h, --help                           Print help information
    -i, --issue-number <ISSUE_NUMBER>
    -o, --organization <ORGANIZATION>
    -r, --repository <REPOSITORY>
    -V, --version                        Print version information
```

## Install

curl

```sh
curl -L -o gid.tar.gz https://github.com/k-nasa/gid/releases/download/0.1.0/gid_x86_64-apple-darwin.tar.gz
tar -zxvf gid.tar.gz

# Move binary file to the path
mv gid_x86_64-apple-darwin/gid /usr/local/bin
```

cargo


```sh
cargo install gid_cli
```

## Contribution

1. Fork it (http://github.com/k-nasa/gid)
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## License

[MIT](https://github.com/k-nasa/gid/blob/master/LICENSE)

## Author

[k-nasa](https://github.com/k-nasa)
