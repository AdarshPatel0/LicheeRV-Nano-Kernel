use crate::{context};

pub fn call(context: &mut context::Context) {
    context.sepc = context.sepc + 4;
    let code = context.a[7];
    match code {
        _ => {
            return;
        }
    }
}