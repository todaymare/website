# One of Those Bugs
You know how every project has one of *those* bugs that make you question whether or not it's even worth it to continue?  

Yeah, me too. From recent memory, not including today's, my [RISC-V Emulator](../speedrunning-a-cpu/) project got abandoned because I couldn't figure out a bug that was happening with virtual memory. But I was working on [margarine](https://github.com/todaymare/margarine) this week ([read more here](../four-years-five-failures-one-compiler/)) and I'm not going to abandon margarine again. It's the longest running toxic relationship in my life.  

I hear you, it's weird to write a whole post about a single bug. But it's my blog page and I've survived the gauntlet to Olympus and I'm going to tell the tale damn it.

Let me set the scene, I'll try not to yap too much.

It was the start of this week, Advent of Code had just started¹, so as any good compiler-dev does I went ahead and started solving it day-to-day in margarine. Day 1 went smooth, I had to add a bunch of features to margarine such as tuple destructuring, multiple standard library functions and a LOT of bug fixes that were relatively easy to do. And then, day 2 arrived.

Part 1 went relatively smoothly, too smoothly. I had the answer `40055209690`. 
![day2 part1 solution](./assets/day2-part1.png)

I moved onto part 2. I wrote it, compiled it to see if there were any compilation errors. See, the thing with margarine is that right now I don't have a separate build command, I just do `run` and it compiles it & runs. So when I compiled it to check for errors inevitably the part 1 code also ran.

There were no errors. But umm, why did it print out `47353585426`?  

And this is where my journey started. For some reason, adding new code broke existing code. I do mean *adding new code*, not *executing new code* because the part 2 solution wasn't even in the main path yet, it was just inside the file.  

Okay, let's see. The program is very closure & generic heavy. So I thought, alright, the problem is probably with the way I handle monomorphization which led me to go ahead and disable the function cache. Before, when I saw `foo::<int>()` I checked a HashMap to see if that function had already been generated. After disabling the function cache I was just creating a new function instead of reusing a newly generated one. That fixed it! But uhh, why?  

I went to investigate, because I can't just leave the function cache off! That'd generate way too big code files for my singular user!  

I have a custom hash function for symbols in margarine. Because margarine has a slightly more complicated type inference system than the simplest of type systems a symbol can be a type variable. We can't just hash a type variable directly, otherwise two generics that resolve to the same type would get different hashes, and the cache would explode.  

Which meant that the function cache HashMap is a HashMap of TypeHash and Function. Which means that it can no longer protect us from hash collisions. Aha! The problem is that we're getting a hash collision! That's a reasonable conclusion, 18 quintillion possible numbers and just my luck is a hash collision.

I added a simple check to the `get_func` function. Well, that's not the problem. Okay, how about closures? I mean the code is very closure heavy so if functions in general aren't the problem maybe closures are. Because when I disabled the function cache I also had to disable it for closures as well!  

Aha! (chatgpt ahh emotes bro) Closures are only hashed based on the closures type + expression id. Maybe it's not accounting for generics and captures, though if the expression id is unique that should account for captures as well. Regardless, doesn't hurt to try!  

I went ahead and added everything a closure could want to its hash but yet the problem was still there. Then I had the thought, maybe the closure isn't getting uniquely hashed because everything is still in generics! The issue is in how I've implemented generics! That's why turning off the function cache fixed the problem, it removed generics from the equation!. 

So the way I've implemented generics is quite possibly the dumbest way ever. I just register them as empty structs in the type table when type checking the function and when it comes to code generation I just create a name to type mapping. Maybe somewhere it's not mapping properly!  

Look, I would not do it this way if I were making it now but margarine has been going on and off for 2 years or so, it's basically legacy code for me.  

I went back and added a custom `Generic` variant to container types, so now I still register them as symbols to the type table but at least I know that they are generics and not just empty structs. Then, I went to the `get_func` function and added an assertion to make sure that the requested function is a fully resolved type.

Yess! There were multiple places where this assertion failed. Field accesses, identifiers, even closures! I fixed those cases, resolved the generics completely and it compiled fully!

Now, margarine's runtime is very slow right now (200-300MIPS), especially for the task day 2 was asking of me. So it took a good 30 seconds to run the entire test each time I wanted to see if it worked. I was counting for 4 garbage collection messages, because the garbage collection still had the debug prints for "garbage collected in 5ms". 

The tension was high is what I'm saying.

garbage collected in 4.7ms  
garbage collected in 3.5ms  
garbage collected in 5.2ms  
garbage collected in 4.1ms  

...  
...  

47353585426

..FFFFFU- ntastic! The problem still exists.  


Okay, well that whole refactor didn't fix the problem. It also didn't make it worse. Though, I can't help but wonder how my code was even running before that refactor. Like, I wasn't even resolving generics and it was working just fine. Anyways, not the time to celebrate when my program is still broken.  

Wait, let me check the output of the codegen. 
Waiiiittt, the codegen is IDENTICAL except for the extra batch of functions. 

## Oh god I've been looking at the wrong thing this whole time?
So the problem is in the runtime. Okay well, I could just output every single instruction and then figure out where the divergence happens.  

Except, when I tried that the output file was 14GBs and it took 15 minutes to run. Look, VSCode crashed while trying to open that file at 3GBs, NVIM couldn't even handle 2, and all that is before I even diffed it in the first place.  

I really doubt diffing it is going to work.

This threw me for a bit of a loop. I tried running it on smaller examples but the bug never happened in them. I had to find where the difference was in a 14GB log file. Except, I don't really need detailed logs on every part of the program, do I? I just need details around where the difference started. 

Here's an idea, what if we hash 100k instructions and print that. That'd cut down the file size by a lot! Then we can just figure out where the first difference is and add logs around there until we find the instruction that was causing the problem!  

Let me tell you, that worked amazingly! The problematic instruction was somewhere around 37 million instructions. 
![alt text](./assets/diff-1.png)

For some reason, that EqObj instruction was returning false.
![alt text](./assets/diff-2.png)

Ah! The split function was failing, probably. Wait no the split function is fine. What is going on here?  


From this point, I don't know how to explain it much but somewhere along the way I thought "alright, let's reduce variance and just give it a lot of memory to work with so the GC doesn't run".
![alt text](./assets/result.png)

Oh.. So the problem is the GC? Yep!  

But what exactly is the problem? No idea, that's a mystery I've yet to solve. For now, I've just disabled the GC until I figure out how to fix it. From my perspective, the GC looks perfectly fine, and the code is extremely simple.  

One day, I'll fix the GC. Today, however, is not that day.

If you have any theories, feel free to share them on [my discord](https://discord.gg/t7gNX8Kp72).  

It really goes to show how hard it is to track down the source of a bug sometimes. I mean that whole function cache thing was a giant red herring!  

Sometimes while trying to fix a bug you fix 5 others before fixing the actual bug that got you started. Makes you wonder how your code even ran properly in the first place, huh?

BTW, You can check out margarine [here](https://github.com/todaymare/margarine) and if you enjoyed this post [if you could spare a cup of coffee, it would be delightful](https://ko-fi.com/todaymare).  

¹: It's only 12 days now 🥀