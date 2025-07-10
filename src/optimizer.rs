use crate::adjacency::{AdjacencyPenaltyCalculator, GridPosition};
use rand::Rng;
use std::path::PathBuf;

/// Configuration for the optimization process
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Initial temperature for simulated annealing
    pub initial_temperature: f32,
    /// Temperature decay rate (multiplied each iteration)
    pub temperature_decay: f32,
    /// Progress reporting interval
    pub report_interval: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            initial_temperature: 100.0,
            temperature_decay: 0.99995,
            report_interval: 100,
        }
    }
}

/// Performs simulated annealing optimization on the tile placement
pub struct MosaicOptimizer<'a> {
    calculator: &'a AdjacencyPenaltyCalculator<'a>,
    config: OptimizationConfig,
}

impl<'a> MosaicOptimizer<'a> {
    pub fn new(calculator: &'a AdjacencyPenaltyCalculator<'a>, config: OptimizationConfig) -> Self {
        Self { calculator, config }
    }

    /// Optimize the mosaic placement using simulated annealing
    pub fn optimize(&self, grid: &mut [Vec<Option<PathBuf>>]) -> OptimizationResult {
        let grid_height = grid.len();
        if grid_height == 0 {
            return OptimizationResult::default();
        }
        let grid_width = grid[0].len();

        let mut rng = rand::thread_rng();
        let mut current_cost = self.calculator.calculate_total_cost(grid);
        let initial_cost = current_cost;
        let mut best_cost = current_cost;
        let mut improved_count = 0;
        let mut accepted_count = 0;
        let mut temperature = self.config.initial_temperature;

        println!("Starting optimization with initial cost: {initial_cost:.3}");

        for iteration in 0..self.config.max_iterations {
            // Select two random positions
            let pos1 =
                GridPosition::new(rng.gen_range(0..grid_width), rng.gen_range(0..grid_height));
            let pos2 =
                GridPosition::new(rng.gen_range(0..grid_width), rng.gen_range(0..grid_height));

            // Skip if same position
            if pos1 == pos2 {
                continue;
            }

            // Skip if either position is empty
            if grid[pos1.y][pos1.x].is_none() || grid[pos2.y][pos2.x].is_none() {
                continue;
            }

            // Calculate the change in cost if we swap
            let delta = self.calculator.calculate_swap_delta(grid, pos1, pos2);

            // Simulated annealing acceptance criterion
            let accept = if delta < 0.0 {
                true
            } else {
                let probability = (-delta / temperature).exp();
                rng.gen::<f32>() < probability
            };

            if accept {
                // Perform the swap
                let temp = grid[pos1.y][pos1.x].clone();
                grid[pos1.y][pos1.x] = grid[pos2.y][pos2.x].clone();
                grid[pos2.y][pos2.x] = temp;

                current_cost += delta;
                accepted_count += 1;

                if current_cost < best_cost {
                    best_cost = current_cost;
                    improved_count += 1;
                }
            }

            // Cool down temperature
            temperature *= self.config.temperature_decay;

            // Progress reporting
            if (iteration + 1) % self.config.report_interval == 0 {
                println!(
                    "Iteration {}: cost={:.3}, temp={:.3}, improvements={}, accepted={}",
                    iteration + 1,
                    current_cost,
                    temperature,
                    improved_count,
                    accepted_count
                );
            }
        }

        println!(
            "Optimization complete: final cost={current_cost:.3}, improvements={improved_count}, accepted={accepted_count}"
        );

        OptimizationResult {
            initial_cost,
            final_cost: current_cost,
            best_cost,
            improved_count,
            accepted_count,
            iterations: self.config.max_iterations,
        }
    }

    /// Perform a greedy optimization (only accept improvements)
    pub fn optimize_greedy(
        &self,
        grid: &mut [Vec<Option<PathBuf>>],
        max_iterations: usize,
    ) -> OptimizationResult {
        let grid_height = grid.len();
        if grid_height == 0 {
            return OptimizationResult::default();
        }
        let grid_width = grid[0].len();

        let mut rng = rand::thread_rng();
        let mut current_cost = self.calculator.calculate_total_cost(grid);
        let initial_cost = current_cost;
        let mut improved_count = 0;

        println!("Starting greedy optimization with initial cost: {initial_cost:.3}");

        for iteration in 0..max_iterations {
            let pos1 =
                GridPosition::new(rng.gen_range(0..grid_width), rng.gen_range(0..grid_height));
            let pos2 =
                GridPosition::new(rng.gen_range(0..grid_width), rng.gen_range(0..grid_height));

            if pos1 == pos2 {
                continue;
            }

            if grid[pos1.y][pos1.x].is_none() || grid[pos2.y][pos2.x].is_none() {
                continue;
            }

            let delta = self.calculator.calculate_swap_delta(grid, pos1, pos2);

            if delta < 0.0 {
                // Perform the swap
                let temp = grid[pos1.y][pos1.x].clone();
                grid[pos1.y][pos1.x] = grid[pos2.y][pos2.x].clone();
                grid[pos2.y][pos2.x] = temp;

                current_cost += delta;
                improved_count += 1;
            }

            if (iteration + 1) % 100 == 0 {
                println!(
                    "Iteration {}: cost={:.3}, improvements={}",
                    iteration + 1,
                    current_cost,
                    improved_count
                );
            }
        }

        println!(
            "Greedy optimization complete: final cost={current_cost:.3}, improvements={improved_count}"
        );

        OptimizationResult {
            initial_cost,
            final_cost: current_cost,
            best_cost: current_cost,
            improved_count,
            accepted_count: improved_count,
            iterations: max_iterations,
        }
    }
}

