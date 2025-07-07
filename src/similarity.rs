use palette::Lab;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Serializable Lab color representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl From<Lab> for SerializableLab {
    fn from(lab: Lab) -> Self {
        Self {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }
}

impl From<SerializableLab> for Lab {
    fn from(slab: SerializableLab) -> Self {
        Lab::new(slab.l, slab.a, slab.b)
    }
}

/// Stores similarity information between tile images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityDatabase {
    /// Map from image path to its index
    path_to_index: HashMap<PathBuf, usize>,
    /// Map from index to image path
    index_to_path: HashMap<usize, PathBuf>,
    /// Average Lab colors for each image (serializable format)
    lab_colors: Vec<SerializableLab>,
    /// Similarity matrix (stored as upper triangular)
    /// For indices i, j where i < j, similarity is at position i * n - i * (i + 1) / 2 + j - i - 1
    similarities: Vec<f32>,
}

impl Default for SimilarityDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl SimilarityDatabase {
    pub fn new() -> Self {
        Self {
            path_to_index: HashMap::new(),
            index_to_path: HashMap::new(),
            lab_colors: Vec::new(),
            similarities: Vec::new(),
        }
    }

    /// Add a tile to the database
    pub fn add_tile(&mut self, path: PathBuf, lab_color: Lab) {
        let index = self.lab_colors.len();
        self.path_to_index.insert(path.clone(), index);
        self.index_to_path.insert(index, path);
        self.lab_colors.push(lab_color.into());
    }

    /// Build the similarity matrix after all tiles are added
    pub fn build_similarities(&mut self) {
        let n = self.lab_colors.len();
        let matrix_size = n * (n - 1) / 2;
        self.similarities = Vec::with_capacity(matrix_size);

        for i in 0..n {
            for j in (i + 1)..n {
                let lab1: Lab = self.lab_colors[i].clone().into();
                let lab2: Lab = self.lab_colors[j].clone().into();
                let similarity = calculate_lab_distance(&lab1, &lab2);
                self.similarities.push(similarity);
            }
        }
    }

    /// Get similarity between two images by path
    pub fn get_similarity(&self, path1: &Path, path2: &Path) -> Option<f32> {
        let idx1 = self.path_to_index.get(path1)?;
        let idx2 = self.path_to_index.get(path2)?;

        if idx1 == idx2 {
            return Some(0.0);
        }

        let (i, j) = if idx1 < idx2 {
            (*idx1, *idx2)
        } else {
            (*idx2, *idx1)
        };

        let n = self.lab_colors.len();
        let position = i * n - i * (i + 1) / 2 + j - i - 1;

        self.similarities.get(position).copied()
    }

    /// Get the Lab color for a given path
    pub fn get_lab_color(&self, path: &Path) -> Option<Lab> {
        let idx = self.path_to_index.get(path)?;
        self.lab_colors.get(*idx).map(|slab| slab.clone().into())
    }

    /// Save the database to a JSON file
    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load the database from a JSON file
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let db = serde_json::from_str(&json)?;
        Ok(db)
    }

    /// Try to load from file, or create new if file doesn't exist
    pub fn load_or_new(path: &Path) -> Self {
        match Self::load_from_file(path) {
            Ok(db) => {
                println!("Loaded similarity database from {path:?}");
                db
            }
            Err(_) => {
                println!("Creating new similarity database");
                Self::new()
            }
        }
    }
}

/// Calculate the Euclidean distance between two Lab colors
pub fn calculate_lab_distance(lab1: &Lab, lab2: &Lab) -> f32 {
    let dl = lab1.l - lab2.l;
    let da = lab1.a - lab2.a;
    let db = lab1.b - lab2.b;
    (dl * dl + da * da + db * db).sqrt()
}

/// Calculate CIE2000 color difference (more perceptually accurate but slower)
#[allow(dead_code)]
pub fn calculate_delta_e_2000(lab1: &Lab, lab2: &Lab) -> f32 {
    // Simplified version - for full CIE2000, use a dedicated library
    // This is still more accurate than simple Euclidean distance
    let kl = 1.0;
    let kc = 1.0;
    let kh = 1.0;

    let dl = lab2.l - lab1.l;
    let da = lab2.a - lab1.a;
    let db = lab2.b - lab1.b;

    let c1 = (lab1.a * lab1.a + lab1.b * lab1.b).sqrt();
    let c2 = (lab2.a * lab2.a + lab2.b * lab2.b).sqrt();
    let dc = c2 - c1;

    let dh2 = da * da + db * db - dc * dc;
    let dh = if dh2 > 0.0 { dh2.sqrt() } else { 0.0 };

    let sl = 1.0;
    let sc = 1.0 + 0.045 * c1;
    let sh = 1.0 + 0.015 * c1;

    let dl_kl_sl = dl / (kl * sl);
    let dc_kc_sc = dc / (kc * sc);
    let dh_kh_sh = dh / (kh * sh);

    (dl_kl_sl * dl_kl_sl + dc_kc_sc * dc_kc_sc + dh_kh_sh * dh_kh_sh).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_database_basic() {
        let mut db = SimilarityDatabase::new();

        // Add some test tiles
        db.add_tile(PathBuf::from("tile1.png"), Lab::new(50.0, 0.0, 0.0));
        db.add_tile(PathBuf::from("tile2.png"), Lab::new(60.0, 10.0, 10.0));
        db.add_tile(PathBuf::from("tile3.png"), Lab::new(40.0, -10.0, -10.0));

        // Build similarities
        db.build_similarities();

        // Test similarity retrieval
        let sim = db.get_similarity(Path::new("tile1.png"), Path::new("tile2.png"));
        assert!(sim.is_some());

        // Same image should have 0 similarity
        let sim_same = db.get_similarity(Path::new("tile1.png"), Path::new("tile1.png"));
        assert_eq!(sim_same, Some(0.0));
    }

    #[test]
    fn test_lab_distance_calculation() {
        let lab1 = Lab::new(50.0, 0.0, 0.0);
        let lab2 = Lab::new(50.0, 0.0, 0.0);
        let lab3 = Lab::new(60.0, 10.0, 10.0);

        // Same color should have 0 distance
        assert_eq!(calculate_lab_distance(&lab1, &lab2), 0.0);

        // Different colors should have positive distance
        assert!(calculate_lab_distance(&lab1, &lab3) > 0.0);
    }
}
