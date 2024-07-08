#!/bin/sh

# Check if sed is available
if ! which sed > /dev/null; then
    echo "sed could not be found"
    exit 1
fi

# Check if cargo-expand is available
if ! cargo expand --version > /dev/null; then
    echo "cargo-expand could not be found"
    exit 1
fi

# wrapper around sed to account for mac/bsd/linux differences
sed_() {
    # sed --version doesn't exist on macos
    if ! sed --version > /dev/null 2>&1; then
        args=$*
        set --
        for arg in $args; do
            if [ "$arg" = "-i" ]; then
                set -- "$@" "$arg" ""
            else
                set -- "$@" "$arg"
            fi
        done
    fi
    $(which sed) "$@"
}

sed_ -i 's/serde_versioning::Deserialize/serde::Deserialize/g' examples/usage.rs
sed_ -i 's/#\[versioning/\/\/#\[versioning/g' examples/usage.rs
cargo expand --example usage > usage.expanded.initial.rs
sed_ -i 's/serde::Deserialize/serde_versioning::Deserialize/g' examples/usage.rs
sed_ -i 's/\/\/#\[versioning/#\[versioning/g' examples/usage.rs
cargo expand --example usage > usage.expanded.modified.rs
diff -u usage.expanded.initial.rs usage.expanded.modified.rs > usage.diff

# clean-up
rm usage.expanded.initial.rs usage.expanded.modified.rs

exit 0
