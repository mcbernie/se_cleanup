
![GitHub release](https://img.shields.io/github/release/mcbernie/se_cleanup.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/4hk5du58es6ur9o5?svg=true)](https://ci.appveyor.com/project/mcbernie/se-cleanup) [![Build Status](https://travis-ci.org/mcbernie/se_cleanup.svg?branch=master)](https://travis-ci.org/mcbernie/se_cleanup)
# Simple Shell Helper tool for SE

## What it does
After startup Windows, it runs as a shell, clean old log files, ensure that PasswordExpires is false and open an connection to mqtt server.
it response on ping messages with an pong and product verion, opens an vnc connection on asking and automatically updates if a newer version is in temp folder.
the automatic updates add a runonce entry in registry with an simple copy command, then the programm calls an restart with wmic

- BTW BAD ENGLISH ;)

## Compiles for the following os's
- win64 
- use appveyor

## Steps
- after push to master, running all tests and ci runs successfull,
- create new tag and push to tag to trigger new deployment:
    ```bash
    git commit -a -m 'commit all changes'
    git tag -a v0.0.0 master -f -m 'release informations'
    git push origin v0.0.0 -f 
    ```
## Ready
- set an newer / or other version for se_shell.exe in update.txt then every clients get the new version after reboot


## To compile windows on macos:
- install mingw-w64:
    ```bash
    brew install mingw-w64
    ```
