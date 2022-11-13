Console based text editor. Work in progress. Name in progress.

# Design Principles

## Scriptable

Data in JSON or TOML is just too limiting. This should be fully configurable using some scheme dialect (more on this later).

## Blank slate

Provide the user with a list of editor commands, and let them configure the key bindings they want. Modal, CUA, GNU Emacs - it shouldn't matter.

## Tight LSP integration

Users should not need plugins for LSP - it should be a core part of the editor.

# What core data structure?

I am going to go with piece tables, for now.

I like them because it looks you get undo functionality thrown in, and they make more sense to me than ropes.

Very open to changing my mind.

# Why not other editors?

## Sublime Text

My favourite, but does not run on the console.

## Micro

Best console editor, but it's customisation will always be limited due to using console files

## Emacs

Great idea - a running LISP interpreter powering an editor. But I just can't wade through the decades of cruft. Also, the terminal is a 2nd class citizen

## Vim

I can use modal key bindings but they're not my favourite. Extending it with lots of plugins to get LSP working is very fragile IME.

# Which scheme to use?

## Scheme <-> Rust Interop

I want the communication to be as simple as possible: ideally the rust has no idea about scheme. Scheme is the master, rust is the slave.

## Features

Stuff I would struggle to program scheme without:

- multiple dispatch
- pattern matching
- hashtables
- records

I also want the ability to reload modules at run time, so the user doesn't have to stop the whole system.

## Implementations/Dialects I'm considering

### Chibi Scheme

Pros:
	- The simplicity of statically embedding appeals to me
	- Has all the features I want

Cons:
	- Seems to be based around C calling Scheme, not Scheme calling C.
	- Smaller implementation - a niche within a niche

### Racket

Pros:
	- Best looking C FFI I've seen 
	- Great docs
	- Biggest ecosystem

Cons: 
	- Not sure if I can recompile modules on the fly

### Guile

Pros:
	- Great manual
	- Almost 30 years old
	- Keeps improving (now has JIT)
	- Widely used as far Schemes go

Cons:
	- From https://github.com/ysimonson/guile-sys: "You probably don't want to use this or any guile bindings for rust. Guile liberally uses setjmp/longjmp, which breaks rust destructors."
	- My distro uses the old 2 version

## Implementations/Dialects I've ruled out

### Chicken

Not really designed to load scripts at runtime.