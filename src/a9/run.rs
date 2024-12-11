
use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct File {
    position: u64,
    id: Option<u64>,
    length: u64,
}

impl File {
    fn new(id: Option<u64>, position: u64, length: u64) -> Self {
        File {
            position, id, length
        }
    }

    fn checksum(&self) -> u64 {
        match self.id {
            None => { 0 }
            Some(i) => {
                let pos_end = self.position + self.length - 1;
                let checksum = i *  (self.position + pos_end) * self.length / 2;
                checksum
            }
        }

    }
}


fn main() {

    let line: Vec<char> = include_str!("input.txt").chars().collect();

    //println!("{}", line);
    let mut files: Vec<File> = Vec::new();
    let mut pos = 0u64;
    for i in 0.. line.len() {
        let len: u64 = line[i].to_string().parse().unwrap();
        if i & 1 == 0 {
            files.push(File::new(Some(i as u64/2), pos, len));
        } else {
            //empty space
            files.push(File::new(None, pos, len));
        }
        pos += len;
    }

    println!("{:?}", &files);

    // part 1
    // loop {
    //     let (last_file, ff, is_done) = remove_last_file(&files);
    //     if is_done {
    //         println!("last {:?}", &files);
    //         calculate_checksum(&files);
    //
    //         break;
    //     }
    //     files = ff;
    //     let (_, ff) = merge_file(&last_file, &files);
    //     files = ff;
    //     //print(&files);
    // }

    // part 2
    let mut work_list = files.clone();
    'outer: loop {
        let files = work_list.clone();
        //print(&files);
        'big: for file in files.iter().rev() {
            if file.id.is_some() {
                //try merge
                for empty in files.iter() {
                    if empty == file {
                        //done with this file
                        continue 'big;
                    }
                    if empty.id.is_none() {
                        if empty.length >= file.length {
                            let mut copy: Vec<File> = Vec::new();
                            //copy merge
                            for f in files.iter() {
                                if f == empty {
                                    let mut fclone = file.clone();
                                    fclone.position = empty.position;
                                    copy.push(fclone);
                                    if empty.length > file.length {
                                        copy.push(File::new(None, empty.position + file.length, empty.length - file.length));
                                    }
                                } else  if f == file {
                                    //insert space where the original file was
                                    copy.push(File::new(None, file.position, file.length));
                                } else {
                                    //copy all else
                                    copy.push(*f)
                                }
                            }
                            work_list = copy.clone();
                            continue 'outer;
                        }
                    }
                }


            }
        }
        break;

    }

    print(&work_list);
    calculate_checksum(&work_list);
    //16108706523781
    //17202538925546 //to high
    //15912048962023 //to high
    //6427424893268
    //6427437134372
}

fn print(file_list: &Vec<File>) {
    for file in file_list {
        match file.id {
            None => {
                for i in 0..file.length {
                    print!("{}", ".");
                }
            }
            Some(id) => {
                for i in 0..file.length {
                    print!("{}", id);
                }
            }
        }
    }
    println!();
}

fn calculate_checksum(file_list: &Vec<File>) {
    let mut checksum = 0u64;
    for file in file_list {
        checksum += file.checksum();
    }
    println!("checksum {}", checksum);
}

fn merge_file(o_file: &File, file_list: &Vec<File>) -> (bool, Vec<File>) {
    let mut new_list: Vec<File> = Vec::new();
    let mut no_move = false;
    let mut work_file_opt = Some(o_file.clone());
    for file in file_list {
        if work_file_opt.is_some() && file.id.is_none() {
            let work_file = work_file_opt.unwrap();
            //empty space
            if file.length >= work_file.length {
                let new_file = File::new(work_file.id, file.position, work_file.length);
                new_list.push(new_file);
                if file.length > work_file.length {
                    let new_empty = File::new(None, file.position + work_file.length, file.length - work_file.length);
                    new_list.push(new_empty);
                }
                //done
                work_file_opt = None;
            } else {
                let new_file = File::new(work_file.id, file.position, file.length);
                new_list.push(new_file);
                work_file_opt = Some(File::new(work_file.id, 0, work_file.length - file.length));
            }
        } else {
            new_list.push(file.clone());
        }

    }
    (no_move, new_list)
}

fn remove_last_file(file_list: &Vec<File>) -> (File, Vec<File>, bool) {
    let (i, last_file) = file_list.iter().enumerate().rfind(|(index, f)| f.id.is_some()).unwrap();
    let (first_space_index, first_space) = file_list.iter().enumerate().find(|(index, f)| f.id.is_none()).unwrap();


    let mut list_clone = (*file_list).clone();
    list_clone.remove(i);
    ((*last_file).clone(), list_clone, i < first_space_index)
}