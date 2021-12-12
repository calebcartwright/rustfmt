// rustfmt-indent_style: Visual
// rustfmt-chain_style: StandardWithOverflow

fn comments() {
    foo.z // foo
                // comment after parent
    .x.y// comment 1
                .bar("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk") // comment after bar()
        // comment 2
        .foobar
           // comment after
           // comment 3
           .baz(x, y, z);
    }