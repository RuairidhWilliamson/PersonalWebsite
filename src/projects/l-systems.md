I was interested in L-systems which is a way of describing fractal patterns.

It works by having a start string e.g. "X".

Then having rules such as X -> XAX and A -> AB would result in the following:

```
n = 0: X
n = 1: XAX
n = 2: XAXABXAX
n = 3: XAXABXAXABBXAXABXAX
n = 4: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX
n = 5: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAXABBBBXAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX
...
```
        
Then by creating rules based on the string for example forward, turn left and turn right we can draw images.

[L-Systems Wikipedia](https://en.wikipedia.org/wiki/L-system)