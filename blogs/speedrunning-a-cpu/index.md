# Speedrunning a CPU: RISC-V in a Week
I made a RISC-V Emulator that runs at 550 million instructions per second from scratch in one week. Here's how it went.

So, here's the thing. After the [Voxel Engine in a Weekend](../voxel-engine-in-a-weekend/) project, I was looking for a new challenge. That's when I thought, huh, why not make an OS? And a compiler running in that OS, and also a browser, a neovim clone inside that OS, and while we're at it GTA 6 as well.

But of course I couldn't just learn to make an OS, nah that'd be too easy. And I quote:
> Why stop at an OS, why not make your own CPU that runs your own OS with your own language that compiles to it.  
> *08/11/2025*

I had to start from the CPU itself, otherwise how could I be proud of it. And I also had to do it fast, considering for this entire project (the OS, browser, compiler, neovim clone, GTA 6) I allocated myself a generous singular year. It was set, I was going to speedrun making a CPU, and I was going to use RISC-V. 

## Step 1: Say you'll make an assembler too!
Okay I hear you, I just said "gotta go fast" and now am considering making an assembler for seemingly no reason. In my defence, I had not a single clue where to start, but I have made [plenty of programming languages](../four-years-five-failures-one-compiler/) so I thought: okay, if we're going to do this I might as well start with an assembler, since it'll be a nice easy step to smooth out the curve and get me used to the binary format of RISC-V. 

In theory, an assembler is as simple as it gets. You have really simple syntax, maybe a few shapes of instructions, basically no semantic analysis, and a trivial 1:1 code generation phase. In practice, however, GOOD assemblers are filled with macros, pseudoinstructions, constant folding, and probably so much more that I can't think of off the top of my head.

Are any of these really hard? Not really. But all of them require a lot more knowledge in the target instruction set than I had at the start of this week. Pseudoinstructions aren't officially documented... what the hell is a macro... constant folding who? You can probably guess that my hope of writing an assembler vanished VERY quickly, and all of that before I even got to the codegen part of the assembler. You know, the part that's actually useful to me making an emulator?

I could've made a toy assembler, but would I have used it? Absolutely not. This thing is hard enough already I don't need one more thing to debug. So I made the very difficult and heart breaking call to not make an assembler. 

## Step 2: Oh it's only 200 pages-
Now, time for the emulator. I wanted to, I needed to keep this emulator as simple as possible so I could complete it in this week. So I decided, no virtual memory, no fancy extensions, no jit, nothing fancy. And then it was Sunday morning.

I opened up my computer with the hopes to get the emulator started. See, the first thing I do with any interpreter I make is to run the recursive-Fibonacci algorithm. It's a nice little stress test that is very easy to run and gets the dopamine flowing you know? So I went to the RISC-V site to check out the instruction set. 

