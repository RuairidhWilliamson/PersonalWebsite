# Parcel Purgatory

May 2023

C#, Unity, GameJam

---

## Background
As in the past, me and a friend participated in Ludum dare 53 this time the theme was `Delivery`.

Our puzzle game is inspired by a web game called bloxorz or as we later found another game called [cuboid](https://en.wikipedia.org/wiki/Cuboid_(video_game)).
The simple concept of the game is that the player controls a cuboid with dimensions `2x1x1`. They can move the cuboid in any of 4 directions rotating about the edge. The game is a challenging puzzle game because the player is on a grid with a limited number of tiles. If the player moves off the tiles they will fall and be reset.

![Parcel Purgatory](/assets/images/parcelpurgatory.png)

## Mechanics
Our game changes some rules from its inspiration. In bloxorz if the player moves their cuboid so that it is partially on a tile but partially not on a tile it will roll off. We decided to allow this situation which means that the player has more freedom to maneuver but as a side effect the levels have fewer tiles in order to constrain the puzzle.

The backstory behind the cuboid is that a person is inside the parcel and is rolling about to escape a large warehouse. This meant we also created security cameras and security robots that if they spot the player will reset them. To increase the puzzle complexity and rely less on player reaction speed the robots and cameras only move when the player moves. This gives the player time to think through their decisions and forces them to be more tactical.

Towards the end of the game we also introduce some more puzzle concepts such as the buttons that shrink and grow the player. These effects run out after a certain number of player movements.

![Parcel Purgatory GIF](/assets/images/parcel.gif)

## Implementation
The implementation of the player movement was the most complicated part. Correctly rotating about the player and detecting when they should fall took some iteration.

We also had a level building system where we created a lower resolution pixel image of the level where each pixel represents a tile in one of several states (floor tile, empty, obstacle, start, end). This image is then processed and used to place objects in the level. But we also added decorations separately inside a prefab for each level. The level manager script would then animate all these objects individually to fly in and out as the player entered the level. The tiles of the level fly in from below and the decorations fly in from above. They are all offset slightly in timing so that it looks more interesting. One problem we had with the tiles flying from below is that the camera is pointing downwards, so the tiles are visible for quite a while from very far below which is visually distracting. So instead we added a shader on the tiles that fades them out based on their `y` position.

One tricky part of the level system is that the start of a level must connect to the end of the previous level. We build a short walkway between levels but the direction of the walkway still had to match the start and end.

The robots and cameras were originally implemented to move irrespective of whether the player moved, but it wasn't too difficult to make this change. The robots and cameras also show a red warning graphic where their vision cone is. However in play testing we noticed that players were not sure where the cameras and robots would move next, making it very difficult to play without studying their full cycle. To address this we created a grey warning indicator to show where the robots and cameras vision would be on the next movement.

## Levels

We made 9 levels with a slow increasing difficulty. The early level designs were very important because we did not include any tutorial in the game. We had to introduce the concepts required to solve later puzzles in ways the player could figure out for themselves.

The first level is extremely simple it has a lot of floor tiles and is designed for the player to try moving around and to learn a bit about the controls. Crucially the exit on the first level is not lined up such that taking the shortest or most obvious path will allow you to exit. This forces the player to learn that they will have to readjust their position to exit.


The second level has far fewer floor tiles and at the start we have a gap that can only be crossed if the player realises that they can hang half of the box over the edge. Demonstrating this early is important as players may be familiar with other games where you cannot do this. The end of level two also has a learning opportunity where the exit requires the player to be in a correct state but the only place where the player has enough floor tiles to change their state is slightly backwards. This teaches the player that sometimes they will have to backtrack to get in the right orientation. Level three is similar to level two, but it asks the player to prove they understand these concepts.

![Parcel Purgatory Level 2](/assets/images/parcelpurgatory2.png)

Level four introduces gaps that the player can cross firstly with a simple gap that has when traveling long ways has only one incorrect way of solving. The next gap is more complicated and has only one correct way to solve it. The player should have an understanding that they can cross gaps but may have to adjust their position. This will be important for level seven.

![Parcel Purgatory Level 4](/assets/images/parcelpurgatory3.png)

Level five introduces the security robots. It first has an easy section that is designed such that the player can see the robot moving and hopefully realise the robot only moves when they move.

Level six introduces security cameras. The level is very simple in terms of puzzle but requires the player to wait for the cameras to do wasted moves in order for the cameras to move into positions where they can move past. In hindsight, it would have been better to swap level five and six. In play testing level five presented more trouble for players than level six and introducing the concept of vision cones is more gently done in level six. The end of level six has some more traditional position puzzles.

![Parcel Purgatory Level 6](/assets/images/parcelpurgatory4.png)

Level seven is the first level that combines all the previously seen concepts into a long puzzle. It starts with the gap trick seen in level four followed by a security camera that requires the player to waste some moves in order to let it pass them. Then there is some tricky positioning required and security robots to avoid. Level seven is particularly challenging because if the player is reset they will go back to the start of the level. When they are near the end of the level and make a mistake it is more punishing. Level seven is probably the hardest level and maybe should have been moved to the end.

![Parcel Purgatory Level 7](/assets/images/parcelpurgatory5.png)

Level eight introduces growable and shrinkable boxes. The puzzles here are tricky but since they are not combining multiple concepts as before. The puzzle to get out the end of level eight is particularly tricky since there is only one way to solve it which in play testing players found difficulty to find. Level nine is a continuation of the growable and shrinkable boxes but with more difficult puzzles. Once the player leaves through the exit on level nine they complete game.

## Thoughts
This game is actually quite fun to play. The puzzles are quite challenging and the difficulty progression from level to level is pretty good.

The weak areas of this game is the graphics. They are very bland which you can argue is part of the aesthetic but adding some more decorations would go a long way. I would have liked to add plants and green vegetation to add contrast with the plain white walls, brown parcels and black background. The background should have maybe been a gradient colour instead of constant dark.

It would have also been nice to more clearly show there is someone inside the parcel. There are some eyes textured in the handles, but it is very difficult to see. This could have been achieved with a cut scene.

A lot of time was spent building levels and play testing which did result in good levels so trading this for better aesthetic is maybe up to personal preference.

We included a timer for how long they player has been playing their current run for and a step counter. This was to add the option of speed running and challenge of minimizing the steps to solve all the levels. From my testing I was able to get my time down to around 3 minutes for the whole game. There is a hard limit to how fast you can go because it takes a fixed time to do each step. Once you have the optimal route and have it memorised it would be easy to get the optimal time.

The feedback we got was very positive, and we achieved 153rd and 147th in overall and fun respectively.

[View Project](https://wilkoco.itch.io/parcel-purgatory)
