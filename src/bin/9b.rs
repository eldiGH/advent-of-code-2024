use std::vec;

use advent_of_code_2024::helpers::file::read_whole;

struct File {
    size: u32,
    id: u32,
}

struct FreeSpace {
    size: u32,
}

enum DiskEntry {
    File(File),
    FreeSpace(FreeSpace),
}

fn log(disk: &Vec<DiskEntry>) {
    for entry in disk {
        match entry {
            DiskEntry::File(file) => {
                for _ in 0..file.size {
                    print!("{}", file.id);
                }
            }
            DiskEntry::FreeSpace(free_space) => {
                for _ in 0..free_space.size {
                    print!(".");
                }
            }
        };
    }

    print!("\n");
}

fn calculate_file_checksum(position: u32, file: &File) -> u64 {
    let mut sum: u64 = 0;

    for p in position..position + file.size {
        sum += (p * file.id) as u64;
    }

    return sum;
}

fn main() {
    let map = read_whole("./inputs/9.txt");

    let mut disk: Vec<DiskEntry> = vec![];

    let mut is_empty = false;
    let mut id = 0;

    for block in map.chars() {
        let size = block.to_digit(10).unwrap();

        if size == 0 {
            is_empty = !is_empty;
            continue;
        }

        if is_empty {
            disk.push(DiskEntry::FreeSpace(FreeSpace { size }));
        } else {
            disk.push(DiskEntry::File(File { size, id }))
        }

        if !is_empty {
            id += 1;
        }
        is_empty = !is_empty;
    }

    let mut i: i32 = disk.len() as i32 - 1;
    while i >= 0 {
        let index = i as usize;

        let file_size = match &disk[index] {
            DiskEntry::File(file) => file.size,
            DiskEntry::FreeSpace(_) => {
                i -= 1;
                continue;
            }
        };

        for empty_space_index in 0..index {
            let free_space = match &disk[empty_space_index] {
                DiskEntry::File(_) => continue,
                DiskEntry::FreeSpace(free_space) => free_space.size,
            };

            if free_space == file_size {
                disk.swap(index, empty_space_index);
                break;
            } else if free_space > file_size {
                disk.swap(index, empty_space_index);

                if let DiskEntry::FreeSpace(free_space) = &mut disk[index] {
                    free_space.size = file_size;
                }

                disk.insert(
                    empty_space_index + 1,
                    DiskEntry::FreeSpace(FreeSpace {
                        size: free_space - file_size,
                    }),
                );
                i += 1;
                break;
            }
        }

        i -= 1;
    }

    let mut sum: u64 = 0;

    let mut position = 0;
    for entry in disk {
        let file = match entry {
            DiskEntry::FreeSpace(free_space) => {
                position += free_space.size;
                continue;
            }
            DiskEntry::File(file) => file,
        };

        sum += calculate_file_checksum(position, &file);
        position += file.size;
    }

    println!("{}", sum);
}
