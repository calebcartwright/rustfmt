// rustfmt-indent_style: Visual
// rustfmt-chain_style: StandardWithOverflow

fn long_inner_child() {
    // Args that do not fit
    let bar =
        foo().foo_bar
             .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
             .foo()
             .bar()
             .baz();

    foo().foo_bar
         .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
         .foo()
         .bar()
         .baz();

    // Long element no args
    let foo =
        bar().foo_bar
             .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
             .foo()
             .bar()
             .baz();

    qux().foo_bar
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
         .foo()
         .bar()
         .baz();

    // Long element with args that fit
    let bar =
        bar().foo_bar
             .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
             .foo()
             .bar()
             .baz();

    qux().foo_bar
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
         .foo()
         .bar()
         .baz();
}
