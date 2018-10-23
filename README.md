
![GitHub release](https://img.shields.io/github/release/mcbernie/se_cleanup.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/4hk5du58es6ur9o5?svg=true)](https://ci.appveyor.com/project/mcbernie/se-cleanup) [![Build Status](https://travis-ci.org/mcbernie/se_cleanup.svg?branch=master)](https://travis-ci.org/mcbernie/se_cleanup)
# Simple Shell Helper tool for SE

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

    
