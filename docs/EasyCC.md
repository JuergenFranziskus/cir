# Easy Calling Conention
The Easy calling convention is an alternative to the prevelant SystemV calling convention on Unix X86 Systems.   
I made it up because implementing SystemV was real hard and I didn't wanna figure it out yet.   
It will probably be deleted once SystemV is properly implemented.   



## Stack Alignment
Upon execution of a 'call' instruction, the stack needs to be aligned to 16 bytes.


## Parameter passing
The first six parameters from left to right that are at most eight bytes are passed in registers.  
The register sequence is the same as in SystemV: RDI, RSI, RDX, RCX, R8, R9.  
All remaining parameters are pushed onto the stack from right to left.  
Padding is inserted as needed to maintain alignment.   
Varargs are not supported.



## Return Values
If a return value is at most eight bytes, it is returned in the RAX register.  
Otherwise, the calling function must allocate space for it and pass a pointer in RAX.


# Register Preservation
| Register | Usage | Preserved |
|----------|-------|-----------|
| RAX | Return Value | No |
| RBX | Callee-saved | Yes |
| RCX | Parameter | No |
| RDX | Parameter | No |
| RDI | Parameter | No |
| RSI | Parameter | No |
| RSP | Stack Pointer | Yes |
| RBP | Base Pointer | Yes |
| R8 | Parameter | No |
| R9 | Parameter | No |
| R10 | Callee-saved | Yes |
| R11 | Temporary | No |
| R12 | Callee-saved | Yes |
| R13 | Callee-saved | Yes |
| R14 | Callee-saved | Yes |
| R15 | Callee-saved | Yes |

All other registers are caller-saved.
