# Delima - A CLI delimeter-based wildcard text replacer written in Rust

## USAGE
```
> delima "this.text?needs;changed" ".%<0%does *|?%>0%*%-1|;%>0%*ing?%-2" --replace_delimeters
```
## OUTPUT
```
> does this text need changing?
```

#  Syntax Is Scary!
The usage statement above may seem scary, but writing a delimeter pattern match has very few rules. Lets break down the delimeter pattern from above to get our toes wet.
```
".%<0%does *|?%>0%*%-1|;%>0%*ing?%-2" --replace_delimeters
```
First we can take notice of | being used as a separator for the individual delimiter rules
```
command 1: .%<0%does *
command 2: ?%>0%*%-1
command 3: ;%>0%*ing?%-2
```
Still looks a bit scary, but now lets make the same observation that we made about | with % as a separator between rule properties. Lets break down command 1.
```
command 1: .%<0%does *
--- Separate by %

delimeter: .

word shift direction and count: <0 
// first word on the left of the delimeter (zero indexed)

text transformation: does * 
// the * represents the word you are currently on and all text around it will be tacked on in relation to that word.

character trim direction and count (defaults to 0 when not supplied): 0
// if a positive number is provided, trims that many characters from the START of the wildcard word, and from the END if it is negative. 0 means no trimming will occur
```
### Input text
```
"this.text?needs;changed"
```
### Output text with command 1 applied
```
does this text?needs;changed
```
---
## Moving on to command 2
```
command 2: ?%>0%*%-1

delimeter: ?
word shift direction and count: >0 
text transformation: * 
character trim direction and count: -1
```
### Input text
```
"does this text?needs;changed"
```
### Output text with command 2 applied
```
does this text need;changed
```
---
## Moving on to command 3
```
command 3: ;%>0%*ing?%-2

delimeter: ;
word shift direction and count: >0 
text transformation: *ing? 
character trim direction and count: -2
```
### Input text
```
"does this text need;changed"
```
### Output text with command 3 applied
```
does this text need changing?
```