use crate::types::instruction::Instruction;

use super::feedback_buffer::FeedbackBuffer;
use super::processing_plan::ProcessingPlan;

pub fn create_instruction_array(
    plan: &ProcessingPlan,
    feedback_buffers: &[FeedbackBuffer],
) -> (Vec<Instruction>, u32) {
    let mut next_buf = plan.next_buf;
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::FetchInput {
        target_buf_idx: plan.graph_input_buf,
    });

    let fb_inject_bufs: Vec<u32> = feedback_buffers
        .iter()
        .enumerate()
        .map(|(reg_idx, _)| {
            let buf = next_buf;
            next_buf += 1;

            instructions.push(Instruction::FetchFeedback {
                reg_idx: reg_idx as u32,
                target_buf_idx: buf,
            });

            buf
        })
        .collect();

    for (wasm_instance_idx, &node_id) in plan.topo_order.iter().enumerate() {
        let out_buf = plan.node_out_buf[&node_id];
        let sources = &plan.reg_inputs[&node_id];
        let feedbacks = &plan.fb_inputs[&node_id];

        let input_buf = match (sources.as_slice(), feedbacks.as_slice()) {
            ([], []) => plan.graph_input_buf,
            ([single], []) => plan.node_out_buf[single],
            ([], [single_fb]) => fb_inject_bufs[*single_fb],
            _ => {
                let mix_buf = next_buf;
                next_buf += 1;

                for &src in sources {
                    instructions.push(Instruction::Sum {
                        source_buf_idx: plan.node_out_buf[&src],
                        target_buf_idx: mix_buf,
                    });
                }

                for &reg_idx in feedbacks {
                    instructions.push(Instruction::Sum {
                        source_buf_idx: fb_inject_bufs[reg_idx],
                        target_buf_idx: mix_buf,
                    });
                }

                mix_buf
            }
        };

        instructions.push(Instruction::ExecuteTransform {
            wasm_instance_idx: wasm_instance_idx as u32,
            input_buf_idx: input_buf,
            output_buf_idx: out_buf,
            param_offset: 0,
        });
    }

    for (reg_idx, feedback_buffer) in feedback_buffers.iter().enumerate() {
        if let Some(&src_buf) = plan.node_out_buf.get(&feedback_buffer.from_id) {
            instructions.push(Instruction::StoreFeedback {
                source_buf_idx: src_buf,
                reg_idx: reg_idx as u32,
            });
        }
    }

    if let Some(&last_id) = plan.topo_order.last() {
        instructions.push(Instruction::WriteOutput {
            source_buf_idx: plan.node_out_buf[&last_id],
        });
    }

    (instructions, next_buf)
}
