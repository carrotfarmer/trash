# trash

a simple bash clone

## building

If you want to try this out, make sure you have cargo installed. Then, run the following commands:

`trash.sh` will basically execute `cargo run` for you

```sh
git clone https://github.com/carrotfarmer/trash.git
cd trash
./trash.sh
```

## features

as of now, trash supports:
- basic shell builtins (`echo`, `exit`, `type`, `pwd`, `cd`)
- run executables in `PATH`
- run shell scripts (mostly works)
- output redirection to files (`>`)
- piping (`|`, mostly works)
