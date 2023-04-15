# Global static variables

C++ allows file-scope, function-scope static variables but
Rust does not. Rust does not have built-in support of static variables
having non-trivial initialization and deinitialization because
C++ has shown several problems with non-trivial static variables.
However there is lazy_static crate....

Rust output is:

    new()
    fly()

C++ output is:

    Bird()
    fly()
    ~Bird()