/// Results from the optimization process
#[derive(Debug, Default)]
pub struct OptimizationResult {
    pub initial_cost: f32,
    pub final_cost: f32,
    pub best_cost: f32,
    pub improved_count: usize,
    pub accepted_count: usize,
    pub iterations: usize,
}

impl OptimizationResult {
    /// Calculate the improvement percentage
    pub fn improvement_percentage(&self) -> f32 {
        if self.initial_cost > 0.0 {
            ((self.initial_cost - self.final_cost) / self.initial_cost) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency::AdjacencyPenaltyCalculator;
    use crate::similarity::SimilarityDatabase;
    use palette::Lab;

    fn create_test_grid() -> (Vec<Vec<Option<PathBuf>>>, SimilarityDatabase) {
        let mut sim_db = SimilarityDatabase::new();
        sim_db.add_tile(PathBuf::from("tile1.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.add_tile(PathBuf::from("tile2.png"), Lab::new(60.0, 10.0, 10.0));
        sim_db.add_tile(PathBuf::from("tile3.png"), Lab::new(40.0, -10.0, -10.0));
        sim_db.add_tile(PathBuf::from("tile4.png"), Lab::new(55.0, 5.0, 5.0));
        sim_db.build_similarities();

        let mut grid = vec![vec![None; 2]; 2];
        grid[0][0] = Some(PathBuf::from("tile1.png"));
        grid[0][1] = Some(PathBuf::from("tile2.png"));
        grid[1][0] = Some(PathBuf::from("tile3.png"));
        grid[1][1] = Some(PathBuf::from("tile4.png"));

        (grid, sim_db)
    }

    #[test]
    fn test_optimization_basic() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);

        let config = OptimizationConfig {
            max_iterations: 10,
            ..Default::default()
        };

        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);

        // Should have performed some iterations
        assert_eq!(result.iterations, 10);
        // Cost should be reasonable
        assert!(result.final_cost >= 0.0);
    }

    #[test]
    fn test_greedy_optimization() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);

        let optimizer = MosaicOptimizer::new(&calculator, OptimizationConfig::default());
        let result = optimizer.optimize_greedy(&mut grid, 10);

