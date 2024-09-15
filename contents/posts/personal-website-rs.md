# Personal Website in Rust

September 2023

Rust, Web

---

![](/assets/images/newsite.png)

This website was built using rust.
It is a static website that uses rust to generate the HTML supported by minimal CSS and JavaScript.
I also implemented a caching system as its own library [Jobber](#jobber) to cache the building work and correctly invalidate the cache when the files change.

The website builder has many features:
- Advanced caching site builder
- Hot reload server mode
- HTML templating
- Posts written in markdown
- Optimizes, resizes and converts images to webp (with fallback if webp not supported)
- Minified HTML, CSS and JavaScript
- Minimal JavaScript (used for theme switching and hot reloading)
- Minimal CSS

The website is split into pages that are templated HTML using a templating library called [tera](https://keats.github.io/tera/) and posts that are written in Markdown and converted to HTML using a library called [markdown](https://github.com/wooorm/markdown-rs).
The rust program has two modes: a build and a server mode.
## Build mode
The build mode builds the website into static HTML, JavaScript and CSS ready to be hosted on any standard static web host.
This is what is used by the continuous integration GitHub actions to build the site and upload it to a google cloud storage bucket.

## Server mode
The server mode is similar when run builds the website and hosts it on a local web server.
It also watches the source directories for any file changes.
When a file change is detected it rebuilds the website and reloads any connected web pages.

This server mode is designed for fast iteration so that changes can be made and previewed very quickly.
A common workflow is to have the website preview open in a browser alongside the code.

When in server mode information is logged to the terminal about how long a generation took to build and how many jobs were run as result of cache misses.
```
 🚀 Built e1ab23e6b2c1cac3
 Generation = 217
 Jobs = 8 / 70 = 88.6%
 ⏱️  43.4ms
```

# Hot Reloading
Hot reloading is very important for the server mode.
It must build the website quickly and reload any web browsers viewing the page.

## File System notifications
Firstly the server must watch the file system for file changes in the source directories.
The library [notify-debouncer-mini](https://crates.io/crates/notify-debouncer-mini) is perfect for this purpose.
It is a wrapper around the library [notify](https://crates.io/crates/notify) that only notifies once over a time period.
This is useful because upon saving a file to the file system, many file events may be emitted.
To avoid doing extra work the server should only reload once over a time period.
[Notify-debouncer-mini](https://crates.io/crates/notify-debouncer-mini) upon receiving a notify event will start a timer and ignore other file system notifications until that timer expires when it emits a single notify event.

## Rebuilding
Once the server receives a file system notification that a source file has changes, the server needs to rebuild the website.
This is straight forward but in order to improve performance we use a cache to cache work that doesn't need to be re computed.
See [Jobber](#jobber) about how the caching works.
Once the website has been rebuilt the files are updating on disk and the browser needs to be triggered to reload the page.

## Reloading
There are a number of ways to trigger the browser to reload the page.
Fundamentally this involves sending an event from the server to the browser.

- The browser could poll regularly for any changes but this is likely inefficient.
Older browsers won't support the other options so this might be better if you care about old browsers.

- WebSockets are a good way to have two way communication between a server and browser.
They allow the client side JavaScript to connect to the server using HTTP and then upgrade the connection to Web Socket.
The client and server can then send messages at any time.

- Server sent events (SSE) are similar to websockets except only the server can send messages.
In this case we only want the server to send events since we just want the client to be able to react to reload triggers.
Just like websockets SSE are connected to using JavaScript on the web page and an HTTP connection is upgraded.

I chose to use server sent events because they are more efficient than polling and I don't need the extra features of web sockets.

The way it works is every time the website is rebuilt a generation number is incremented.
A small piece of JavaScript is injected into the website with the current generation number it was built with.
On load the JavaScript connects to the server sent events stream.
When it receives an event, the event contains the new generation number.
If the new generation number does not match the generation number injected into the JavaScript it will call `window.reload()`.
The generation number is not strictly necessary but is useful in avoiding potential issues with infinite reloading.

# Jobber
Jobber is library I wrote to cache the site building work.
It is located in the same repository as the website [Jobber](https://github.com/RuairidhWilliamson/PersonalWebsite/tree/main/jobber).

The goal of jobber is to memoize/cache function calls while keeping track of their dependencies.
Their dependencies include their function arguments and files they read from disk.

The cache evicts based on least recently used with fixed capacity.
The `JobId` is the key to the cache which is composed of the job name and the hash of its arguments.
The `JobCtx` allows functions to declare dependencies on files.

## Usage
A simple jobber function looks like this:
```rust
ctx.job(JobId::new("my_job", 0), |_ctx: &mut JobCtx<'_>| {
    println!("my_job");
    Ok(())
})
```
This function doesn't have any arguments so its argument hash is purposefully set to zero.
Jobber will run this function the first time it is called and cache the result under the name given "my_job". 
Any subsequent calls to a job id of "my_job" will not run the function and just return the cached value.

The ctx being passed through is used to declare dependencies. For example
```rust
ctx.job(JobId::new("my_job", 0), |ctx: &mut JobCtx<'_>| {
    println!("my_job");
    ctx.depends_file("test1.txt")?;
    Ok(())
})
```
This job still takes no arguments but depends on a file named `test1.txt`.
Jobber will take the hash of the contents of that file so that if the file changes the cache will fault for this job the next time it is run.

Writing all this is quite clunky so jobber also has procedural macros to automate this.
```rust
#[jobber_derive::job]
fn my_job(ctx: &mut JobCtx<'_>, x: usize) -> Result<()> {
    println!("Run my_job with x = {x}");
    Ok(())
}
```
This is much cleaner and more understandable.
The generated code computes the hash of the arguments for the job id and a call to `ctx.job()`.
It requires that all arguments implement `Hash` of course.

Jobber keeps track of the tree of jobs so to start the jobs requires starting the root job.
```rust
let cache = Cache::new(NonZeroUsize::new(16).unwrap());
cache.root_job(JobIdBuilder::new("top").build(), |ctx| my_job(ctx, 5)).unwrap();
```

Jobber even supports progress reporting that can be used to display progress bars.
It allows an implementer of a progress report trait to be passed in that will be called with information about cache hits and misses.

## Potential Improvement
Jobber is a nice experiment in this project and speeds up the iteration time by caching most work that does not need to be done every reload.
Jobber is not perfect, it cannot "know" about all data dependencies.
For instance although rust makes it hard to use global variables it is still possible using statics and unsafe.
This is out of scope for jobber though as a programmer could still forget to declare a dependency on a file that a function does in fact depend on.
To correctly track all dependencies without the possibility of error would require modifying the programming language.

Jobber could be improved in the following areas:

- Jobber could also have dependencies on resources not on the local file system.
It could be useful for jobber to be able to depend on an HTTP resource.
There are HTTP headers that can be used to determine cache behavior without downloading the entire resource.

- Jobber could persist its cache between program invocations.
There would be value in writing the cache to disk and reusing after the program has exited.
Currently jobber just uses an in memory cache which is not persisted.
There is also a potential issue that would need to be addressed with the hashing algorithm not being stable between computers or rust versions.
Maybe a trade off could be made with a slower hashing algorithm like sha256 so that the cache could be shared across computers.

- Jobber could use the modified time of files to check if they have changes without hashing the entire file.
This is more complicated than it first seems as file modified times are not very reliable and vary based on the file system.
Different file systems and OSes have different resolutions of modified time too.

- Jobber could use the file system events from notify to mark files as dirty.
Since we are monitoring the file system for changes to the files we care about.
It is wasteful to open all the files and check if their contents have changed since we already know exactly which files have changed.

[View on GitHub](https://github.com/RuairidhWilliamson/PersonalWebsite)
