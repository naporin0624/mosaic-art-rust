use crate::similarity::SimilarityDatabase;
use std::path::Path;

/// Represents a position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Get all adjacent positions (up, down, left, right)
    pub fn get_adjacent_positions(
        &self,
        grid_width: usize,
        grid_height: usize,
    ) -> Vec<GridPosition> {
        let mut adjacent = Vec::new();

        // Up
        if self.y > 0 {
            adjacent.push(GridPosition::new(self.x, self.y - 1));
        }

        // Down
        if self.y < grid_height - 1 {
            adjacent.push(GridPosition::new(self.x, self.y + 1));
        }

        // Left
        if self.x > 0 {
            adjacent.push(GridPosition::new(self.x - 1, self.y));
        }

        // Right
        if self.x < grid_width - 1 {
            adjacent.push(GridPosition::new(self.x + 1, self.y));
        }

        adjacent
    }
}

/// Manages adjacency penalties for tile placement
pub struct AdjacencyPenaltyCalculator<'a> {
    similarity_db: &'a SimilarityDatabase,
    penalty_weight: f32,
}

impl<'a> AdjacencyPenaltyCalculator<'a> {
    pub fn new(similarity_db: &'a SimilarityDatabase, penalty_weight: f32) -> Self {
        Self {
            similarity_db,
            penalty_weight,
        }
    }

    /// Calculate the adjacency penalty for placing a tile at a specific position
    pub fn calculate_penalty(
        &self,
        candidate_path: &Path,
        position: GridPosition,
        grid: &[Vec<Option<std::path::PathBuf>>],
        grid_width: usize,
        grid_height: usize,
    ) -> f32 {
        let adjacent_positions = position.get_adjacent_positions(grid_width, grid_height);
        let mut penalty = 0.0;

        for adj_pos in adjacent_positions {
            if let Some(neighbor_path) = &grid[adj_pos.y][adj_pos.x] {
                if let Some(similarity) = self
                    .similarity_db
                    .get_similarity(candidate_path, neighbor_path)
                {
                    // Higher similarity (smaller distance) results in higher penalty
                    // Using inverse with offset to avoid division by zero
                    penalty += 1.0 / (similarity + 1.0);
                }
            }
        }

        penalty * self.penalty_weight
    }

    /// Calculate total adjacency cost for the entire grid
    pub fn calculate_total_cost(&self, grid: &[Vec<Option<std::path::PathBuf>>]) -> f32 {
        let grid_height = grid.len();
        if grid_height == 0 {
            return 0.0;
        }
        let grid_width = grid[0].len();

        let mut total_cost = 0.0;

        for y in 0..grid_height {
            for x in 0..grid_width {
                if let Some(current_path) = &grid[y][x] {
                    // Only check right and down to avoid double counting
                    // Right neighbor
                    if x < grid_width - 1 {
                        if let Some(right_path) = &grid[y][x + 1] {
                            if let Some(similarity) =
                                self.similarity_db.get_similarity(current_path, right_path)
                            {
                                total_cost += 1.0 / (similarity + 1.0);
                            }
                        }
                    }

                    // Down neighbor
                    if y < grid_height - 1 {
                        if let Some(down_path) = &grid[y + 1][x] {
                            if let Some(similarity) =
                                self.similarity_db.get_similarity(current_path, down_path)
                            {
                                total_cost += 1.0 / (similarity + 1.0);
                            }
                        }
                    }
                }
            }
        }

        total_cost
    }

