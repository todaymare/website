# No, LLVM can't fix your code
Y'all mind if I rant a bit first, promise I'll talk about a few optimizations that got my RISC-V emulator to run at 550 million instructions per second purely interpreted.  

This is sort of a follow-up to [Speedrunning a CPU](../speedrunning-a-cpu/). But before I talk about what I did to make it run that fast

Let me set the scene, alright.  

2023, I'm deep in the compiler-development scene ([see: Four Years, Five Failures, One Compiler](../four-years-five-failures-one-compiler/)) and this is about the time where I discovered arenas and StringMaps.  

An Arena, at its core, is a big vector of bytes. You can put any data you want in there. If I wanted to store a `Foo`, I'd just write it into the buffer and bump a pointer by `sizeof(Foo)`. Basically a very fast way to allocate data (like `malloc`), but without the ability to free anything individually afterwards.

This is useful in compilers since you have a lot of data to allocate and more often than not the drawback doesn't matter since you won't be releasing memory in individual bits but in phases (think lexing phase, parsing phase, semantic analysis phase, etc.). 

Then there's the StringMap, which is a HashMap + a vector of strings. You have a HashMap of String to an index into a vector, and then a bunch of Strings.  

With this you can convert all strings in the program (identifiers, actual strings, etc.) to an integer index which not only has a smaller payload but also comes with constant time comparison. (think it's actually called string interning?)  

These kinds of data-structure decisions are what I'd call macro optimizations. The ones where you really can't expect LLVM to help you with, but of course that doesn't stop people from believing LLVM will.  

Anyways back to the rant, I was having a conversation with someone who was making their own programming language as well. He was trying to improve his compilers performance but he was tunnel-visioned on micro-optimizations.  

Look I love to give my opinion on things without being asked as much as the next guy, but this time I was asked. And considering I'd just spent weeks squeezing out huge gains from StringMaps alone, I naturally suggested he give it a try. That's when he hit me with:

> LLVM can already handle those with `mem2reg` and stuff, so why would I complicate my code?

I can't make a deep breath sound in a blog post but just imagine one here. The fact that I still remember this quote probably says a lot about how deeply it shook me. Not to mention that this was coming from someone making compilers, someone who presumably knows more about what a compiler backend can do than the average dev.  

Just to be clear: no, LLVM can't and won't magically turn the thousands of generic containers and redundant string copies into a cache-friendly arena-backed interning engine.  

But, while that is a conversation that stuck with me, I think (I hope) that most of us know that LLVM can't do these macro-optimizations that drastically change the data layout of the program.  

There is, however, a category of optimizations I'd expect LLVM to be able to do

## Micro-optimizations
Let's jump to the end of last week, optimizing the RISC-V emulator portion of the story.  

Micro-optimizations include the aforementioned `mem2reg`, loop unrolling, constant folding (converting stuff like `3 * 4` to `12` at compile time), etc. the stuff that is very localized and would be tedious to do by hand. Not much reason to talk about the stuff that LLVM already handles so let's zoom in on one micro-optimization that needed my help. 

But before that I'd like to give an honourable mention to likely/unlikely path hints.  

These fellas gave me about 100MIPS~ of performance in the emulator and solve a particular kind of information problem: LLVM doesn't know what your data is, so it needs to assume all branches are equally likely to be taken if it can't infer otherwise.  

This is a problem because it messes with the branch predictor and pollutes the instruction cache. But if you know something about your data, like that `time_elapsed > 5s` almost never happens, you can tell LLVM to put that in the 'cold' section so the CPU doesn't even think about it. Which makes the hot path possibly way faster, and also makes the cold path way slower but that's a tradeoff we can choose to make.

Now let's actually talk about the optimization that gave me more than 75MIPS.  

No, it's not removing bounds checks. I think I still have those in the emulator, those really matter less than you'd think (hey cold paths!).  

It's actually just deleting 5 letters.
```rs
self.cycle += 1;
```
into
```rs
cycle += 1;
```

uh, so what did that change do exactly? Well I'm glad you asked, me from 10 seconds ago! It moved the `cycle` from stack memory into a CPU register.  

Okay well, technically the code change was a bit more than that because LLVM can move things in memory into a register (hey `mem2reg` is back!).

That is as long as the variable isn't passed mutably into a non-inlined function. Otherwise LLVM would have to assume it could be modified¹ and write it into memory before calling the function and at that point oftentimes LLVM just prefers not to put it into a register.

In this context, the cycle counter is incremented every single instruction and isn't really passed into a lot of functions so I was really expecting this change to do basically nothing. Especially considering I had to put something like:
```rs
if unlikely(cycle % 128 == 0) {
    self.cycle = cycle;
}
```
in the main loop.


## The end

Well, that's it. The moral of the story is that LLVM isn't sentient yet. Maybe when we solve the halting problem.  

But until then, hope to see you [in my discord server](https://discord.gg/t7gNX8Kp72).

If you enjoyed this, [considering dropping me a coffee](https://ko-fi.com/todaymare).

Either way, I'll be here as usual next Sunday.

¹: Yes, LLVM does fancy things like function specialization, inline heuristics, and other stuff. This is still a gross oversimplification, but I have no interest in doing a deep dive into all of that nor am I in any way qualified to.
