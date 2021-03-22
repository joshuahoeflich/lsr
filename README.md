# LSR
Tiny program that recursively prints out all the files in the directories you give it. Given a directory like

```
many_level
├── 1
├── 2
└── deep
    ├── 3
    ├── 4
    └── deep
        ├── 5
        ├── 6
        ├── 7
        └── deep
            ├── 10
            └── 9
```

Running `lsr many_level` will print the absolute paths of `many_level/1`, `many_level/2`, `many_level/deep/3`, and so forth. 

`lsr` will try to interpret whatever you give it as a directory or a file; if it doesn't exist, it'll print nothing without erroring.

# Installing
On Arch Linux, there's an AUR package that you can install called `lsr-git` with your favorite AUR helper. For example, if you use Paru:

```
paru -S lsr-git
```

Should do the trick.

To build from source, simply run `cargo build release` and copy the binary somewhere convenient. 