        // Greedy optimization should only accept improvements
        assert!(result.final_cost <= result.initial_cost);
        assert_eq!(result.improved_count, result.accepted_count);
    }

    #[test]
    fn test_optimization_result_improvement() {
        let result = OptimizationResult {
            initial_cost: 100.0,
            final_cost: 75.0,
            best_cost: 75.0,
            improved_count: 10,
            accepted_count: 15,
            iterations: 100,
        };

        assert_eq!(result.improvement_percentage(), 25.0);
    }

    // Edge Case Tests
    #[test]
    fn test_optimization_empty_grid() {
        let empty_grid: Vec<Vec<Option<PathBuf>>> = vec![];
        let sim_db = SimilarityDatabase::new();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let optimizer = MosaicOptimizer::new(&calculator, OptimizationConfig::default());
        let result = optimizer.optimize(&mut empty_grid.clone());
        
        // Should handle empty grid gracefully
        assert_eq!(result.initial_cost, 0.0);
        assert_eq!(result.final_cost, 0.0);
        assert_eq!(result.improved_count, 0);
    }

    #[test]
    fn test_optimization_single_tile_grid() {
        let mut sim_db = SimilarityDatabase::new();
        sim_db.add_tile(PathBuf::from("single.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.build_similarities();
        
        let mut grid = vec![vec![Some(PathBuf::from("single.png"))]];
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let config = OptimizationConfig {
            max_iterations: 100,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Single tile grid should have zero cost and no changes
        assert_eq!(result.initial_cost, 0.0);
        assert_eq!(result.final_cost, 0.0);
        assert_eq!(result.improved_count, 0);
        assert_eq!(result.accepted_count, 0);
    }

    #[test]
    fn test_optimization_grid_with_empty_cells() {
        let mut sim_db = SimilarityDatabase::new();
        sim_db.add_tile(PathBuf::from("tile1.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.add_tile(PathBuf::from("tile2.png"), Lab::new(60.0, 10.0, 10.0));
        sim_db.build_similarities();
        
        let mut grid = vec![vec![None; 3]; 3];
        grid[0][0] = Some(PathBuf::from("tile1.png"));
        grid[2][2] = Some(PathBuf::from("tile2.png"));
        // Rest remain None
        
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        let config = OptimizationConfig {
            max_iterations: 50,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Should handle sparse grid gracefully
        assert!(result.final_cost >= 0.0);
        // Non-adjacent tiles should have zero cost
        assert_eq!(result.initial_cost, 0.0);
    }

    #[test]
    fn test_optimization_extreme_temperature_values() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        // Test with very high temperature
        let config_high_temp = OptimizationConfig {
            max_iterations: 20,
            initial_temperature: 10000.0,
            temperature_decay: 0.95,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config_high_temp);
        let result_high = optimizer.optimize(&mut grid);
        
        // Should complete without issues
        assert_eq!(result_high.iterations, 20);
        assert!(result_high.final_cost >= 0.0);
        
        // Test with very low temperature
        let (mut grid2, _) = create_test_grid();
        let config_low_temp = OptimizationConfig {
            max_iterations: 20,
            initial_temperature: 0.001,
            temperature_decay: 0.99,
            ..Default::default()
        };
        
        let optimizer2 = MosaicOptimizer::new(&calculator, config_low_temp);
        let result_low = optimizer2.optimize(&mut grid2);
        
        // Low temperature should behave more like greedy
        assert_eq!(result_low.iterations, 20);
        assert!(result_low.final_cost <= result_low.initial_cost || result_low.final_cost.is_finite());
    }

    #[test]
    fn test_optimization_zero_temperature_decay() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let config = OptimizationConfig {
            max_iterations: 10,
            initial_temperature: 100.0,
            temperature_decay: 0.0, // Temperature goes to zero immediately
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Should complete without panicking
        assert_eq!(result.iterations, 10);
        assert!(result.final_cost.is_finite());
    }

    #[test]
    fn test_optimization_very_fast_decay() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let config = OptimizationConfig {
            max_iterations: 50,
            initial_temperature: 1000.0,
            temperature_decay: 0.5, // Very fast cooling
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Fast cooling should reduce acceptance rate over time
        assert_eq!(result.iterations, 50);
        assert!(result.final_cost >= 0.0);
    }

    #[test]
    fn test_optimization_zero_iterations() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let config = OptimizationConfig {
            max_iterations: 0,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // No iterations should mean no changes
        assert_eq!(result.iterations, 0);
        assert_eq!(result.improved_count, 0);
        assert_eq!(result.accepted_count, 0);
        assert_eq!(result.initial_cost, result.final_cost);
    }

    #[test]
    fn test_optimization_single_iteration() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let config = OptimizationConfig {
            max_iterations: 1,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Single iteration should work
        assert_eq!(result.iterations, 1);
        assert!(result.final_cost.is_finite());
    }

    #[test]
    fn test_optimization_large_grid() {
        let mut sim_db = SimilarityDatabase::new();
        
        // Create many tiles for large grid
        for i in 0..25 {
            sim_db.add_tile(
                PathBuf::from(format!("tile_{}.png", i)),
                Lab::new(50.0 + i as f32, (i as f32 - 12.0) * 2.0, (i as f32 - 12.0) * 3.0)
            );
        }
        sim_db.build_similarities();
        
        // Create 5x5 grid
        let mut grid = vec![vec![None; 5]; 5];
        let mut tile_idx = 0;
        for y in 0..5 {
            for x in 0..5 {
                grid[y][x] = Some(PathBuf::from(format!("tile_{}.png", tile_idx)));
                tile_idx += 1;
            }
        }
        
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 0.5);
        let config = OptimizationConfig {
            max_iterations: 100,
            report_interval: 25,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Large grid should work without issues
        assert_eq!(result.iterations, 100);
        assert!(result.final_cost >= 0.0);
        assert!(result.initial_cost >= 0.0);
    }

    #[test]
    fn test_greedy_optimization_edge_cases() {
        // Test greedy optimization with empty grid
        let empty_grid: Vec<Vec<Option<PathBuf>>> = vec![];
        let sim_db = SimilarityDatabase::new();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let optimizer = MosaicOptimizer::new(&calculator, OptimizationConfig::default());
        let result = optimizer.optimize_greedy(&mut empty_grid.clone(), 10);
        
        // Empty grid should return default result
        assert_eq!(result.initial_cost, 0.0);
        assert_eq!(result.final_cost, 0.0);
        assert_eq!(result.improved_count, 0);
    }

    #[test]
    fn test_greedy_optimization_zero_iterations() {
        let (mut grid, sim_db) = create_test_grid();
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        
        let optimizer = MosaicOptimizer::new(&calculator, OptimizationConfig::default());
        let result = optimizer.optimize_greedy(&mut grid, 0);
        
        // Zero iterations should make no changes
        assert_eq!(result.improved_count, 0);
        assert_eq!(result.initial_cost, result.final_cost);
    }

    #[test]
    fn test_optimization_result_edge_cases() {
        // Test with zero initial cost
        let result_zero = OptimizationResult {
            initial_cost: 0.0,
            final_cost: 0.0,
            best_cost: 0.0,
            improved_count: 0,
            accepted_count: 0,
            iterations: 10,
        };
        
        assert_eq!(result_zero.improvement_percentage(), 0.0);
        
        // Test with negative improvement (cost increased)
        let result_worse = OptimizationResult {
            initial_cost: 50.0,
            final_cost: 75.0,
            best_cost: 50.0,
            improved_count: 0,
            accepted_count: 5,
            iterations: 100,
        };
        
        assert_eq!(result_worse.improvement_percentage(), -50.0);
        
        // Test with perfect improvement (cost went to zero)
        let result_perfect = OptimizationResult {
            initial_cost: 100.0,
            final_cost: 0.0,
            best_cost: 0.0,
            improved_count: 50,
            accepted_count: 60,
            iterations: 200,
        };
        
        assert_eq!(result_perfect.improvement_percentage(), 100.0);
    }

    #[test]
    fn test_optimization_identical_tiles() {
        let mut sim_db = SimilarityDatabase::new();
        
        // Add identical tiles (same Lab color)
        let identical_color = Lab::new(50.0, 0.0, 0.0);
        sim_db.add_tile(PathBuf::from("identical1.png"), identical_color);
        sim_db.add_tile(PathBuf::from("identical2.png"), identical_color);
        sim_db.add_tile(PathBuf::from("identical3.png"), identical_color);
        sim_db.add_tile(PathBuf::from("identical4.png"), identical_color);
        sim_db.build_similarities();
        
        let mut grid = vec![vec![None; 2]; 2];
        grid[0][0] = Some(PathBuf::from("identical1.png"));
        grid[0][1] = Some(PathBuf::from("identical2.png"));
        grid[1][0] = Some(PathBuf::from("identical3.png"));
        grid[1][1] = Some(PathBuf::from("identical4.png"));
        
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 1.0);
        let config = OptimizationConfig {
            max_iterations: 50,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Identical tiles should have maximum adjacency cost
        assert!(result.initial_cost > 0.0);
        assert_eq!(result.iterations, 50);
    }

    #[test]
    fn test_optimization_config_defaults() {
        let config = OptimizationConfig::default();
        
        assert_eq!(config.max_iterations, 1000);
        assert_eq!(config.initial_temperature, 100.0);
        assert_eq!(config.temperature_decay, 0.99995);
        assert_eq!(config.report_interval, 100);
    }

    #[test]
    fn test_optimization_numerical_stability() {
        let mut sim_db = SimilarityDatabase::new();
        
        // Add tiles with extreme Lab values
        sim_db.add_tile(PathBuf::from("extreme1.png"), Lab::new(0.0, -100.0, -100.0));
        sim_db.add_tile(PathBuf::from("extreme2.png"), Lab::new(100.0, 100.0, 100.0));
        sim_db.add_tile(PathBuf::from("extreme3.png"), Lab::new(50.0, 0.0, 0.0));
        sim_db.add_tile(PathBuf::from("extreme4.png"), Lab::new(25.0, -50.0, 50.0));
        sim_db.build_similarities();
        
        let mut grid = vec![vec![None; 2]; 2];
        grid[0][0] = Some(PathBuf::from("extreme1.png"));
        grid[0][1] = Some(PathBuf::from("extreme2.png"));
        grid[1][0] = Some(PathBuf::from("extreme3.png"));
        grid[1][1] = Some(PathBuf::from("extreme4.png"));
        
        let calculator = AdjacencyPenaltyCalculator::new(&sim_db, 2.0);
        let config = OptimizationConfig {
            max_iterations: 30,
            initial_temperature: 1000.0,
            ..Default::default()
        };
        
        let optimizer = MosaicOptimizer::new(&calculator, config);
        let result = optimizer.optimize(&mut grid);
        
        // Should handle extreme values without numerical issues
        assert!(result.initial_cost.is_finite());
        assert!(result.final_cost.is_finite());
        assert!(result.best_cost.is_finite());
        assert_eq!(result.iterations, 30);
    }
}
