# J92

**********

In progress, not usable right now 

*********

This computer code, in the Rust language, provides data on the 
ninety-two Johnson Solids - Convex Three Dimensional Polyhedra that have 
Regular Polygons as Faces. It can output vertex + edge data in various 
precisions and formats, including the following:

  String symbolic, such as "2,Φ,√3" (Φ=Golden Ratio)
  Binary floating point approximation, f16, f32, or f64
  ASCII Decimal approximation (using print!("{}",x) where x is f16,f32,f64

# Usage


```
fn main() {
    println!("{}",j92::PentgaonalCupola().vertices().count);
    println!("{}",j92::PentgaonalCupola().faces().count);
    for v in j92::SquareCupola().vertices() {
        println!("{}",v);
    }
    for v in j92::SquareCupola().vertices().to_f32() {
        println!("{}",v);
    }
    for v in j92::SquareCupola().vertices().to_f16() {
        println!("{}",v);
    }
    for v in j92::SquareCupola().vertices().to_rugfloat() {
        println!("{}",v);
    }
    let p1 = j92::Point::new("2,1,0");
    let p2 = j92::Point::new("2 1 0");
    let p3 = j92::Point::new("[2 1 0]");
    let p4 = j92::Point::new("<2 1 0>");
}
```

# Coordinate number types

String symbolic, such as "2,Φ,√3" (Φ is of course the Golden Ratio)
is allowed by the fact we are not really calculating much when we 
build the vertex lists, we are building the Polyhedron literally from
string symbols. 

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

# Design of Geometry objects (Point, Edge, Face, Polyhedron)

The code is based on a special way of imagining the numbers that are the
coordinates of a Point in Space. 

Let us start with traditional computer graphics. In the olden days of computers
you would see a lot of code like this:

      Point: x: int, y: int

We say that the "coordinates" of the Point are integers. Now in more modern
code you might see something like this:

      Point: x: f64, y: f64

We would say that the coordinates of the Point are 64 bit floating point
numbers, implying probably the IEEE format for most computers.

Now what if I want Point to basically have any type that I want? In this
case we are focusing on the actual number type used to describe each coordinate.

And in this project, j92, the Point is designed to use a very diverse set
of number types.

How is this accomplished? Basically, its like this;

      Point: x: PseudoField, y: PseudoField

However instead of Point being a Struct, its actually a Trait, in other
languages this is called an Interface. It defines the behavior of a Point
without specifying how the data inside the Point is stored. You access
the values x and y via calling methods (functions) on the Point rather than
reading a memory location. 

And instead of int or f64, the coordinates are PseudoField, which is another
Trait which defines how objects which are Numbers will behave. Rather than
specifying how their data is stored in memory.

Is this slow? Yes. But for j92 speed is considered irrelevant, we are going
for an extremely flexibly code base that will allow various forms of output.
We don't care about speed in this situation. 

# PseudoField number type

What is a PseudoField? The name comes from the mathematical concept of a 
Field, which is a set of numbers and the operations on them, such as 
addition and multplication. In the Field of normal arithmetic we learn 
in grade school, there is an additive inverse and a multiplicative 
inverse. In other words, you can add something to any number in the set 
and get 0, and you can multiply any number by something to get 1.

Now why would I say "Pseudo"? Because IEEE floating point numbers do not 
form a Field. If you add an f32 to another f32, you don't necessarily 
get back a number that is within the f32 set of numbers. Same with 
multiplication, especially multiplicative inverses. And associativity
and commutativity. 

And in computer math, often we use int64 or floatf64 imagining that they
will never need to deal with overflow/underflow, but in fact they do, so
these are not Fields either.

But I certainly wouldn't want to throw out IEEE floating point numbers 
or integers entirely, so hence the name PseudoField. 

# What about a Finite Field with Field Extension of the Golden Ratio or...

That is beyond the scope of this project.

# The end

Thanks for reading

