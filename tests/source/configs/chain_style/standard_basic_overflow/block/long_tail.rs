// rustfmt-indent_style: Block
// rustfmt-chain_style: StandardWithOverflow


fn long_tail() {
     // Args that do not fit
     bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
 
     let foo = bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
 
     // Long element with args that fit
     bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abc123def456");
 
     let foo = bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abc123def456");
 
     // Long element no args
     bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff();
 
     let foo = bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff();
 }