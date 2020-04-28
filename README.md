# Git-Duet: Wrapper for SOPS

I use SOPS for encrypting a few bits an pieces. One thing I like to do
is keep my Dotfiles in a repository, so I thought it might be a neat
thing to keep my git duet authors files encrypted. This saves my pairs
emails from being reveled to the world.

## Setup

I have it setup like this

    tree ~/.bin/git-*
    /Users/billie/.bin/git-duet-pre-commit -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-solo -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-install-hook -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-merge -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-revert -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-prepare-commit-msg -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-post-commit -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet -> /usr/local/bin/git-duet-wrap-for-sops
    /Users/billie/.bin/git-duet-commit -> /usr/local/bin/git-duet-wrap-for-sops

With `$HOME/.bin` on my path.

The name of the binary is used by my script to decide what git duet
command to call.

It expects the your authors file to be encrypted at
`$HOME/.config/git-duet/authors.yml` and you to have a service account
key at `$HOME/.config/gcloud/application_sops_credentials.json`

## Usage

Once setup simply use git-duet as normal.

## Installing

First tap my homebrew repo

``` shell
brew tap PurpleBooth/repo
```

Next install the binary

``` shell
brew install PurpleBooth/repo/git-duet-wrap-for-sops
```

You can also download the [latest
release](https://github.com/PurpleBooth/git-duet-wrap-for-sops/releases/latest)
and run it.

## License

[CC0](LICENSE.md) - Public Domain
