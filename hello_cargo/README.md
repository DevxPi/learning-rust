# Building and Running a Cargo Project

```bash
cargo build
```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:

```bash
# linux
./target/debug/hello_cargo

# output
Hello, world!
```

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use `cargo run` to compile the code and then run the resultant executable all in one command:

```bash
cargo run
```

Using `cargo run` is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use `cargo run`.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```bash
cargo check
```

### Let’s recap what we’ve learned so far about Cargo:

- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## Building for Release

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in target/release.