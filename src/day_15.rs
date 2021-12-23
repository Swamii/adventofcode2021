use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
    risk: u8,
}

#[derive(Debug, Hash, Clone, Eq)]
struct Visit {
    position: Position,
    distance: usize,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

fn get_adjacent_coordinates(coord: &Position, map: &Vec<Vec<Position>>) -> Vec<Position> {
    let mut adjacent: Vec<Position> = Vec::new();
    if coord.y > 0 {
        adjacent.push(Position {
            y: coord.y - 1,
            x: coord.x,
            risk: map[coord.y - 1][coord.x].risk,
        });
    }
    if coord.y < map.len() - 1 {
        adjacent.push(Position {
            y: coord.y + 1,
            x: coord.x,
            risk: map[coord.y + 1][coord.x].risk,
        });
    }
    if coord.x > 0 {
        adjacent.push(Position {
            y: coord.y,
            x: coord.x - 1,
            risk: map[coord.y][coord.x - 1].risk,
        });
    }
    if coord.x < map[0].len() - 1 {
        adjacent.push(Position {
            y: coord.y,
            x: coord.x + 1,
            risk: map[coord.y][coord.x + 1].risk,
        });
    }

    adjacent.sort_by_key(|pos| pos.risk);
    return adjacent;
}

fn make_graph(content: &str) -> (HashMap<Position, Vec<Position>>, Vec<Vec<Position>>) {
    let mut map: Vec<Vec<Position>> = Vec::new();
    for (y, row) in content.split("\n").enumerate() {
        let mut row_vals = Vec::new();
        for (x, column) in row.chars().enumerate() {
            row_vals.push(Position {
                x,
                y,
                risk: column.to_string().parse::<u8>().unwrap(),
            });
        }
        map.push(row_vals);
    }

    let mut graph: HashMap<Position, Vec<Position>> = HashMap::new();
    for row in &map {
        for pos in row {
            let adj = get_adjacent_coordinates(pos, &map);
            graph.insert(pos.clone(), adj);
        }
    }
    return (graph, map);
}

fn dijkstras(contents: &str) -> usize {
    let (graph, map) = make_graph(contents);
    let mut todo = BinaryHeap::new();
    todo.push(Visit {
        position: map[0][0].clone(),
        distance: 0,
    });
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    while let Some(Visit { position, distance }) = todo.pop() {
        if !visited.insert(position.clone()) {
            continue;
        }
        if let Some(neighbours) = graph.get(&position) {
            for neighbour in neighbours {
                let new_distance = distance + neighbour.risk as usize;
                let is_shorter = distances
                    .get(&neighbour)
                    .map_or(true, |&current| new_distance < current);
                if is_shorter {
                    distances.insert(neighbour, new_distance);
                    todo.push(Visit {
                        position: neighbour.clone(),
                        distance: new_distance,
                    })
                }
            }
        }
    }
    let end = &map[map.len() - 1][map[0].len() - 1];
    return *distances.get(&end).unwrap_or(&0);
}

fn expand(contents: &str) -> String {
    let mut blocks: Vec<Vec<Vec<String>>> = vec![vec![contents
        .split("\n")
        .map(|line| line.to_string())
        .collect()]];
    for y in 0..5 {
        for x in 0..4 {
            let to_copy = &blocks[y][x];
            let mut new_lines = Vec::new();
            for line in to_copy {
                let mut new_line = String::new();
                for char in line.chars() {
                    if let Ok(digit) = char.to_string().parse::<u8>() {
                        if digit == 9 {
                            new_line.push('1');
                        } else {
                            new_line.push_str(&(digit + 1).to_string());
                        }
                    }
                }
                new_lines.push(new_line);
            }
            if y == 0 && x <= 3 {
                blocks.push(vec![new_lines.clone()]);
            }
            if x <= 3 {
                blocks[y].push(new_lines);
            }
        }
    }
    let mut result = vec![String::new(); blocks[0][0].len() * 5];
    for block in blocks {
        for (j, lines) in block.iter().enumerate() {
            for (k, line) in lines.iter().enumerate() {
                result[k + j * line.len()] += line;
            }
        }
    }
    return result.join("\n");
}

pub fn run() {
    let contents = crate::utils::read_input("day15.txt");
    println!("Part1: {}", dijkstras(&contents));
    println!("Part2: {}", dijkstras(&expand(&contents)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = &"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
    ";
    const EXPANDED_INPUT: &'static str = &"
11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479 
    ";
    const EXPECTED_PART1: usize = 40;
    const EXPECTED_PART2: usize = 315;

    #[test]
    fn test_day_15_part1() {
        assert_eq!(dijkstras(&INPUT.trim()), EXPECTED_PART1);
    }

    #[test]
    fn test_day_15_part2_expand() {
        assert_eq!(expand(&INPUT.trim()), EXPANDED_INPUT.trim());
    }

    #[test]
    fn test_day_15_part2() {
        assert_eq!(dijkstras(&EXPANDED_INPUT.trim()), EXPECTED_PART2);
    }
}
