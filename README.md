# ssb
ssb 1.0.0  
Keating Reid <keating.reid@pm.me>  
Bookmarks for your ssh connections

# Description
`ssb` is a small tool to make managing ssh connections easier. Instead of
having to type `ssh user@hostname` each time, user/hostname combinations can
be saved and referred to by a short-form name.

# Synopsis
| **ssb** -a KEY ADDR [ssh-args] 
| **ssb** -rm KEY 
| **ssb** [-l|-h|-v|--version]

# Usage
## Adding/removing a bookmark
Adding a bookmark named `rpi` that corresponds to the address `pi@raspberrypi`:
```console
# ssb -a rpi pi@raspberrypi
```
You can also specify arguments to pass to the ssh command whenever connecting to a bookmark:
```console
# ssb -a rpi pi@raspberrypi -- "-i ~/.ssh/id_rsa" 
```
Deleting the bookmark named `rpi`:
```console
# ssb -rm rpi
```

## Starting a connection
```console
# ssb rpi
```
If invoked without any flags, `ssb` will interpret the first argument as the
name of a bookmark and attempt to initiate an ssh connection. The `ssb` process
is replaced by the ssh process via a call to `execvp`.

## Listing bookmarks
```console
# ssb -l
pi -> (addr: pi@pihole, args: ["-i", "~/.ssh/id_rsa"])
foo -> (addr: bar@baz)
dev -> (addr: jsmith@devserver)
```


# Bookmark file location
Bookmarks are stored in `$XDG_DATA_HOME/ssb/bookmarks.json,` or, if that
environment variable is unset, `$HOME/.local/share/ssb/bookmarks.json`.


# Compatibility
As mentioned above, `ssb` relies on the `execvp` function provided for by the
POSIX standard. Naturally, it won't work on Windows, but should on just about
anything else.
