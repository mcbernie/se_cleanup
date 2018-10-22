[![Build status](https://ci.appveyor.com/api/projects/status/plwortay465b02ih/branch/master?svg=true)](https://ci.appveyor.com/project/mcbernie/se-cleanup/branch/master) [![Build Status](https://travis-ci.org/mcbernie/se_cleanup.svg?branch=master)](https://travis-ci.org/mcbernie/se_cleanup)

# Simple Shell Helper tool for SE

## Compile
env OPENSSL_STATIC=0 OPENSSL_LIB_DIR=/Users/nico/test/openssl-1.1.1-win64-mingw/lib OPENSSL_INCLUDE_DIR=/Users/nico/test/openssl-1.1.1-win64-mingw/include cargo build --target=x86_64-pc-windows-gnu