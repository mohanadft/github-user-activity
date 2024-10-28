# `github-user-activity` — A Rust-Powered GitHub Activity Tracker 🚀

Welcome to `github-user-activity`, a fast and efficient utility written in Rust to track GitHub user activity! 🦀  
This tool allows you to get a summary of a GitHub user's contributions, repositories, and other activities with ease.

[github-user-activity Project Description on Roadmap.sh](https://roadmap.sh/projects/github-user-activity)

## 🚀 Features

- [ ] **Contribution Count**: Fetch the number of contributions the user has made in the past year.
- [ ] **Public Repositories**: Get the list of public repositories owned by the user.
- [x] **Followers**: Retrieve the number of followers.
- [x] **Gists**: See the number of public gists by the user.
- [x] **Recent Activity**: Check out the user’s latest events and activity on GitHub.

## 📦 Installation

Installing `github-user-activity` is a breeze! First, make sure you have Rust installed. Then, run the following commands:

```bash
  git clone https://github.com/mohanadft/github-user-activity.git  # Clone the Repo
  cd github-user-activity                                          # Go inside the package
  cargo install --path .                                           # Install the package globally
```

## Pre Usage 🧪

Before using the package, you have to provide your own github api key, so you can authenticate when fetching github API. [See this page for more info on how to get your API key](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)

After getting your API credential, you just want to export the `TOKEN` environment variable in order for the program to recognize it.

```bash
export TOKEN=[Your API Key]
```
Note: environment variables are temporarily availabe based on the session, so whenever you want to use the program you have to export it once before using, but if you exit your terminal (terminal tab), and opened a new one, you have to export it again.
## ⌨️  Usage

Simple and straightforward! Just provide a GitHub username as the argument to fetch the user’s activity summary.

```bash
  ghua [OPTIONS] <USERNAME>

  Arguments:
    <USERNAME>    GitHub username to fetch activity for

  Options:
    -f, --followers     Number of Followers
    -p, --public-gists  Number of Public Gists
    -h, --help          Print help information
    -V, --version       Print version information
```

## 🤝 Contributing

Want to add new features, fix bugs, or improve the codebase? PRs are welcome! Check out our CONTRIBUTING.md to get started.
