# Mobilisation

A weekend project : creating a client for [Mobilizon](https://mobilizon.org/) !

<img width="1399" height="925" alt="image" src="https://github.com/user-attachments/assets/2c4dd4dc-9af1-42ce-bf37-7389dbb333d2" />

## Status

This is my Sunday morning relax project, do not expect fast changes.
If you want to give a hand, contributions are welcome !

Loosely, the next goals are :

- user can login
- user can switch instance
- user can see the details of an event
- user can register to an event
- app is fully mobile ready
- android release (if/when Gtk supports the android backend)

# Build and test

First create the build directory

`meson setup builddir`

Then compile

`meson compile -C ./builddir`

And test

`meson test -C ./builddir`
