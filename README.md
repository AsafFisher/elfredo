# elfredo
<p align="center">
<img
    src="https://bit.ly/3eG4Q3a"
    width="408px" border="0" alt="elfredo">
<br>
</p>
`elfredo` is a library that allows you to patch executables after they were
compiled. It utilize an extra embedded section to store data/configurations
that one might want to change without recompiling the binary.

Unfortunately I was able to make objcopy work only up to 113,650 bytes :disappointed:

There are two main components to any project that uses elfredo:
* Customizing your embeditor to mach your datatype (step 1)
* Calling the `get_embedded_data` method to retrieve the embedded
data (step 2)
  
After these two are implemented you end up with two binaries:
* [app executable].elf - Your editable app
* [embeditor executable].elf - The binary that you use to change the embedded data


Steps to integrate (See `./example`):
1.   Write your configurable data structure in the embeditor binary:
```rust
// my_embeditor.rs
use elfredo::embeditor::run_embeditor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    id: u32,
}

fn main() {
    // This will print the embedded data if exists
    if let Err(err) = run_embeditor::<Person>() {
        panic!("{}", err);
    }
}
```

2. Read the configurations by using elfredo's api:
```rust
// main.rs
use elfredo::get_embedded_data;
mod my_embeditor;
use my_embeditor::Person;

fn main() -> Result<(), failure::Error> {
    let person = get_embedded_data::<Person>().unwrap();
    println!("{:?}", person);
    Ok(())
}

```
3. Embed new data to the elf
```shell
john@ubuntu:/mnt/hgfs/elfredo/example$ cat person.json
{
"name": "Ronald",
"id": 5
}

john@ubuntu:/mnt/hgfs/elfredo/example$ cargo run --bin embeditor ../target/debug/example person.json
   Compiling elfredo v0.1.0 (/mnt/hgfs/elfredo)
   Compiling example v0.1.0 (/mnt/hgfs/elfredo/example)
    Finished dev [unoptimized + debuginfo] target(s) in 24.99s
     Running `/mnt/hgfs/elfredo/target/debug/embeditor ../target/debug/example person.json`
"/tmp/.tmpXh5QHg"

john@ubuntu:/mnt/hgfs/elfredo/example$ ../target/debug/example
Person { name: "Ronald", id: 5 }
```

# Roadmap

- [x] Dynamic elf patching on linux
- [ ] Dynamic PE patching on windows (Using Resources WinAPI)
- [ ] Dynamic MAC-o patching
