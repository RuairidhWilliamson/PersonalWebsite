# Rust Chess Engine

September 2019

Rust, Chess

---

![](/assets/images/fallback.png)

A min max chess engine written in rust.

This engine is similar to the python chess engine except rewritten in rust and it uses the python program to communicate with the lichess API.

Min max is a great algorithm for turn based games like chess but it takes exponentially long for the number of turns to look ahead.

This can be improved by removing some moves to look at because they are too bad but this can make it impossible to spot long sacrifices that benefit.

I originally implemented a similar algorithm in python and tested it using the Lichess API to play against it and test it.

Python is quite slow for this and so I turned to Rust to offer greater performance.

It was more challenging to write it in Rust since I have never programmed using it before.

The result is not perfect and is not very good at chess.

[View on GitHub](https://github.com/RuairidhWilliamson/chess)
