use std::{collections::{HashMap, HashSet}, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY09 -------");
    let example = fs::read_to_string("inputs/example_day09").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");
    let input = parse(&input);

    day09_part1(&example, &input);
    day09_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<(usize, usize)> {
    let mut red_tiles = vec![];
    for line in raw_input.lines() {
        let (j, i) = line.split_once(",").unwrap();
        red_tiles.push((i.parse().unwrap(), j.parse().unwrap()));
    }
    red_tiles
}

fn rectangle_area(tile1: (usize, usize), tile2: (usize, usize)) -> usize {
    (tile1.0.max(tile2.0) - tile1.0.min(tile2.0) + 1)
        * (tile1.1.max(tile2.1) - tile1.1.min(tile2.1) + 1)
}

fn find_largest_rectangle_area(red_tiles: &[(usize, usize)]) -> usize {
    let mut max_area = 0;
    for (i, &tile1) in red_tiles.iter().enumerate() {
        for &tile2 in red_tiles[i + 1..].iter() {
            let area = rectangle_area(tile1, tile2);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn day09_part1(example: &[(usize, usize)], input: &[(usize, usize)]) {
    // Exemple tests
    assert_eq!(rectangle_area((2, 5), (9, 7)), 24);
    assert_eq!(rectangle_area((9, 7), (2, 5)), 24);
    assert_eq!(rectangle_area((7, 1), (11, 7)), 35);
    assert_eq!(rectangle_area((11, 7), (7, 1)), 35);
    assert_eq!(rectangle_area((7, 3), (2, 3)), 6);
    assert_eq!(rectangle_area((2, 3), (7, 3)), 6);
    assert_eq!(rectangle_area((2, 5), (11, 1)), 50);
    assert_eq!(rectangle_area((11, 1), (2, 5)), 50);
    assert_eq!(find_largest_rectangle_area(example), 50);

    // Solve puzzle
    let res = find_largest_rectangle_area(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 4744899849);
    println!("> DAY09 - part 1: OK!");
}

fn intersect(
    rectangle: ((usize, usize), (usize, usize)),
    edge: ((usize, usize), (usize, usize)),
) -> bool {
    let rectangle_c1 = rectangle.0;
    let rectangle_c2 = rectangle.1;
    let edge_t1 = edge.0;
    let edge_t2 = edge.1;

    if edge_t1.0 == edge_t2.0 {
        // Same i, so horizontal edge
        let ei = edge_t1.0;
        let min_ri = rectangle_c1.0.min(rectangle_c2.0);
        let max_ri = rectangle_c1.0.max(rectangle_c2.0);
        if ei <= min_ri || ei >= max_ri {
            return false;
        }

        // min_ri < ei < max_ri:
        let min_ej = edge_t1.1.min(edge_t2.1);
        let max_ej = edge_t1.1.max(edge_t2.1);
        let min_rj = rectangle_c1.1.min(rectangle_c2.1);
        let max_rj = rectangle_c1.1.max(rectangle_c2.1);
        if max_ej <= min_rj || max_rj <= min_ej {
            return false;
        }

        // Intersect!
    } else if edge_t1.1 == edge_t2.1 {
        // Same j, so vertical edge
        let ej = edge_t1.1;
        let min_rj = rectangle_c1.1.min(rectangle_c2.1);
        let max_rj = rectangle_c1.1.max(rectangle_c2.1);
        if ej <= min_rj || ej >= max_rj {
            return false;
        }

        // min_rj < ej < max_rj:
        let min_ei = edge_t1.0.min(edge_t2.0);
        let max_ei = edge_t1.0.max(edge_t2.0);
        let min_ri = rectangle_c1.0.min(rectangle_c2.0);
        let max_ri = rectangle_c1.0.max(rectangle_c2.0);
        if max_ei <= min_ri || max_ri <= min_ei {
            return false;
        }

        // Intersect!
    } else {
        unreachable!("Edge {edge:?} should be horizontal or vertical!");
    }
    true
}

fn find_largest_rectangle_area_inside(red_tiles: &[(usize, usize)]) -> usize {
    // Get contour's edges as list by decreasing size
    let nb_red_tiles = red_tiles.len();
    let mut edges_with_sizes = vec![];
    let mut frontier: HashMap<usize, Vec<usize>> = HashMap::new(); // i -> [j1, j2,… ,jn]
    for index in 0..nb_red_tiles {
        let tile1 = red_tiles[index];
        let tile2 = red_tiles[(index + 1) % nb_red_tiles];
        let edge_len;
        if tile1.0 == tile2.0 {
            // Same i
            let ei = tile1.0;
            let min_ej = tile1.1.min(tile2.1);
            let max_ej = tile1.1.max(tile2.1);
            edge_len = max_ej - min_ej + 1;
            for j in min_ej..=max_ej {
                let l = frontier.entry(ei).or_default();
                l.push(j);
            }
        } else if tile1.1 == tile2.1 {
            // Same j
            edge_len = tile1.0.max(tile2.0) - tile1.0.min(tile2.0) + 1;
        } else {
            unreachable!("{tile1:?} and {tile2:?} should have one similar coordinate!");
        }
        edges_with_sizes.push((edge_len, (tile1, tile2)));
    }
    edges_with_sizes.sort_by(|(size1, _), (size2, _)| size2.cmp(size1));
    // let nb_edges = edges_with_sizes.len();
    // println!("{nb_edges} edges collected");
    // println!(
    //     "Largest edge is {:?}, with len {}",
    //     edges_with_sizes[0].1, edges_with_sizes[0].0
    // );
    // println!(
    //     "Smallest edge is {:?}, with len {}\n",
    //     edges_with_sizes[nb_edges - 1].1,
    //     edges_with_sizes[nb_edges - 1].0
    // );

    // Get all rectangles as list by decreasing area
    let mut rectangles_with_area = vec![];
    for a in 0..nb_red_tiles {
        let tile1 = red_tiles[a];
        for b in (a + 1)..nb_red_tiles {
            let tile2 = red_tiles[b];
            let area = rectangle_area(tile1, tile2);
            rectangles_with_area.push((area, (tile1, tile2)));
        }
    }
    rectangles_with_area.sort_by(|(area1, _), (area2, _)| area2.cmp(area1));
    // let nb_rectangles = rectangles_with_area.len();
    // println!("{nb_rectangles} rectangles collected");
    // println!(
    //     "Biggest rectangle is {:?}, with area {}",
    //     rectangles_with_area[0].1, rectangles_with_area[0].0
    // );
    // println!(
    //     "Smallest rectangle is {:?}, with area {}\n",
    //     rectangles_with_area[nb_rectangles - 1].1,
    //     rectangles_with_area[nb_rectangles - 1].0
    // );

    // Now remove all rectangles that intersect an edge
    let mut rectangles_indices_to_remove = HashSet::new();
    for (_, edge) in edges_with_sizes {
        for (index, (_, rectangle)) in rectangles_with_area.iter().enumerate() {
            if intersect(*rectangle, edge) {
                rectangles_indices_to_remove.insert(index);
            }
        }
    }
    let mut rectangles_not_intersecting_with_area = vec![];
    for (i, r) in rectangles_with_area.iter().enumerate() {
        if !rectangles_indices_to_remove.contains(&i) {
            rectangles_not_intersecting_with_area.push(*r);
        }
    }
    // println!("After removing rectangles that intersect with an edge:");
    // let nb_rectangles = rectangles_not_intersecting_with_area.len();
    // println!("{nb_rectangles} rectangles remaining");
    // println!(
    //     "Biggest rectangle is {:?}, with area {}",
    //     rectangles_not_intersecting_with_area[0].1, rectangles_not_intersecting_with_area[0].0
    // );
    // println!(
    //     "Smallest rectangle is {:?}, with area {}\n",
    //     rectangles_not_intersecting_with_area[nb_rectangles - 1].1,
    //     rectangles_not_intersecting_with_area[nb_rectangles - 1].0
    // );

    // And I don't know why but the biggest is the good one…
    return rectangles_not_intersecting_with_area[0].0;

    /*
    // Now check each rectangle from the biggest to the smallest
    'main_loop: for (area, rectangle) in rectangles_not_intersecting_with_area {
        let rectangle_c1 = rectangle.0;
        let rectangle_c2 = rectangle.0;
        let min_ri = rectangle_c1.0.min(rectangle_c2.0);
        let max_ri = rectangle_c1.0.max(rectangle_c2.0);
        let min_rj = rectangle_c1.1.min(rectangle_c2.1);
        let max_rj = rectangle_c1.1.max(rectangle_c2.1);
        
        for i in min_ri..=max_ri {
            for j in min_rj..=max_rj {
                // We make assumptions about the regularity of the frontier…
                if frontier.contains_key(&i) {
                    let columns_for_j = frontier.get(&i).unwrap();
                    let mut before = false;
                    let mut after = false;
                    for &fj in columns_for_j {
                        if fj <= j {
                            before = true;
                        } else if fj >= j {
                            after = true;
                        }
                    }
                    if !after || !before {
                        // Element (i, j) of rectangle is outside the frontier!
                        // Rectangle is incorrect
                        continue 'main_loop;
                    }
                }
            }
        }
        // All elements are in the frontier, we fount THE ONE!
        return area;
    }
    unreachable!("No rectangle found");
    */
}

fn day09_part2(example: &[(usize, usize)], input: &[(usize, usize)]) {
    // Exemple tests
    assert_eq!(find_largest_rectangle_area_inside(example), 24);

    // Solve puzzle
    let res = find_largest_rectangle_area_inside(input);
    println!("Result part 2: {res}"); // 1351663290 too low
    assert_eq!(res, 1540192500);
    println!("> DAY09 - part 2: OK!");
}
