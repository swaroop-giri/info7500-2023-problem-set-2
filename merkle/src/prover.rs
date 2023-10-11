use crate::util::{Hash32Bytes, write_merkle_proof,encode_hash,hash_internal, MerkleProof, hash_leaf};
use sha2::{Sha256, Digest};

fn gen_leaves_for_merkle_tree(num_leaves: usize) -> Vec<String> {
    let leaves: Vec<String> = (0..num_leaves)
        .map(|i| format!("data item {}", i))
        .collect();

    println!("\nI generated #{} leaves for a Merkle tree.", num_leaves);

    leaves
}
pub fn gen_merkle_proof(leaves: Vec<String>, leaf_pos: usize) -> Vec<Hash32Bytes> {
    let height = (leaves.len() as f64).log2().ceil() as u32;
    let padlen = (2u32.pow(height)) as usize - leaves.len();

    // hash all the leaves
    let mut state: Vec<Hash32Bytes> = leaves.into_iter().map(hash_leaf).collect();

    // Pad the list of hashed leaves to a power of two
    let zeros = [0u8; 32];
    for _ in 0..padlen {
        state.push(zeros);
    }

    // initialize a vector that will contain the hashes in the proof
    let mut hashes: Vec<Hash32Bytes> = vec![];

    let mut level_pos = leaf_pos;
    for _level in 0..height {
        let is_right_sibling = level_pos % 2 == 0; // Check if the current leaf is a right sibling
        let sibling_pos = if is_right_sibling {
            level_pos - 1
        } else {
            level_pos + 1
        };

        if sibling_pos < state.len() {
            // Append the sibling hash to the proof
            hashes.push(state[sibling_pos].clone());
        } else {
            // If there's no sibling, use a zero hash
            hashes.push(zeros);
        }

        // Function to hash two nodes

        fn hash_nodes(left: &Hash32Bytes, right: &Hash32Bytes) -> Hash32Bytes {
            let mut hasher = Sha256::new();
            hasher.update(left);
            hasher.update(right);
            let result = hasher.finalize();
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&result);
            hash
        }

        // Calculate the parent hash of the current leaf and sibling
        let parent_hash = if is_right_sibling {
            hash_nodes(&state[sibling_pos], &state[level_pos])
        } else {
            hash_nodes(&state[level_pos], &state[sibling_pos])
        };

        // Update the current position to the parent position for the next iteration
        level_pos = level_pos / 2;

        // Replace the current leaf hash with the parent hash
        state.push(parent_hash);
    }

    // Returns list of hashes that make up the Merkle Proof
    hashes
}

pub fn run(leaf_position: usize) {
    const NUM_LEAVES: usize = 1000; // replace with your actual constant

    let leaves = gen_leaves_for_merkle_tree(NUM_LEAVES);
    assert!(leaf_position < leaves.len());
    let leaf_value = leaves[leaf_position].clone();
    let hashes = gen_merkle_proof(leaves, leaf_position);

    let mut proof_hash_values_base64: Vec<String> = Vec::new();

    for hash in hashes {
        proof_hash_values_base64.push(encode_hash(hash))
    }

    let proof = MerkleProof{
        leaf_position,
        leaf_value,
        proof_hash_values_base64,
        proof_hash_values: None,
    };

    write_merkle_proof(&proof, "proof_gen.yaml")
}