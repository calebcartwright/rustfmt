// rustfmt-indent_style: Visual
// rustfmt-chain_style: StandardWithOverflow

fn long_tail() {
    // Args that do not fit
    let bar =
        foo().foo_bar
             .foo()
             .bar()
             .baz()
             .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf");

    foo().foo_bar
         .foo()
         .bar()
         .baz()
         .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf");

    // Log element no args
    let foo =
        bar().foo_bar
             .foo()
             .bar()
             .baz()
             .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff();

    qux().foo_bar
         .foo()
         .bar()
         .baz()
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff();

    // Long element with args that fit
    bar().xxxxxxx
         .map(|x| x + 5)
         .map(|x| x / 2)
         .fold(0, |acc, x| acc + x)
         .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");

    let foo =
        bar().xxxxxxx
             .map(|x| x + 5)
             .map(|x| x / 2)
             .fold(0, |acc, x| acc + x)
             .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");
}
