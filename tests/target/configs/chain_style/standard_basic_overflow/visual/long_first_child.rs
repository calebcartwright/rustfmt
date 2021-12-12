// rustfmt-indent_style: Visual
// rustfmt-chain_style: StandardWithOverflow

fn long_first_child() {
    // Args that do not fit
    let bar =
        foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
             .foo()
             .bar()
             .baz();
    // better next-line fit
    let barrrrrrrrrrrrrrrr =
        foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
             .foo()
             .bar()
             .baz();
    // better same-line fit
    a = foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
             .foo()
             .bar()
             .baz();

    foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
         .foo()
         .bar()
         .baz();

    // Long element no args
    let foo =
        bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
             .foo()
             .bar()
             .baz();

    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
         .foo()
         .bar()
         .baz();

    // Long element with args that fit
    let bar =
        bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
             .foo()
             .bar()
             .baz();

    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
         .foo()
         .bar()
         .baz();
}
