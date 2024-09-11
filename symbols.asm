
# Define the capabilities the program needs. These will be stored in the header
# of the executable format. Besides memory allocation there are others to ask for.
ask @[8:]       # ask for eight pages of accumulator
req @[4:]       # require a minimum of four pages of accumulator to run
req $[1/4:]     # require 1/4th of a page of cache to run (256/4 = 64)

# Link in functions from other files or libraries
# use thingamagadget.asm

# Variable definitions in the scope of func main. These are similiar to labels,
# and have a colon ':' after the variable name.
    accum   : .stuvwxyz,    8byte, capability
    cache   : .stuvwxyz,    8byte, capability
    foo     : .stuv,        4byte, unsigned
    bar     :     .wxyz,    4byte, unsigned # .wxyz gets packed into the previous dword.

# main is a label, but the func keyword makes it a function label.
func main:

accum = @           # save accumulator
@ = @[:0...3]       # Square brackets '[' and ']' imply indexing -- not direct memory access

return              # func main ends
