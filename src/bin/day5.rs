use std::ops::Range;

fn main() {
    let content = String::from_utf8(std::fs::read("inputs/day5.txt").unwrap()).unwrap();
    part1(&content);
    part2(&content);
}

#[derive(Debug)]
struct PartialMap {
    offset: isize,
    src_range: Range<usize>,
}

#[derive(Debug)]
struct Map(Vec<PartialMap>);

fn parse_maps<'a>(mut lines: impl Iterator<Item = &'a str>) -> Vec<Map> {
    let mut maps = Vec::<Map>::new();
    loop {
        let mut partial_maps = Vec::<PartialMap>::new();
        // map name
        if lines.next().is_none() {
            break;
        }
        let mut cur = lines.next().unwrap();
        while !cur.is_empty() {
            let mut cur_iter = cur.split_ascii_whitespace();
            let dest_start = cur_iter.next().unwrap().parse::<usize>().unwrap();
            let src_start = cur_iter.next().unwrap().parse::<usize>().unwrap();
            let offset: isize = dest_start as isize - src_start as isize;
            let range: usize = cur_iter.next().unwrap().parse().unwrap();

            partial_maps.push(PartialMap {
                offset,
                src_range: src_start..src_start + range,
            });

            match lines.next() {
                Some(line) => cur = line,
                None => break,
            }
        }
        maps.push(Map(partial_maps));
    }
    maps
}

fn part2(content: &str) {
    let mut lines = content.lines();
    let seeds_line = lines.next().unwrap();
    let mut seeds_iter = seeds_line.split_ascii_whitespace();
    seeds_iter.next().unwrap();

    let mut seeds: Vec<Range<usize>> = Vec::new();
    for s in seeds_iter.collect::<Vec<&str>>().chunks_exact(2) {
        let seed_start = s[0].parse::<usize>().unwrap();
        let len = s[1].parse::<usize>().unwrap();
        seeds.push(seed_start..seed_start + len);
    }
    lines.next().unwrap(); // empty

    let maps = parse_maps(lines);
    let mut min_loc = isize::MAX;
    'outer: for loc in 0..isize::MAX {
        let inital = loc;

        let mut seed_rev = loc;
        for map in maps.iter().rev() {
            for partial_map in &map.0 {
                let s = partial_map.src_range.start as isize + partial_map.offset;
                let e = partial_map.src_range.end as isize + partial_map.offset;
                let rev_range = s as usize..e as usize;

                if rev_range.contains(&(seed_rev as usize)) {
                    seed_rev = seed_rev - partial_map.offset;
                    break;
                }
            }
        }

        for seed in &seeds {
            if seed.contains(&(seed_rev as usize)) {
                min_loc = inital;
                break 'outer;
            }
        }
    }
    assert!(min_loc != isize::MAX);
    println!("[PART2] min = {}", min_loc)
}

fn part1(content: &str) {
    let mut lines = content.lines();
    let seeds_line = lines.next().unwrap();
    let mut seeds_iter = seeds_line.split_ascii_whitespace();
    seeds_iter.next().unwrap();

    let seeds: Vec<usize> = seeds_iter.map(|s| s.parse::<usize>().unwrap()).collect();
    lines.next().unwrap(); // empty

    let maps = parse_maps(lines);

    let mut min_loc = isize::MAX;
    for seed in seeds.iter() {
        let mut loc = *seed as isize;
        for map in &maps {
            for partial_map in &map.0 {
                if partial_map.src_range.contains(&(loc as usize)) {
                    loc = loc + partial_map.offset;
                    break;
                }
            }
        }
        min_loc = min_loc.min(loc);
    }

    println!("[PART1] min = {min_loc}");
}
