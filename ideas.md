# Optimization

- [x] <s>Use an iterator, not a SmallVec for iterating over moves in MinMax. Then one could use the bit_loop</s>
  - implemented, large impact
- [ ] Switch board to vertical mode 
  - [ ] Fix bugs for #cfg(straigt_board)

# Algorithms
- [x] <s>What if minmax evaluation used a rollout?</s>
  - Tested, wasn't great. Test should be repeated though.

# TO FIX
- [x] <s>When a light piece reaches the opposite end of the board and hops on an opponent's piece at the same time, it should
be removed. The client then only get's _one_ point.</s>
  - Should be fixed now