# J92

This computer code, in the Rust language, provides data on the 
ninety-two Johnson Solids - Convex Polyhedra that have Regular Polygons 
as Faces. It can output vertices in various precisions and formats.


# Usage


```
fn main() {
    println!("{}",j92::PentgaonalCupola().vertices().count);
    println!("{}",j92::PentgaonalCupola().faces().count);
    for v in j92::SquareCupola().vertices() {
        println!("{}",v);
    }
}
```

# Design Philosophy

This only does the ninety-two Johnson Solids, no more and no less. 
None will ever be added or taken away.

The library is designed so it can spit out code for various uses,
for example if you want just the raw ASCII decimal numbers for an OBJ
file, you can get that. If you want high precision floating point numbers
for coordinates you can get that too.

The dependency count should be somewhat low for the library itself. That
is why it has it's own Point type and other stuff.

The dependency count for examples can be high. 

# The end

Thanks for reading

