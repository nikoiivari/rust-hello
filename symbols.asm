# a comment
# a second comment line
# a third comment line

# Start with variable definitions. These are similiar to labels,
# and have a colon ':' after the variable name.
accum   : .stuvwxyz,    8byte, capability
cache   : .stuvwxyz,    8byte, capability
foo     : .stuv,        4byte, unsigned
bar     : .wxyz,        4byte, unsigned # .wxyz gets packed into the previous dword.

accum = @
@ = @[:0...3]       # Square brackets '[' and ']' imply indexing -- not direct memory access
