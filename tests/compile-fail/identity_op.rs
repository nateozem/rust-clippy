#![feature(plugin)]
#![plugin(clippy)]

const ONE : i64 = 1;
const NEG_ONE : i64 = -1;
const ZERO : i64 = 0;

#[deny(identity_op)]
fn main() {
    let x = 0;

    x + 0;        //~ERROR the operation is ineffective
    0 + x;        //~ERROR the operation is ineffective
    x - ZERO;     //~ERROR the operation is ineffective
    x | (0);      //~ERROR the operation is ineffective
    ((ZERO)) | x; //~ERROR the operation is ineffective

    x * 1;        //~ERROR the operation is ineffective
    1 * x;        //~ERROR the operation is ineffective
    x / ONE;      //~ERROR the operation is ineffective

    x & NEG_ONE;  //~ERROR the operation is ineffective
    -1 & x;       //~ERROR the operation is ineffective
}