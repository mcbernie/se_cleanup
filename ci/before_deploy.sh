# This script takes care of building your crate and packaging it for release

set -ex

export OPENSSL_STATIC=0
export OPENSSL_DIR=/tmp/openssl-1.1.1-win64-mingw
export OPENSSL_LIB_DIR=/tmp/openssl-1.1.1-win64-mingw/lib
export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.1.1-win64-mingw/include

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    export OPENSSL_STATIC=0
    export OPENSSL_DIR=/tmp/openssl-1.1.1-win64-mingw
    export OPENSSL_LIB_DIR=/tmp/openssl-1.1.1-win64-mingw/lib
    export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.1.1-win64-mingw/include

    ls -All /tmp/openssl-1.1.1-win64-mingw
    # TODO Update this to build the artifacts that matter to you
    env OPENSSL_STATIC=0 OPENSSL_DIR=/tmp/openssl-1.1.1-win64-mingw cross rustc --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    cp target/$TARGET/release/se_shell $stage/
    cp target/$TARGET/release/se_shell.exe $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main