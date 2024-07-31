# trash

a simple bash clone

https://github.com/carrotfarmer/trash/blob/fd3d41dfb2e6ff2823a36d7b3847f041a44d59fb/demo.mp4

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
