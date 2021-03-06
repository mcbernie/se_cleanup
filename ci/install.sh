set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup target install armv7-apple-ios
            ;;
        armv7s-apple-ios)
            rustup target install armv7s-apple-ios
            ;;
        i386-apple-ios)
            rustup target install i386-apple-ios
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
        x86_64-pc-windows-gnu)
            rustup target add x86_64-pc-windows-gnu
            ;;
    esac

    # install mingw-w64
    #if [ $TARGET = x86_64-pc-windows-gnu ]; then
    #    apt-get install mingw-w64
    #fi


    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target

    curl -LSfs "https://bintray.com/vszakats/generic/download_file?file_path=openssl-1.1.1-win64-mingw.tar.xz" > /tmp/unpack.tar.xz 
    tar -xf /tmp/unpack.tar.xz -C /tmp
    ls -All /tmp
    ls -All /tmp/openssl-1.1.1-win64-mingw
}

main