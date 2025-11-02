# So I became God: Artificial Life
Let me take you through a journey of curiosity, growth, and cannibalism. One inspired by a video game and some questionable media..

About a year ago, I watched the [Black Mirror episode "Plaything"](https://en.wikipedia.org/wiki/Plaything_(Black_Mirror)) (awesome ep. btw). I didn't think much of it then but it was my introduction to the idea of artificial life.

Obviously, even before then I'd seen the fancy YouTube videos, you know the "I made an ecosystem in Unity!" ones. But again, still nothing.

Until I saw a roguelike called [Primordialis](https://store.steampowered.com/app/3011360/Primordialis/). 
It's this physics-based roguelike where you design your own creature out of cells (muscles, spikes, electric organs, etc.). And your enemies are also designed in the same way. Now I don't know if you see the vision but if you do, you already know what's coming.

While playing the game I just couldn't help but imagine a world with the same rules of Primordialis, one where the creatures can evolve not only their tiny lil brains but also their body.

So I became God.

![auto](./assets/the-first-day.webm)

## The first day
In the beginning, there was nothing.  
And before there was nothing, there was ~~MONSTERS~~ Rust.

Of course, the first step was to decide what our little goobers (that's the name I called them in the code, yes) would be made out of. The elements, if you will.

But before I could even start on the fancy simulation I needed to fill this one glaring hole in my knowledge. I had never touched a neural network in my life!

So I did what every responsible dev does, I went on YouTube and learned about them. I'm kidding, I just asked ChatGPT to explain it to me. Then I decided to write my own version, because honestly that sounded easier than using Rust libraries¹. I made the simplest of feed-forward neural networks.

The first goal was simple, collect all the apples as fast as possible.
![auto](./assets/apples.webm)

It went in the most boring way ever, it went alright. I had about 100 agents running for 5ish minutes and it had already perfected everything.  

Which, while a little disappointing, meant that I had the green light to go ahead and work on the fancy simulation now.

## The second day
Let there be motion.

Previously, each goober had its own instance. But now they'd have to share the same resources.  
And for them to actually care about those resources, there had to be stakes.

The rules of life were simple:  

Eat, you reproduce.   
Move too fast, you get hungry.  
Get too hungry, you die.  

The goobers also got a vector pointing in the direction of the nearest food, ah the simpler times.
Of course, when they reproduced there was a small chance to evolve their brain.

![auto](./assets/technically-survival-of-the-fittest.webm)

And then, the great blinding occurred. After that instead of having a direction to the nearest food, they would have to see.  

I made it so each goober had to shoot out 8 rays at slightly different angles in order to see². It'd return a 1 if it was food, -1 if it was another goober and 0 if there was nothing.

![auto](./assets/the-great-blinding.webm)

As you can (maybe) see, strategies were starting to develop. Some went with the strategy of vibrating in place, some went with the strategy of hunting food, and some went with the strategy of going as fast as fucking possible, hoping they'd hit some food before they died.

Well, it's cool 'n all but single cells are BORING.  
So after that little experiment was over I was ready to move onto multicellular organisms, also known as drawing more than one rectangle.

So I went back to isolated worlds, which started with the goobers being able to modify their velocity directly.

![auto](./assets/more-apples.webm)

Which then led to me restricting their movement to "Look Left/Right" and "Move forward"

![auto](./assets/are-they-stupid.webm)

Are they stupid?  
Yeah.  
They're trying, okay


## The third day
Let there be... fat people?

Now that we have goobers, and they can move around, eat food, and reproduce we can finally get to the entire point of this project and give them more cells to work with.

At first I had very simple cell types that were basically stat modifiers. There was a SpeedyCell that increased your max speed & decreased how much energy you lost from speeding. Or a basic cell that just increased your mass.  
Oh and also a HealthyCell which decreased the basal metabolic rate and fat cells that could increase the maximum energy they could store.  

And uh.. that was sorta it for a bit but I'll get to that later.³

Of course cells came with some drawbacks as well. Each cell would increase your mass and weight, the heavier you got the slower you could move and the more energy it took to do the same actions.  
Which obviously, led to the sword meta.

![auto](./assets/the-sword-meta.webm)


So uh, this is about the point where I started to fear I created a cult. But obviously the swords had to be stopped.  

My mate Kiniro (the dev of the game [Faster Bunnies](https://store.steampowered.com/app/3943850/Faster_Bunnies/)) suggested a great fix: make goobers grow to their full size before they can spawn children.  

Which actually worked!  

Except now they were just not growing at all.

![](./assets/smol.png)


So I thought: well, if these guys don't want to grow, I'll just let them fight it out. 

## And on the fourth day..
I let them fight.  

I gave them weapons (Spike cells). I gave them shields (..shield cells). But most importantly, I gave them an incredible 12 floats of memory⁴ so they could avenge their father.
![auto](./assets/i-let-them-fight.webm)

Combat wasn't random either, getting hit on the side dealt more damage than getting hit on the head. And every collision damaged both sides. I thought they'd learn to be strategic about it, angling attacks & dodging, but it mostly just led to the big guys eating their babies for food. 

Evolution is a cruel thing, huh?


## On the fifth day
> the scientists who studied the rivers  
> were forbidden to speak  
> or to study the rivers.

Which is totally irrelevant. I just didn't have a better way of ending this post.

This was a really fun experiment that I'm definitely planning on returning to⁵.  

If you enjoyed it, laughed, or want to join the cult of rectangles. Maybe [buy me a coffee](https://ko-fi.com/todaymare)?  
Or if you would like to fork around [here's the repo](https://github.com/todaymare/evolution-sim). If you have any questions or feedback feel free to [join my discord server](https://discord.gg/t7gNX8Kp72) or mail me at [contact@daymare.net](mailto:contact@daymare.net)


---

¹: Related: [Everybody's So Creative!](../everbody-so-creative/)

²: To be honest the way I handled raytracing was horribly inefficient. I've made raytracers before so I know that they can be fast, it's just that I really could not care less about it. I wonder how many hours of simulation time I could've saved had I only made it slightly more optimized by just adding a spatial grid or something

³: After reproduction each goober had a small chance to mutate their neural-network and/or delete/add/modify one of their cells.

⁴: I felt like a genius coming up with this idea, though I have no idea if the neural networks ever used it. It's basically just 12 floats that get passed into the neural-network and then are preserved for the next iteration. Yes, that's what I called "memory". You know, so they could store information about life's most important questions such as "Is this food?", "Is this enemy?", "Am I food?", etc.

⁵: Okay this one probably could've been inlined but I wanted to talk a bit more about it after I had already finished writing. I think the biggest failure of this experiment was that the goobers never really got a chance to live, their really short life revolved around "eat food as fast as possible" with nothing else they could do. Maybe in another iteration I could allow them to build, move food around, live 5 mins on a single piece of food instead of 30 seconds and just be more patient with them. There's a lot of ground to explore here.⁶

⁶: I really enjoyed writing these footnotes and this entire post in general, can you tell?⁷

⁷: Fun fact, the videos and screenshots for this post were taken months before I even had the idea for the website. So it's a total coincidence that the colours fit so perfectly.. wait is my favourite colour green??