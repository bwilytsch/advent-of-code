use anyhow::Result;

pub fn part_two(input: &str) -> Result<u64> {
    // Read the puzzle input from stdin.
    // Replace this line with a hard-coded string if desired.
    let disk_map = input.trim();

    // Parse the dense disk map into a sequence of (file_length, free_length, ...)
    // We assume the format always starts with a file length as per the puzzle description.
    let digits: Vec<u8> = disk_map.bytes().map(|b| b - b'0').collect();

    // We'll convert this to a structure: a sequence of alternating segments
    // The pattern: file_length, free_length, file_length, free_length, ...
    let mut segments = Vec::new();
    {
        // segments will be a vector of (length, is_file), starting from a file.
        let mut is_file = true;
        for &d in &digits {
            segments.push((d as usize, is_file));
            is_file = !is_file;
        }
    }

    // Now build the block-level representation.
    // We'll assign file IDs in the order they appear.
    let mut blocks: Vec<Option<usize>> = Vec::new(); // None = free, Some(file_id) = file block
    let mut file_id = 0;
    let mut file_starts = Vec::new(); // store start indices for each file
    let mut file_lengths = Vec::new(); // store length for each file

    for (length, is_file) in &segments {
        if *is_file {
            // add `length` blocks with file_id
            file_starts.push(blocks.len());
            file_lengths.push(*length);
            for _ in 0..*length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // add `length` free blocks
            for _ in 0..*length {
                blocks.push(None);
            }
        }
    }

    // Now we have a block-level representation of files and free spaces.
    // Part Two Compaction:
    // For each file in decreasing order of file ID:
    // Move the entire file to the leftmost free segment (before the file's start)
    // that can fit the file, if possible.

    // We'll re-locate files by checking free gaps to the left of the file.
    // A "free gap" is a contiguous run of None blocks.
    //
    // Strategy:
    // - For each file in descending order of file_id:
    //   1. Find its current position (start index and length).
    //   2. Look to the left (from the start of the disk up to the start of this file)
    //      for a free segment large enough to hold the entire file.
    //   3. If found, move the file there:
    //      - That means setting those blocks to Some(file_id)
    //      - Setting old file positions to None
    //   4. Update the file's start position in our metadata.

    // We'll keep track of file start and length in arrays that we update after moves.
    // This way, we don't have to re-derive them from scratch.
    let num_files = file_id;
    let mut file_positions: Vec<(usize, usize)> = file_starts
        .into_iter()
        .zip(file_lengths.into_iter())
        .collect();

    // Function to find a free segment of a given length to the left of a certain index
    fn find_free_segment(
        blocks: &Vec<Option<usize>>,
        end_index: usize,
        needed: usize,
    ) -> Option<usize> {
        // We'll scan from left to right up to end_index-1 and find the first free segment of length >= needed
        if needed == 0 {
            return None; // no need to move empty files (though problem states files have length)
        }

        let mut start = None;
        let mut count = 0;
        for i in 0..end_index {
            if blocks[i].is_none() {
                if start.is_none() {
                    start = Some(i);
                    count = 1;
                } else {
                    count += 1;
                }

                if count >= needed {
                    // Found a big enough segment
                    // The segment starts at `start.unwrap()` and length is at least `needed`.
                    return start;
                }
            } else {
                // reset search
                start = None;
                count = 0;
            }
        }

        None
    }

    // Move files from highest ID to lowest
    for current_id in (0..num_files).rev() {
        let (start_idx, length) = file_positions[current_id];
        if length == 0 {
            continue;
        }
        // Find a free segment to the left
        if let Some(new_start) = find_free_segment(&blocks, start_idx, length) {
            // Move file
            // Clear old positions
            for i in start_idx..(start_idx + length) {
                blocks[i] = None;
            }
            // Place file in the found segment
            for i in new_start..(new_start + length) {
                blocks[i] = Some(current_id);
            }
            // Update file position
            file_positions[current_id] = (new_start, length);
        }
    }

    // Compute checksum
    let mut checksum: u64 = 0;
    for (i, b) in blocks.iter().enumerate() {
        if let Some(fid) = b {
            checksum += (i as u64) * (*fid as u64);
        }
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/example.txt") {
            assert_eq!(part_two(&input)?, 2858);
        };

        // 6301361958738 -- Correct
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
