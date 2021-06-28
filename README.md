# ssb
## Bookmarks for your ssh connections

`ssb` is a small tool to make managing ssh connections easier. Instead of
having to type `ssh user@hostname` each time, user/hostname combinations can
be saved and referred to by a shortform name.

## Usage
### Starting a connection
```
>>> ssb rpi
```
If invoked without any flags, `ssb` will interpret the first argument as the
name of a bookmark and attempt to initiate an ssh connection. The `ssb` process
is replaced by the ssh process via a call to `execvp`.

### Adding/removing a bookmark
```
>>> ssb -a rpi pi@raspberrypi  # Adds a bookmark named 'rpi' that expands to pi@raspberrypi
>>> ssb -rm rpi  # Deletes the bookmark named 'rpi'
```

### Listing bookmarks
```
>>> ssb -l
rpi -> pi@raspberrypi
dev -> jsmith@devserver
foo -> bar@baz
# etc.
```

## Bookmark file location
Bookmarks are stored in `$XDG_DATA_HOME/ssb/bookmarks.json,` or, if that
environment variable is unset, `$HOME/.local/share/ssb/bookmarks.json`.

