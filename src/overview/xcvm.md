# The XCVM

The XCVM is a virtual machine that processes XCM programs.
It’s an ultra-high level non-Turing-complete computer whose instructions are designed to be roughly at the same level as transactions.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
A message in XCM is simply just a program that runs on the XCVM.
XCM instructions might change a register, they might change the state of the consensus system or both.
