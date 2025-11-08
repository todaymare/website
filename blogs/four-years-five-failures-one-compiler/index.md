# Four years, Five failures, One compiler
At 14, I thought writing a compiler would be a quick side quest in building a game engine. Four years later, I finally built one that works. And it was one hell of a journey.

Before I dive into the compilers themselves, I should explain why I even started making them in the first place. Initially, writing a programming language wasn't the goal; it was just supposed to be a small part of a bigger piece: building my own game engine. I didn't expect that "small part" to turn into a four-year rabbit hole, but now that you know why it all started, let's look at a few of the failed projects, shall we?

## CryScript
Super edgy name, I know. To be fair, the game engine this was meant for was called *Crytex*, so that doesn't make it any better. It was back in early 2022, so technically, the title of this post is clickbait. Oh well.

While writing this, I dug through [the old GitHub repo](https://github.com/todaymare/CryScript/), which had a very odd structure. Something like this:
```rust
crates/
 cry_script/
 src/
        ..
    Cargo.toml
src/
 main.rs
Cargo.toml
```
I have no idea what I was on when I set it up like that, but for a moment, it actually scared me. I thought I'd lost the source. Luckily, the code's all still there, so let's take a look and have some fun.

I'm pretty sure I started off by following a [Python compiler tutorial](https://www.youtube.com/watch?v=Eythq9848Fg&list=PLZQftyCk7_SdoVexSmwy_tBgs7P0b97yD)... in Rust. 

And surprisingly, that worked well for the first half. I got the lexer and parser running (basically the parts that read the code), and they still hold up well even now. 

However, things started falling apart once I reached the interpreter, the bit that's supposed to run the code.  

But up until that point? Honestly, not bad. Even skimming through it today, I can see the same patterns I still use in my work.  

..Oh my god, I take all of that back.  
The interpreter is an abomination of Rust code, and it's also where you can clearly see the effects of Python. It's a tree-walk interpreter, which basically means the Rust compiler absolutely hates it.  

I vividly remember fighting the borrow checker & sprinkling in `Rc<RefCell<_>>'s everywhere whilst writing this, and it seems like eventually, I just gave up and resorted to doing:
```rust
struct VariableReference {
 reference: *mut Variable,
}
```
Which is, apparently, a non-clone, manual version of a reference-counted value?  
Not a clue who wrote that code; definitely wasn't me!  
For non-Rust users: this is me fighting the language's safety system and losing badly. 

I have no idea where this was needed, but it might be one of the worst pieces of Rust code I've ever written, second only to the *Crytex* engine.

That one doesn't even compile anymore, by the way, because newer Rust versions prevent casting an immutable reference to a mutable one. (Which, fun fact, is undefined behaviour)

Oh, and apparently, I was passing around the main context of the interpreter as a mutable pointer. Those of you who use languages like C or C++ might think "so what? that's normal", but in Rust, using pointers mixed with references comes with quite a bit more headache.

In hindsight, *CryScript* was less of a compiler and more of a Rust safety demonstration. The fact that it even ran code at all is a miracle. If I pulled this at a Rust conference, I'd probably get banned. But it was a Rust newbie's first attempt at making a compiler, and it wasn't a half-bad one.

## azurite
Fast forward a year, and I started making *azurite* (the 'a' is non-capitalised intentionally). This one's very near and dear to my heart as it's the project that let me meet some amazing people I still talk to today. 

Unlike *CryScript*, this one came a bit later, when I actually knew how to use GitHub. It has 76 commits, many of which have completely nonsensical names that could probably get me rejected from a few jobs.

I should, however, mention that between starting *azurite* and finishing *CryScript*, I joined the r/ProgrammingLanguage's Discord server, which led to me reading the book [Crafting Interpreters](https://craftinginterpreters.com/). I think everyone interested in compilers or interpreters should go through it at least once. 

Unfortunately, I don't actually know when *azurite* started, since the GitHub repo seems to have been created after the language was already up and running. There are a bunch of example programs and documentation in there, so it must've been pretty far along.

There are binaries uploaded from the early days, but since I apparently thought my code was too special to share, I didn't upload the source, you know, in case someone stole my brainchild. After a while, though, I came to my senses, uploaded the code, and blessed the world with my amazing engineering.  

A significant difference between *azurite* and *CryScript* is that *azurite* actually had static analysis, whereas *CryScript* was dynamically-typed. That said, the lexer and the parser are almost identical.  

I'd love to make fun of the static analysis, but honestly, many of the core ideas I used back then are still part of how I think about language design today. It's surprising to see that, at least on a high level, I already had a so-so grasp of the basics. 

But I did find this comment, which I think perfectly captures my ambition at the time.
```
// TODO: Maybe make the multi-file-loading multi-threaded
```

The bytecode interpreter for *azurite* had a major performance flaw that someone else figured out. If I remember correctly, it was something like this:
```rust
let mut callstack = ..
let mut code = &mut callstack[0];
```
vs 
```rust
let mut code = callstack[0].clone();
```
That one change made the interpreter run **10x faster**. Wild. But more importantly, that flaw introduced me to [leddoo](https://www.youtube.com/@leddoo), who's now a close friend. So I guess being bad at programming has its perks.

I also noticed other people starring and contributing to this project, which might make you wonder why I stopped working on it. The reason is quite simple really, the codebase collapsed under its own weight.  

See, that's the problem with making a long-term project in a field you barely understand. Every time you add something that wasn't initially expected, in my case that was generics, the codebase just gets worse and worse and eventually it got too much to handle.

It's bittersweet reading the old commit log. People added features like a REPL — that one was my now-friend [Pyrotek45](https://github.com/pyrotek45/) — and then months later, I removed it. To anyone who ever contributed to *azurite* and happens to read this: thank you. I really mean it.  


## margarine
And now the final gauntlet. *margarine*.  
I started near the end of 2023. At first, it was supposed to be for another game engine, this one called *butter*. The plan was to make a language centred around ECS architecture and value types. I even wrote a lexer, parser, semantic analysis, and an LLVM-based codegen. 

But when it came to integrating it, I caved and just used Lua. Yeah. Because here's the truth: I'd spent four years learning how to make a compiler... but not how to make a game engine.   

That actually felt horrible. After all that work, I shelved margarine. I moved on to voxel engines, games, raytracers, fluid simulations and much more..  

You read the title, though, you know the story didn't end there.  

A couple of weeks ago, I came back to *margarine*, for what reason I don't know. This time, I stripped away the over-engineering (no more ECS gimmick). I ditched LLVM (way too painful) and built a clean bytecode interpreter instead.  

And now? I finally have what I dreamed of: an embeddable programming language I can use in any project.

So here it is, after years of broken interpreters, pointer hacks, and abandoned repos, I am delighted to introduce to you, *margarine*: 
```rust
fn main() {
 var numbers = [1, 2, 3, 4, 5]
 var multiplier = int::parse(std::read()!)!

 var doubled = numbers.map(|n| n * multiplier)
    print(doubled)
    // If input is 2 → [2, 4, 6, 8, 10]
}
```

A language that feels like Rust, but without the fights I used to lose with the borrow checker. It's a statically-typed language that is almost too trivial to embed into any project. The syntax is very akin to Rust with Iterators, Closures, and much more!  

Was this post just a big ad for *margarine*? It certainly wasn't the intention but after four years it feels good to finally share something that works, and it couldn't hurt to [check it out](https://github.com/todaymare/margarine) can it?

and hey, if it resonated with you [consider dropping me a coffee](https://ko-fi.com/todaymare)