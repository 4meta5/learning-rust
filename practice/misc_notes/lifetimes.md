# Lifetimes
> from that linked list tutorial

Lifetimes are unecessary in garbage collected languages because the garbage collector ensures that everything magically lives as long as it needs to. Most data in Rust is manually managed, so that data needs another solution. C and C++ give us a clear example what happens if you just let people take pointers to random data on the stack: pervasive unmangeable unsafety. This can be roughly seperated into two classes of error:
* Holding a pointer to something that went out of scope
* Holding a pointer to something that got mutated away

Lifetimes solve both of these problems and 99% of the time, they do this in a totally transparent way.

So what's a lifetime?

Quite simply, a lifetime is the name of a scope somewhere in a program. That's it. When a reference is tagged with a lifetime, we're saying that it has to be valid for that entire scope. Different things place requirements on how long a reference must and can be valid for. The *entire lifetime system is in turn just a constraint-solving system that tries to minimize the scope of every reference*. If it sucessfully finds a set of lifetimes that satisfies all the constraints, your program compiles! Otherwise you get an error back saying that something didn't live long enough.