    /// Calculate the change in cost if two positions are swapped
    pub fn calculate_swap_delta(
        &self,
        grid: &[Vec<Option<std::path::PathBuf>>],
        pos1: GridPosition,
        pos2: GridPosition,
    ) -> f32 {
        let grid_height = grid.len();
        if grid_height == 0 {
            return 0.0;
        }
        let grid_width = grid[0].len();

        // Get the paths at both positions
        let path1 = match &grid[pos1.y][pos1.x] {
            Some(p) => p,
            None => return 0.0,
        };
        let path2 = match &grid[pos2.y][pos2.x] {
            Some(p) => p,
            None => return 0.0,
        };

        // If same path, no change
        if path1 == path2 {
            return 0.0;
        }

        let mut old_cost = 0.0;
        let mut new_cost = 0.0;

        // Calculate cost changes for pos1's neighbors
        let adj1 = pos1.get_adjacent_positions(grid_width, grid_height);
        for adj_pos in &adj1 {
            // Skip if it's pos2 (will be handled separately)
            if *adj_pos == pos2 {
                continue;
            }

            if let Some(adj_path) = &grid[adj_pos.y][adj_pos.x] {
                // Old cost with path1 at pos1
                if let Some(old_sim) = self.similarity_db.get_similarity(path1, adj_path) {
                    old_cost += 1.0 / (old_sim + 1.0);
                }
                // New cost with path2 at pos1
                if let Some(new_sim) = self.similarity_db.get_similarity(path2, adj_path) {
                    new_cost += 1.0 / (new_sim + 1.0);
                }
            }
        }

        // Calculate cost changes for pos2's neighbors
        let adj2 = pos2.get_adjacent_positions(grid_width, grid_height);
        for adj_pos in &adj2 {
            // Skip if it's pos1 (will be handled separately)
            if *adj_pos == pos1 {
                continue;
            }

            if let Some(adj_path) = &grid[adj_pos.y][adj_pos.x] {
                // Old cost with path2 at pos2
                if let Some(old_sim) = self.similarity_db.get_similarity(path2, adj_path) {
                    old_cost += 1.0 / (old_sim + 1.0);
                }
                // New cost with path1 at pos2
                if let Some(new_sim) = self.similarity_db.get_similarity(path1, adj_path) {
                    new_cost += 1.0 / (new_sim + 1.0);
                }
            }
        }

        // If pos1 and pos2 are adjacent, handle their mutual cost
        if adj1.contains(&pos2) {
            // Their similarity remains the same after swap, so no change in cost
            if let Some(sim) = self.similarity_db.get_similarity(path1, path2) {
                let cost = 1.0 / (sim + 1.0);
                old_cost += cost;
                new_cost += cost;
            }
        }

        new_cost - old_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::similarity::SimilarityDatabase;
    use palette::Lab;
    use std::path::PathBuf;

    #[test]
    fn test_grid_position_adjacent() {
        // Test center position
        let pos = GridPosition::new(1, 1);
        let adjacent = pos.get_adjacent_positions(3, 3);
        assert_eq!(adjacent.len(), 4); // Should have all 4 neighbors
        assert!(adjacent.contains(&GridPosition::new(1, 0))); // Up
        assert!(adjacent.contains(&GridPosition::new(1, 2))); // Down
        assert!(adjacent.contains(&GridPosition::new(0, 1))); // Left
        assert!(adjacent.contains(&GridPosition::new(2, 1))); // Right

        // Test corner position
        let corner = GridPosition::new(0, 0);
        let corner_adjacent = corner.get_adjacent_positions(3, 3);
        assert_eq!(corner_adjacent.len(), 2); // Only right and down
        assert!(corner_adjacent.contains(&GridPosition::new(1, 0))); // Right
        assert!(corner_adjacent.contains(&GridPosition::new(0, 1))); // Down
    }

    #[test]
    fn test_adjacency_penalty_calculation() {
        // Create a simple similarity database
        let mut sim_db = SimilarityDatabase::new();
        sim_db.add_tile(PathBuf::from("tile1.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.add_tile(PathBuf::from("tile2.png"), Lab::new(60.0, 10.0, 10.0));
        sim_db.add_tile(PathBuf::from("tile3.png"), Lab::new(40.0, -10.0, -10.0));
        sim_db.build_similarities();

        // Create calculator
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);

        // Create a simple grid
        let mut grid = vec![vec![None; 3]; 3];
        grid[0][0] = Some(PathBuf::from("tile1.png"));
        grid[0][1] = Some(PathBuf::from("tile2.png"));

        // Calculate penalty for placing tile3 at position (1, 0)
        let penalty = calculator.calculate_penalty(
            Path::new("tile3.png"),
            GridPosition::new(1, 0),
            &grid,
            3,
            3,
        );

        // Should have penalty from tile2 neighbor
        assert!(penalty > 0.0);
    }

    #[test]
    fn test_total_cost_calculation() {
        let mut sim_db = SimilarityDatabase::new();
        sim_db.add_tile(PathBuf::from("tile1.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.add_tile(PathBuf::from("tile2.png"), Lab::new(50.0, 0.0, 0.0)); // Same color
        sim_db.build_similarities();

        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);

        let mut grid = vec![vec![None; 2]; 2];
        grid[0][0] = Some(PathBuf::from("tile1.png"));
        grid[0][1] = Some(PathBuf::from("tile2.png"));

        let total_cost = calculator.calculate_total_cost(&grid);

        // Should have high cost since tiles are identical (similarity = 0)
        assert!(total_cost > 0.5);
    }
}