There were 2 specs. [Volume 1](https://docs.riscv.org/reference/isa/_attachments/riscv-unprivileged.pdf) for unprivileged architecture, and [Volume 2](https://docs.riscv.org/reference/isa/_attachments/riscv-privileged.pdf) for privileged. Since my goal was to make an OS and not to compile to a RISC-V user platform I thought: okay, I just need Volume 2. I opened it up and it had about 221 pages. Okay that's a lot of pages. Hell, that's 6 pages of just table of contents.

But it's managable, I could skim through it in a day or so and then reference it back or something. So I got looking. Huh. I can't see where the instructions are.  

Here's the thing, dear reader. You might've noticed that the books are titled Volume 1 & 2, and while I didn't think they actually meant it in that way they absolutely did. The instructions were on Volume 1 as well.  

Would you like to guess how long Volume 1 is?  
About as long as Volume 2? 
Yeah that's about what I'd expect too, 

NOT 727 PAGES.

I ain't reading all that. If you're curious that's 19 pages of table of contents.

Granted the spec is very well structured and looking back all I would've had to check out was maybe about 20-30 pages filled with diagrams. But '727 pages' was enough to make me close the tab and look for other more concise sheets. And that's when I found it. My saviour, [Writing a RISC-V Emulator in Rust](https://book.rvemu.app/index.html). When I saw this I went
> Omg, I don't need to learn anything let's go!

..that is, until I clicked on the link. Here, I'll show you the table of contents of it at the time of writing:
```
Writing a RISC-V Emulator in Rust
   1. Hardware Components
      1.1. CPU with Two Instructions
      1.2. Memory and System Bus
      1.3. Control and Status Registers
   2. Instruction Set
      2.1. RV64I Base Integer Instruction Set
```

And, that's it. I'm not trying to bash on the creator of it I'm sure it's either an unfinished or even an abandoned passion project, but when I saw this the timing was so unfortunate it became funny. For those curious, that's about 0.1 pages of table of contents.

BUT THEN IT APPEARED! 

My actual saviour,  
the [RISC-V Instruction Set Specifications](https://msyksphinz-self.github.io/riscv-isadoc/) by some random github person. 

It was pretty, it was useful, it sometimes had typoes and mistakes, but most importantly, it had whitespace so it didn't look scary.

## Step 3: ..make the thing already?
It was still Sunday. I got some very basic RISC-V code working.
```
    .section .text
    .globl _start
_start:
    li   a0, 42
    li   a1, 99
    add  a2, a0, a1
1:  j   1b                # infinite loop
```
can you guess what it does? Probably not, it's too advanced.  

But now that I had this very simple snippet working the momentum curve was slowly tilting in my favour.  

By the end of Sunday I had 13 instructions ready. And fast forward a few days I had the entirety of RISCV64I implemented. Which of course surfaced a new kind of problem. The hardest, least motivating part of writing an emulator like this is that you really have no idea if your implementation is correct. Sure yeah your programs might be working but you don't know the edge-cases, and when a program doesn't work like you expected good luck trying to debug it.  

When I brought this point up to one of my friends he linked me the [RISC-V Test Suite](https://github.com/riscv-software-src/riscv-tests), which I promptly ignored in favour of compiling some Rust code and running it on my emulator.

..damn, all that set up for nothing.  

Okay, to compile Rust to RISC-V, Rust provides a bunch of pre-made targets. But you see, RISC-V at it's core is a very simple instruction set with 47 instructions and it provides optional extensions like multiplcation, floating-point numbers, atomics, etc. And not implementing these optional extensions is really not an option with Rust's premade targets.

So I had to make my own custom Rust target. Which sounds interesting but was mostly done by my friend and when I asked him how he did it his response was "I asked ChatGPT and copy pasted the errors until it worked". Which, fair enough.

Of course, the first thing I compiled with Rust was

```rs
fn fib(n: usize) -> usize {
    if n <= 1 { n }
    else { fib(n-1) + fib(n-2) }
}
```

Which worked first try of course, on the 4th try.

And now that Fibonacci was working I had to go for the second milestone of interpreter development: Flappy Bird.


## Step 4: So how do we see?
Well, first things first, we need a window.

Since this is my emulator, I get to make the rules. And I decided that a specific memory range would be dedicated to the framebuffer. One of the few places where I actually went with the simplest option yes.
So at address `0x1000_0000`, I placed a 1920x1080 RGBA framebuffer, so I could see the 16x16 pixel art of Flappy Bird in full HD.

Needless to say I got humbled FAST. 

See at this point, my emulator was running at a solid 100 million instructions per second (MIPS!, which sounds impressive until you do the math. 

A 1920x1080 framebuffer has over 2 million pixels.
Even if we only spent 5 instructions per pixel (which is practically impossible), that's already 10 million instructions per frame, or 10FPS.

That's with math that's about as accurate as rounding PI to 10.

I know that I'm "speedrunning a CPU" but like.. it has to be at least usable you know?  

So in a moment of weakness, in the middle of a uni lecture, I lowered the resolution to 640x360. Which brought the FPS from a theoretical 10FPS to about 40 real FPS.  
For drawing a white background.  

But 40 FPS ain't real time. That's AI-interpolated horrible animation framerate, not a usable GUI app framerate. So I got to optimizing, and the one thing that really improved the performance was the Instruction Cache.

### The Instruction Cache

Before I explain that, let's look at the problem it solves first:

When decoding a RISC-V instruction, you don't really have a single number that tells you what the instruction does. Instead, you have these instruction shapes (like register-register, register-immediate, jump instructions) and only after you figure out the instruction shape you can decode the instruction itself.  

For hardware, this is extremely fast. Since the main decoder only needs to know the shape of the instruction and then can send it to something that knows how to decode it.  
For software, this is absolutely horrible.  

For software, we first need to match on the shape of the instruction and then match on the actual instruction in order to execute it. But if you look like 5 lines above, you'll know that there's a solution to this. We can just cache the decoded instruction instead of decoding it on the fly every time. Let me introduce you to: The Instruction Cache

Basically, when were trying to decode an instruction we first hit up the instruction cache to see if it has already been decoded. If yes, then just execute that. If no, then decode it and put it in the instruction cache, then run it.  

But how are we going to decide what to decode? 

- We could have a 1:1 mapping of our entire memory to instructions  
our entire 64 bits of memory..?

- Can't we just decode the code given to the emulator?  
Well no. If we want to run actual OS code, it'll need to be able to load new programs at runtime

- What about a HashMap?  
A HashMap lookup every single instruction would probably reduce our 40FPS to 0.4

- So what did I do?  
Pages!

When loading a PC we can look at what page it's in and load that page to the instruction cache. And that works! But (not so) surprisingly, when implemented naively it's not much faster than just decoding on the fly.  

The reason for this is that in the naive solution, we're checking the PC every time it moves to the next instruction, which is horribly slow and unnecessary considering most of the time we're just moving to the next instruction.

But if we're not checking the PC every time, how do we know that it has changed? 

Here's the idea, let's say our pages are 1024 instructions (4KBs).  
What if we inserted a fake instruction at the end of the page that tells us that we're at the end of the page. Kind of like a nul-terminated string!  

Then we can go fetch the new page our PC is in.

And then for instructions that modify the PC directly (jumps, branches, etc.) we can just always fetch the page for the PC.  

Of course I did add early-outs to that page fetch for the common case where the jump is in the same page but that's the general idea of the Instruction Cache.

After tinkering around with the Instruction Cache a bit more, instruction decoding became essentially free. Which is absolutely amazing, but I was still at only 65FPS.

### The M extension

Next thing I implemented was support for more advanced operations. Such as multiplication (scary), and division!  

This brought the performance up from 65FPS to about 80. Mostly because before this we were using a software implementation of division and multiplication.

Which means hundreds of instructions that got replaced with a single instruction.

And unfortunately I was out of ideas for how to make this any faster, so I had to accept that my emulator was only at 160MIPS, which is still amazing for a pure interpreter!  
If I had that many MIPS back in the year 200 I would've been basically god.

This is about the time where I went back to the RISC-V test suite I mentioned a bit ago. 


## Step 5) Timeout
Originally, this section was going to be about how, on the very last day (the saturday right before this post went up) [leddoo](https://www.youtube.com/@leddoo), completely unprompted, found a way to bump the interpreter from 160 MIPS to 540 MIPS. Purely interpreted.

And I was gonna talk about it.  
I really was.  

But then I wrote the title "Timeout" and realized:
crap, I forgot to mention timers.

And listen-  
I would very much rather not talk about whatever that was.  
It apparently involved something called CSRs??  

So that's it. Just like that my one-week CPU speedrun was over.  
That's your ending. We're done here.


...


oh  
What's that?  
You're curious how we went from 160MIPS to 550?  


Well that's awkward.  
Honestly, I'd love to explain it. But it's 11:30PM, I haven't even made a thumbnail yet, and this post goes up at 9AM.
Maybe I'll write about it in another post if enough people ask. For now, [here's the repo](https://github.com/todaymare/riscv-emulator)

Here, I'll even make it easy to bother me:
- [My discord server](https://discord.gg/t7gNX8Kp72), if you want to bother me about the deets this is probably the best place.  
- [My E-Mail](mailto:contact@daymare.net), probably not the best place but I do enjoy getting mails a lot. Makes me feel important  

And hey, if you enjoyed this post consider [buying me a cup of coffee](https://ko-fi.com/todaymare).  
I kinda forgot this was a "weekly blog" and got way too deep into emulator dev.  
It happens.  

Alright. Goodnight.  
