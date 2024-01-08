[no-cd]
[private]
install-cmd options src dest:
    install {{options}} {{src}} {{dest}}

[no-cd]
[private]
install-bin src dest: (install-cmd '-Dm0755' src dest)

[no-cd]
[private]
install-file src dest: (install-cmd '-Dm0644' src dest)
