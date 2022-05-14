rm -rf ./docs
cargo doc --all --no-deps --manifest-path pangalacticcc/Cargo.toml --target-dir ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=doc/pangalacticcc\">" > ./docs/index.html
