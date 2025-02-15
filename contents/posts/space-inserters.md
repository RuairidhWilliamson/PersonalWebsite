# Space Inserters

October 2023

C#, Unity, GameJam, Rust

---


For Ludum Dare 54, we decided to take a different kind of game entirely. The theme was `Limited Space`. We interpreted this very literally as the character space and there are limited number of spaces. Our game is inspired by the wordle genre of games.

![](/assets/images/space_inserters.png)

The premise of the game is that the player is presented with a jumble of capital letters. The player may move their caret around the letters and start typing letters to create words. They use the space bar to finish one word and start on the next. Once they have used all the capital letters to make words they complete the puzzle. The puzzle is difficult because the player essentially chooses where to subdivide the letters and then appends their letters on to each subdivided section.

## Examples

For example the puzzle might be 
```
MONAACRI
```
with at most one space allowed.

So one potential solution to this is
```
MONArch ACRId
```
Forming the words monarch and acrid.

A few other puzzles are
```
PONTOOCCUPA (1 space)
ABSEHOTOO   (2 spaces)
PACAICHAG   (2 spaces)
```
and possible solutions are
```
PONTOon OCCUPAtion
ABSEnt HOme TOOth
PACk AIoli CHAGrin
```

## Puzzle Difficulty

The core of what makes this game satisfying to play is being able to solve the puzzles. If the puzzles are too easy then the player does not feel sufficiently challenged but if the puzzles are too hard the player will feel that it is not possible and feel cheated. To strike this balance is tricky for this game because it relies on knowledge of English words and how likely players are to know those words.

The reason the last puzzle in the example above is harder than the others, is because it uses two unusual words aioli and chagrin. It combines them in places that make it ambiguous where the word boundaries lie and hence where the spaces should be inserted. But ambiguity alone is not enough as if a puzzle is too ambiguous it will have many solutions making it very easy. Striking this balance is where most of the work on this game went and is detailed by the two modes we made.

## Modes
We had two different modes in the game. An infinite puzzle mode where puzzles are generated randomly and get harder over time. And a daily puzzle where five puzzles were hand picked everyday as a special challenge.

### Infinite Mode

The infinite puzzle mode and random generation algorithm was implemented by my friend so I don't want to get into the details of that. The general idea of it was to use a dictionary to pick out words, choose a random prefix from each word and join these together to make the puzzle.

### Daily Mode

The daily puzzle mode was implemented by the game contacting a web server implemented in rust and hosted on Google Cloud Run. I hand-made 5 puzzles for each day. My goal was to make the puzzles ramp up in difficulty so the first two are easy the third is average and the fourth and fifth are hard. When implementing this I realized I would have to premake most of the puzzles ahead of time so that I didn't have to make the puzzle on the day it was due.

I found that premaking a lot of puzzles was difficult to do by hand, so I developed some tools (all in rust) to help me. My tools included:
- Random word of a specific size
- Solve puzzle takes a puzzle input and number of spaces and finds solutions
- Generate puzzle using random numbers within parameters

These tools were helpful in creating puzzles. I could generate a few random words and join them in a way I liked. Then I would use the solve tool to find how many solutions there are which would give me an idea of how difficult the puzzle was and where to place it in the order of the day. The generate tool was also helpful as it did these two steps automatically and would make it very easy to generate a lot of puzzles quickly. Using this I could quickly select puzzles I liked and discard puzzles I found too easy or too hard.

As part of the solver I had different metrics of how many solutions there existed for a given puzzle. Counting the number of solutions was useful but if one prefix had many different words that could be formed using its prefix then it would warp the number. I added another metric which was the minimum number of words for a given prefix. This was better because it factored in how hard the hardest prefix was to find a word for. By looking at both of these metrics for puzzles it gave a better idea of how difficult a puzzle was.

I ended up creating 17 puzzles and decided that to avoid having to create puzzles forever I would have it loop every 17 days.

The game was received well considering it is very different to most games. Achieving 483rd in overall, 472nd in fun and 156th in innovation.

[View Project](https://ruairidhwilliamson.itch.io/space-inserters)
