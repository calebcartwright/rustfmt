// rustfmt-indent_style: Block
// rustfmt-chain_style: StandardWithOverflow

fn long_parent() {
    // Args that do not fit
    let bar =
        baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
            .foo()
            .bar()
            .baz();

    baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    // Long element no args
    let foo =
        looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg()
            .foo()
            .bar()
            .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
        .foo()
        .bar()
        .baz();

    // Long element with args that fit
    let bar =
        looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg("ffffffffffffffffffffffffffffffffffff")
            .foo()
            .bar()
            .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("ffffffffffffffffffffffffffffffffffff")
        .foo()
        .bar()
        .baz();
}
