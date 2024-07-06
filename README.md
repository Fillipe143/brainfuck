# Brainfuck
Brainfuck interpreter written in rust 

## Documentation
| Character | Description |
| :---: | :--- |
| > | Points the pointer to the next cell. |
| < | Points the pointer to the previous cell. |
| + | Increment current cell value by one. |
| - | Decrement current cell value by one. |
| . | Print the current value as an ascii. |
| , | Reads user's ascii and saves in current cell. |
| [ | If the current value is zero, skip to the next command after its closing ']', otherwise execution continues. |
| ] | If the current value is different from 0, it returns to its respective '[', otherwise, it continues execution. |

## Examples
Hello World:
```
>+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]
<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.
```


# Reference
https://en.wikipedia.org/wiki/Brainfuck
