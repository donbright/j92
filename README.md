# J92

This computer code, in the Rust language, provides data on the 
ninety-two Johnson Solids - Convex Polyhedra that have Regular Polygons 
as Faces. It can output vertex + edge data in various precisions and 
formats, including the following:

  String symbolic, such as "2,Φ,√3" (Φ=Golden Ratio)
  Binary floating point approximation, f16, f32, or f64
  ASCII Decimal approximation (using print!("{}",x) where x is f16,f32,f64

# Usage


```
fn main() {
    println!("{}",j92<f32>::PentgaonalCupola().vertices().count);
    println!("{}",j92<f32>::PentgaonalCupola().faces().count);
    for v in j92<f32>::SquareCupola().vertices() {
        println!("{}",v);
    }
    for v in j92<String>::SquareCupola().vertices() {
        println!("{}",v);
    }
}
```

# Coordinate number types

String symbolic, such as "2,Φ,√3" (Φ is of course the Golden Ratio)
is allowed by the fact we are not really calculating much when we 
build the vertex lists, we are building the Polyhedron literally from
string symbols. 

Binary Floating Point for the standard Rust floating point types such as 
f16 and f32 is achieved by a custom floatify function which takes as 
input a small set of strings, and converts them into num_traits::Float.
This works for this paritcular j92 project because we have a small and 
finite list of corordinates in symbolic form that we actually need,
so floatify() is just a few dozen lines of code.

Binary floating point for 16-bit float types is achieved through the use
of the 'half' crate with the num-traits feature turned on. This allows
the use of num_traits::Float code like floatify with f16. It however
doesn't allow using floating point literals but that's OK, we have workarounds.

f128 doesn't work because the f128 crate doesn't implement FromStr

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

