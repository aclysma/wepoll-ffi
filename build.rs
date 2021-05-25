fn main() {
    cc::Build::new()
        .file("vendor/wepoll/wepoll.c")
        .compile("wepoll");

    // We regenerate binding code and check it in. (See generate_bindings.bat)
}
