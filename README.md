# rs-dawg
Construct "Directed Acyclic Word Graphs"

A directed acyclic word graph is really just a minimal deterministic finite state automaton 
that can be constructed incrementally from a sorted list of strings. Where by "incrementally"
I mean that the automaton remains deterministic and minimal as each string is added. And this
without actually having to execute the subset construction or minimization.

Furthermore, a DAWG can be used to implement a perfect hash function, which will just assign
to every word in the original list its ordinal position in that list (starting with 1).

Probably the best way to think of a DAWG is as a Trie structure that allows you to tie together
shared suffixes in a single path. So they are good replacements for many applications of tries.
In particular, they make a very good foundation for implementing Lexicons in NLP systems.

# Status

This is currently a work in progress. The library is complete, but untested and under-optimized. 
And it still needs a means to serialize compiled DAWGs to file, and a reference implementation
that will read a file and store the resulting DAWG.

# References

There are many implementations floating around on the web in various different languages. 
But this is the paper you should read first:

[Daciuk, Jan, et al. "Incremental construction of minimal acyclic finite-state automata." Computational linguistics 26.1 (2000): 3-16.](http://www.aclweb.org/anthology/J00-1002.pdf)
