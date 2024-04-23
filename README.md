# Polygon.io crawler
This is a simple crawler, which is designed to be hugely scalable.
By scalable we mean this modularization practice allows us having a tree of modules,
owned by different teams, who can work independently by just sharing some conventions/definitions.
The definitions are inside modules named `io`, input & output, which define how to interact with
the implementations, available in the `impl` modules.
Someone can argue this is the application of Dependency Inversion Principle, DIP,
where we separate the interfaces from the implementations, and only depending on interfaces.
However, applying DIP more than this modularization practice is not recommended,
therefore, inside the `impl` modules the developers are free to apply whatever practice works
for them, as long as the `impl` module is small enough.
We argue the other developers will not have problem to dig-in, read the code, or even debug it,
if the `impl` is small enough, and we had this modularization practice in place.
This means the developers have the freedom to practice all kind of architectural designs,
applying all kinds of benchmarks and flags on them,
as long as they keep it short enough in the `impl` modules,
which is the secret of being scalable.
You get the idea, if someone in the team cannot understand the `impl`,
the developers need to break it down into smaller `impl` modules, along with their `io` of course.
I can even read and understand a `impl` module that used single character variables,
if the `impl` module small enough.
This freedom would be the source of all kinds of innovation.
In my humble opinion, it's cleaner than the Clean Architecture,
to take the word 'clean' back to whatever architecture that works for you!
Indeed, you may noticed it will give developers the freedom of microservices.

Another advantage of this modularization practice is that
it'll keep the incremental build time short to make the iteration on features fast.
The assumption is that developers mostly need to change their code in the `impl` modules to iterate,
hence, their `io` modules are much more stable.
By forcing other modules to invert their dependency from `impl` to `io`,
we expect other modules don't need to rebuild while a developer only changes the `impl` modules.
This implies the incremental build time is kept low while a developer iterates on a single `impl` module,
even though the project could have a thousand modules.
This advantage also contributes to the scalability of this modularization practice.

By the way, I didn't invent this practice, I am just an advocate.
If you have noticed, the C programming languages had the concept of header files for a long time,
where it's its secret to scale up to OS level, and let a lot of developers contribute independently.
The only difference here is that we force DIP on the module level,
but we totally ignore it inside the `impl` modules.

Also in the giant monorepo Android code bases, this modularization practice is common,
because in monorepos, you have to build the whole source the same as what we have in Rust,
so the problems are similar, thus, the solutions are similar.



## Usage
Create a file named `.config.nix`, before running `shell.nix`, with the following contents.

```nix
{
  dbUser = "you";
  dbPassword = "very complicate pass!";
  apiKey = "highly cryptic api key!";
  dbName = "postgres";
  dbData = "/home/$DB_USER/data";
  dbTestName = "postgres_test";
  dbTestData = "/tmp/data";
  dbPort = 5432;
  policyCoolDownTime = 350; # seconds
  policyAllowedRequests = 4;
}
```

Then you can run `shell.nix`. After that launch the [database](container/README.md).
Now it's ready to run `cargo test`, and of course, `cargo run`.

Enjoy!

