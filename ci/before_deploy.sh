# This script takes care of building your crate and packaging it for release

set -ex



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

    echo "environment variables:"
    echo $OPENSSL_STATIC
    echo $X86_64_PC_WINDOWS_GNU_OPENSSL_DIR
    echo $X86_64_PC_WINDOWS_GNU_OPENSSL_LIB_DIR
    echo $X86_64_PC_WINDOWS_GNU_OPENSSL_INCLUDE_DIR
    echo "=================== Thank you for flying with Deutsche Bahn ==="

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    #cross rustc --target $TARGET --release -- -C lto
    cargo build --target $TARGET --release

    # TODO Update this to package the right artifacts
    #cp target/$TARGET/release/se_cleanup $stage/se_shell
    cp target/$TARGET/release/se_shell.exe $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main