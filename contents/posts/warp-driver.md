# Warp Driver

October 2021

C#, Unity, GameJam

---

![Warp Driver](/assets/images/warpdriver.png)

For the fourth time a friend and I participated in a game jam. This was Ludum Dare 49 and the theme was unstable. We had many ideas and settled on space golf like game with unstable orbits. We had 48 hours to finish the game.

![Warp Driver](/assets/images/warpdriver_animated.gif)

The first day was spend programming the gravity physics. It consisted of defining a `SpaceObject` class that could be used for all the space objects in the game. It defined the mass and radius of the object . The mass was used for the gravity force calculation (just sticking with classical Newtonian mechanics). The radius was used to perform collision detection. If two space objects radii overlapped this was a collision, and we had to figure out what should happen. This solution worked nicely it was quite easy to get planets to orbit each other. We wanted to black holes too. We could define them as a `SpaceObject` and give them a large mass and small radius however we wanted a visual representation of the black hole. We created a shader using unity's shader graph that captured the rendered image from behind the black hole and rerendered it with some modification. We came up with an equation for how to distort the game around the black hole. It is probably not accurate to real life, but it gives a convincing effect. We also created a planet shader to randomly generate the planets, an asteroid shader to randomly generate asteroid shapes.

The second day was spend designing levels and polishing the game. We added wormholes made of layers of spinning shader textures. We designed 9 levels which each introduce mechanics and test the player. It was also important that every level be possible in one shot. This added extra value in that after players had played the game once they could try and complete it again in the minimum number of shots. One of the ways we incentivized players to play again is once they completed the last level they would have the chance to upload their score and time to online leaderboards.

[View Project](https://wilkoco.itch.io/warp-driver)
