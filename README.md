# HopSlotMap segfault

HopSlotMap sometimes segfaults when clear() or drain() is called on it. This doesn't happen with SlotMap or DenseSlotMap.

The example in this repo is as far as I've been able to narrow it down.
