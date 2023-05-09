use crate::{instruction::Instruction, Module};

pub fn run_local_store_load_forwarding(module: &mut Module) -> bool {
    let mut modified = false;
    for block in &mut module.blocks {
        let mut av_store = None;
        for instr in &mut block.body {
            match instr {
                &mut Instruction::Store {
                    pointer, ref value, ..
                } => av_store = Some((pointer, value.clone())),
                &mut Instruction::Load {
                    target, pointer, ..
                } => {
                    if let Some(store) = &av_store {
                        if store.0 == pointer {
                            *instr = Instruction::Set(target, store.1.clone());
                            modified = true;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    modified
}
