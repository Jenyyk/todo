# ToDo app made in Rust
I just wanted to make something in Rust  

## Usage
**Commands:**
- **`help` prints help**
- `add ~` adds a task  
  example: `add feed the chickens`
- `del ~...` deletes tasks, based on index, can take multiple arguments  
  example: `delete 0 3 5` => deletes 0th, 3rd and 5th tasks
- `move ~ ~` swaps two tasks based on index  
  example: `move 1 2` => swaps 1st and 2nd task
- `color ~ r g b` colors a task based on the index and color input  
  examples:  
  `color 2 255 0 0` => colors the 2nd task solid red  
  `color 2` => resets the 2nd tasks color back to white
