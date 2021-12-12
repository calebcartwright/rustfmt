// rustfmt-indent_style: Block
// rustfmt-chain_style: StandardWithOverflow

fn long_inner() {
     // Args that do not fit
     let bar = bar().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf").foo().bar().baz();
 
     qux().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf").foo().bar().baz();
 
     // Long element with args that fit
     let bar = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("ffffffffffffffffffffffffffffffffffff").foo().bar().baz();
 
     qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("ffffffffffffffffffffffffffffffffffff").foo().bar().baz();
 
     // Long element no args
     let foo = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();
 
     qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();
 }
