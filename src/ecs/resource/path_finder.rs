// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use dat;
use ecs::resource::{OccupiedTiles, Terrain};
use identifier::{TerrainId, UnitTerrainRestrictionId};
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use types::{Fixed, ToFixed, Vector3};

const PASSABILITY_THRESHOLD: f32 = 0.999;

pub type PathNode = Vector3;
pub type Path = Vec<PathNode>;

type TileNode = (i32, i32);
type TilePath = Vec<TileNode>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TilePathCandidate {
    path: TilePath,
    heuristic: i32,
    dist_from_target: i32,
    direction: (i32, i32),
}

impl TilePathCandidate {
    fn new(path: TilePath,
           heuristic: i32,
           dist_from_target: i32,
           direction: (i32, i32))
           -> TilePathCandidate {
        TilePathCandidate {
            path: path,
            heuristic: heuristic,
            dist_from_target: dist_from_target,
            direction: direction,
        }
    }
}

impl Ord for TilePathCandidate {
    fn cmp(&self, other: &TilePathCandidate) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for TilePathCandidate {
    fn partial_cmp(&self, other: &TilePathCandidate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Trait for faking out the Empires DB for tests
trait PassabilityProvider: Send + Sync {
    fn passable(&self, restriction_id: UnitTerrainRestrictionId, terrain_id: TerrainId) -> bool;
}

struct EmpiresPassabilityProvider {
    empires: dat::EmpiresDbRef,
}

impl EmpiresPassabilityProvider {
    pub fn new(empires: dat::EmpiresDbRef) -> EmpiresPassabilityProvider {
        EmpiresPassabilityProvider { empires: empires }
    }
}

impl PassabilityProvider for EmpiresPassabilityProvider {
    fn passable(&self, restriction_id: UnitTerrainRestrictionId, terrain_id: TerrainId) -> bool {
        let restrictions = self.empires.terrain_restrictions(restriction_id);
        restrictions.passability(terrain_id) >= PASSABILITY_THRESHOLD
    }
}

pub struct PathFinder {
    passability_provider: Box<PassabilityProvider>,
}

impl PathFinder {
    pub fn new(empires: dat::EmpiresDbRef) -> PathFinder {
        PathFinder { passability_provider: Box::new(EmpiresPassabilityProvider::new(empires)) }
    }

    #[cfg(test)]
    fn new_with(passability_provider: Box<PassabilityProvider>) -> PathFinder {
        PathFinder { passability_provider: passability_provider }
    }

    pub fn find_path(&self,
                     terrain: &Terrain,
                     occupied_tiles: &OccupiedTiles,
                     from: &Vector3,
                     to: &Vector3,
                     restriction_id: UnitTerrainRestrictionId)
                     -> Path {
        let from_tile: (i32, i32) = (from.y.into(), from.x.into());
        let to_tile: (i32, i32) = (to.y.into(), to.x.into());
        let tile_path = self.find_tile_path(terrain, occupied_tiles, from_tile, to_tile, restriction_id);

        let mut position_path: Vec<PathNode> = Vec::new();
        for tile_node in tile_path.iter().skip(1) {
            let tile = terrain.tile_at_row_col(tile_node.0, tile_node.1);
            let position = Vector3::new(tile_node.1.to_fixed() + fixed_const!(0.5),
                                        tile_node.0.to_fixed() + fixed_const!(0.5),
                                        tile.elevation.to_fixed());
            position_path.push(position);
        }
        if *tile_path.last().unwrap() == to_tile {
            position_path.pop(); // Remove the tile center for the last tile
            position_path.push(*to);
        }

        position_path
    }

    fn find_tile_path(&self,
                      terrain: &Terrain,
                      occupied_tiles: &OccupiedTiles,
                      from: TileNode,
                      to: TileNode,
                      restriction_id: UnitTerrainRestrictionId)
                      -> TilePath {
        let (width, height) = (terrain.width(), terrain.height());
        let (from, to) = (clamp(from, width, height), clamp(to, width, height));
        if from == to {
            return vec![to];
        }

        // For tracking the path that comes closest to the target in case it's not possible to reach the target
        let mut closest = {
            let distance = dist(&from, &to);
            TilePathCandidate::new(vec![from], 1 + distance, distance, (0, 0))
        };

        // For tracking nodes whose neighbors we've already pushed onto the queue
        let mut visited: HashSet<TileNode> = HashSet::new();
        visited.insert(from);

        let mut path_queue: BinaryHeap<TilePathCandidate> = BinaryHeap::new();
        path_queue.push(closest.clone());

        // Breadth-first search with priority queue and heuristic; also known as A*
        while !path_queue.is_empty() {
            let next = path_queue.pop().unwrap();
            let last_node = *next.path.last().unwrap();
            if last_node == to {
                return next.path;
            } else {
                // Setup future exploration of neighbors
                for neighbor in neighbors(&last_node, width, height).into_iter() {
                    let tile = terrain.tile_at_row_col(neighbor.0, neighbor.1);
                    if !visited.contains(neighbor) && !occupied_tiles.tiles.contains(neighbor) &&
                       self.passability_provider.passable(restriction_id, tile.terrain_id) {
                        let mut neighbor_path = next.path.clone();
                        neighbor_path.push(*neighbor);

                        let neighbor_direction = (neighbor.0 - last_node.0, neighbor.1 - last_node.1);
                        let neighbor_dist = dist(neighbor, &to);
                        let neighbor_heuristic = heuristic(&neighbor_path,
                                                           neighbor_dist,
                                                           next.direction != neighbor_direction);
                        let neighbor_candidate = TilePathCandidate::new(neighbor_path,
                                                                        neighbor_heuristic,
                                                                        neighbor_dist,
                                                                        neighbor_direction);
                        path_queue.push(neighbor_candidate);
                    }
                    visited.insert(*neighbor);
                }

                // Keep track of the closest path
                if closest.dist_from_target > next.dist_from_target {
                    closest = next;
                }
            }
        }

        closest.path
    }
}

fn neighbors(node: &TileNode, width: i32, height: i32) -> [TileNode; 8] {
    [clamp((node.0 - 1, node.1), width, height),
     clamp((node.0 + 1, node.1), width, height),
     clamp((node.0, node.1 - 1), width, height),
     clamp((node.0, node.1 + 1), width, height),
     clamp((node.0 - 1, node.1 - 1), width, height),
     clamp((node.0 - 1, node.1 + 1), width, height),
     clamp((node.0 + 1, node.1 - 1), width, height),
     clamp((node.0 + 1, node.1 + 1), width, height)]
}

fn heuristic(path: &TilePath, dist_to_goal: i32, direction_change: bool) -> i32 {
    let length = path.len() as i32;
    length + dist_to_goal + (direction_change as i32)
}

fn dist(from: &TileNode, to: &TileNode) -> i32 {
    // Chebyshev distance (from: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html)
    let rdist = (from.0 - to.0).abs();
    let cdist = (from.1 - to.1).abs();
    cmp::max(rdist, cdist)
}

fn clamp(node: TileNode, width: i32, height: i32) -> TileNode {
    (cmp::min(height, cmp::max(0, node.0)), cmp::min(width, cmp::max(0, node.1)))
}

#[cfg(test)]
mod tests {
    use dat::{EmpiresDb, EmpiresDbRef};
    use ecs::resource::{OccupiedTiles, Terrain, Tile};
    use identifier::{TerrainId, UnitTerrainRestrictionId};
    use super::*;
    use super::PassabilityProvider;

    struct FakePassabilityProvider {
        map: Vec<i32>,
        width: i32,
    }

    impl PassabilityProvider for FakePassabilityProvider {
        fn passable(&self, restriction_id: UnitTerrainRestrictionId, terrain_id: TerrainId) -> bool {
            0 != *terrain_id
        }
    }

    fn make_terrain_and_path_finder(passability: Vec<i32>, width: i32) -> (Terrain, PathFinder) {
        let empires = EmpiresDbRef::new(EmpiresDb::new());
        let tiles = passability.iter()
            .cloned()
            .map(|v| Tile::new((v as usize).into(), 0))
            .collect();

        let terrain = Terrain::new(width as i32,
                                   passability.len() as i32 / width,
                                   tiles,
                                   empires);
        let path_finder = PathFinder::new_with(Box::new(FakePassabilityProvider {
            map: passability,
            width: width,
        }));
        (terrain, path_finder)
    }

    #[test]
    fn test_find_tile_path() {
        let width = 7;
        let map = vec![
            1, 1, 1, 0, 1, 1, 1, // 1 = passable
            1, 1, 1, 0, 1, 1, 1, // 0 = impassible
            1, 1, 1, 0, 1, 1, 1, // for the purposes of this test
            0, 0, 0, 0, 0, 1, 0,
            1, 1, 0, 1, 1, 1, 1,
            1, 1, 0, 1, 1, 0, 1,
            1, 1, 1, 1, 1, 1, 1,
        ];

        let (terrain, path_finder) = make_terrain_and_path_finder(map, width);
        let occupied_tiles = OccupiedTiles::new();

        let test = &mut |from, to, exp| {
            let path = path_finder.find_tile_path(&terrain,
                                                  &occupied_tiles,
                                                  from,
                                                  to,
                                                  UnitTerrainRestrictionId::Flying);
            assert_eq!(exp, path);
        };

        // Simple cases
        test((0, 0), (0, 0), vec![(0, 0)]);
        test((0, 0), (0, 1), vec![(0, 0), (0, 1)]);
        test((0, 0), (0, 2), vec![(0, 0), (0, 1), (0, 2)]);
        test((0, 0), (2, 2), vec![(0, 0), (1, 1), (2, 2)]);

        // Test impossible path just returns path to closest tile
        test((0, 0), (2, 4), vec![(0, 0), (0, 1), (0, 2)]);
        test((6, 0), (0, 0), vec![(6, 0), (5, 0), (4, 0)]);

        // Test one corner of map to the other
        test((2, 4),
             (6, 0),
             vec![(2, 4), (3, 5), (4, 4), (5, 3), (6, 2), (7, 1), (6, 0)]);
    }

    #[test]
    fn test_find_tile_path_diagonal_only() {
        let width = 3;
        let map = vec![
            1, 0, 0,
            0, 1, 0,
            0, 0, 1,
        ];

        let (terrain, path_finder) = make_terrain_and_path_finder(map, width);
        let occupied_tiles = OccupiedTiles::new();

        let test = &mut |from, to, exp| {
            let path = path_finder.find_tile_path(&terrain,
                                                  &occupied_tiles,
                                                  from,
                                                  to,
                                                  UnitTerrainRestrictionId::Flying);
            assert_eq!(exp, path);
        };

        test((0, 0), (2, 2), vec![(0, 0), (1, 1), (2, 2)]);
    }
}
