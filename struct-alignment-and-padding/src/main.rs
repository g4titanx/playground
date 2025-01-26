/// an impl of the structure alignment and padding rules
/// defined here: https://elric.pl/blog/struct-padding
#[derive(Debug, Clone)]
struct TypeInfo {
    size: usize,
    alignment: usize,
}

#[derive(Debug)]
struct StructLayout {
    member_offsets: Vec<usize>,
    paddings: Vec<usize>,
    total_size: usize,
    alignment: usize,
}

fn pad(x: usize, alignment: usize) -> usize {
    ((x + alignment - 1) / alignment) * alignment
}

impl StructLayout {
    fn compute(members: &[TypeInfo]) -> Self {
        if members.is_empty() {
            return StructLayout {
                member_offsets: vec![],
                paddings: vec![],
                total_size: 0,
                alignment: 1,
            };
        }

        let mut offsets = Vec::with_capacity(members.len());
        let mut paddings = Vec::with_capacity(members.len() - 1);
        let _current_offset = 0;

        // First member starts at offset 0
        offsets.push(0);

        // Calculate offsets and paddings between members
        for (i, member) in members.iter().enumerate().skip(1) {
            let previous_end = offsets[i - 1] + members[i - 1].size;
            let aligned_offset = pad(previous_end, member.alignment);
            
            paddings.push(aligned_offset - previous_end);
            offsets.push(aligned_offset);
        }

        // Calculate total size with final padding
        let last_member_end = offsets.last().unwrap() + members.last().unwrap().size;
        let struct_alignment = members.iter().map(|t| t.alignment).max().unwrap();
        let total_size = pad(last_member_end, struct_alignment);

        StructLayout {
            member_offsets: offsets,
            paddings,
            total_size,
            alignment: struct_alignment,
        }
    }
}

fn main() {
    let members = vec![
        TypeInfo { size: 4, alignment: 4 },  // T1
        TypeInfo { size: 2, alignment: 2 },  // T2
        TypeInfo { size: 8, alignment: 8 },  // T3
    ];

    let layout = StructLayout::compute(&members);
    
    println!("Struct Layout:");
    println!("Alignment: {}", layout.alignment);
    println!("Total size: {}", layout.total_size);
    
    for (i, offset) in layout.member_offsets.iter().enumerate() {
        println!("Member {}: offset={}, size={}", i + 1, offset, members[i].size);
        if i < layout.paddings.len() {
            println!("  Padding after: {}", layout.paddings[i]);
        }
    }
}