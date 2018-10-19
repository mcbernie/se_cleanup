# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    env OPENSSL_STATIC=0
    env OPENSSL_DIR=/tmp/openssl-1.1.1-win64-mingw
    env OPENSSL_LIB_DIR=/tmp/openssl-1.1.1-win64-mingw/lib
    env OPENSSL_INCLUDE_DIR=/tmp/openssl-1.1.1-win64-mingw/include
    cross run --target $TARGET
    cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi