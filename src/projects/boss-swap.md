Ludum Dare 51. After the quite ambitious 3D game [Climate Crisis](/projects/climate-crisis) from last time we decided to do a 2D game to allow us to focus on the gameplay and the fun aspect. There were a few internet disruptions this time which reduced our time significantly and meant we did not implement all the features we wanted.

![Boss Swap](src/assets/images/boss_swap1.png)

The theme was `Every 10 seconds`. The idea we had was to have a boss fight that switched boss every 10 seconds. The idea being that the player would have to adapt and predict the boss changes to master the game.

The player has typical 2D controls, the four directions (allowing diagonal movement too) and a roll that grants invincibility while in the roll. One point of feedback is that players did not realise there was a roll or if they did, they did not know the roll granted them invincibility and so didn't use it.

The player has two attacks a primary and secondary. The primary attack is a short range shot gun like blast which is best used up close to a boss. The secondary attack is a long range single shot. By giving the player choice of attack, we encourage the player to think about the distance they engage from.

There are 3 bosses with 3 unique strengths and weaknesses.

- The fiery boss has a lot of projectiles which require careful and precise dodging.
- The summoning boss can leap towards the player crushing them if he lands on them. He can also summon slimes and rocks which can hurt the player.
- The final boss is the rhino which will charge the player very quickly.

The player has a fixed number of lives while the bosses have a shared health bar that must be depleted to win the game. The player has a choice of 3 difficulties which vary the boss's health and the player's lives.

![Boss Swap](src/assets/images/boss_swap2.png)

The implementation of this game was fun because we had to implement some relatively complex systems quickly. These included each bosses abilities, the boss swapping mechanic, AI for all the bosses, AI for the slimes, player movement and damage systems.

One area that was troublesome was the leaping boss. I originally implemented this by calculating the required velocity for each frame to move to follow the curve from start to destination, however this technique would often miss the destination because the frame in accuracies would compound. This wasn't too much of an issue however because the boss was leaping to where the player was, when the player was near the edge of the arena, there was a chance for the boss to jump out of the arena. To solve this I reimplemented it to simply linearly interpolate along the curve over the leap time. This way the boss always landed in the correct destination.

![Boss Swap](src/assets/images/boss_swap3.png)

I am very pleased with how this entry turned out. We really focussed on the fun aspect of the game and that was reflected in our scoring.

[View Project](https://wilkoco.itch.io/boss-swap)
