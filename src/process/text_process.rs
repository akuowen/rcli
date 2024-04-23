use anyhow::Result;

use crate::TextOps;

pub fn process_text(text_opts: &TextOps) -> Result<()> {
    match text_opts {
        TextOps::Sign(text_sign_opts) => {
            print!("{:?}", text_sign_opts);
            todo!()
        }
        TextOps::Verify(text_verify_opts) => {
            print!("{:?}", text_verify_opts);
            todo!()
        }
    }
}
