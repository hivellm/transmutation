/// Layout postprocessing - merge, deduplicate, and order detected regions
/// 
/// Based on docling/utils/layout_postprocessor.py
use crate::document::types::DocItemLabel;
use crate::document::types_extended::{BoundingBox, Cluster};
use crate::error::{Result, TransmutationError};
use std::collections::{HashMap, HashSet};
use rstar::{RTree, AABB};

/// Union-Find data structure for grouping overlapping clusters
struct UnionFind {
    parent: HashMap<usize, usize>,
    rank: HashMap<usize, usize>,
}

impl UnionFind {
    fn new(elements: &[usize]) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        
        for &elem in elements {
            parent.insert(elem, elem);
            rank.insert(elem, 0);
        }
        
        Self { parent, rank }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[&x] != x {
            let root = self.find(self.parent[&x]);
            self.parent.insert(x, root); // Path compression
        }
        self.parent[&x]
    }
    
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return;
        }
        
        let rank_x = self.rank[&root_x];
        let rank_y = self.rank[&root_y];
        
        if rank_x > rank_y {
            self.parent.insert(root_y, root_x);
        } else if rank_x < rank_y {
            self.parent.insert(root_x, root_y);
        } else {
            self.parent.insert(root_y, root_x);
            self.rank.insert(root_x, rank_x + 1);
        }
    }
    
    fn get_groups(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        
        // Clone keys to avoid borrowing issue
        let keys: Vec<usize> = self.parent.keys().copied().collect();
        
        for elem in keys {
            let root = self.find(elem);
            groups.entry(root).or_insert_with(Vec::new).push(elem);
        }
        
        groups
    }
}

/// Spatial indexing for efficient overlap detection
struct SpatialIndex {
    rtree: RTree<ClusterRect>,
}

#[derive(Debug, Clone)]
struct ClusterRect {
    id: usize,
    bbox: BoundingBox,
}

impl rstar::RTreeObject for ClusterRect {
    type Envelope = AABB<[f64; 2]>;
    
    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.bbox.l, self.bbox.t],
            [self.bbox.r, self.bbox.b],
        )
    }
}

impl SpatialIndex {
    fn new(clusters: &[Cluster]) -> Self {
        let rects: Vec<ClusterRect> = clusters
            .iter()
            .map(|c| ClusterRect {
                id: c.id,
                bbox: c.bbox,
            })
            .collect();
        
        let rtree = RTree::bulk_load(rects);
        
        Self { rtree }
    }
    
    fn find_overlapping(&self, bbox: &BoundingBox, threshold: f64) -> Vec<usize> {
        let envelope = AABB::from_corners([bbox.l, bbox.t], [bbox.r, bbox.b]);
        
        self.rtree
            .locate_in_envelope_intersecting(&envelope)
            .filter(|rect| {
                let iou = rect.bbox.intersection_over_union(bbox);
                iou >= threshold
            })
            .map(|rect| rect.id)
            .collect()
    }
}

/// Options for layout postprocessing
pub struct LayoutPostprocessorOptions {
    pub merge_overlap_threshold: f64,
    pub merge_containment_threshold: f64,
    pub deduplicate_threshold: f64,
    pub enable_reading_order: bool,
}

impl Default for LayoutPostprocessorOptions {
    fn default() -> Self {
        Self {
            merge_overlap_threshold: 0.5,
            merge_containment_threshold: 0.8,
            deduplicate_threshold: 0.9,
            enable_reading_order: true,
        }
    }
}

/// Layout postprocessor
pub struct LayoutPostprocessor {
    options: LayoutPostprocessorOptions,
}

impl LayoutPostprocessor {
    pub fn new(options: LayoutPostprocessorOptions) -> Self {
        Self { options }
    }
    
    /// Process clusters: merge overlaps, remove duplicates, assign reading order
    pub fn process(&self, mut clusters: Vec<Cluster>) -> Result<Vec<Cluster>> {
        if clusters.is_empty() {
            return Ok(clusters);
        }
        
        // 1. Merge overlapping clusters
        clusters = self.merge_overlapping_clusters(clusters)?;
        
        // 2. Remove duplicates
        clusters = self.remove_duplicate_clusters(clusters)?;
        
        // 3. Assign reading order
        if self.options.enable_reading_order {
            clusters = self.sort_reading_order(clusters)?;
        }
        
        Ok(clusters)
    }
    
    /// Merge clusters with high overlap using Union-Find
    fn merge_overlapping_clusters(&self, clusters: Vec<Cluster>) -> Result<Vec<Cluster>> {
        if clusters.len() < 2 {
            return Ok(clusters);
        }
        
        let ids: Vec<usize> = clusters.iter().map(|c| c.id).collect();
        let mut uf = UnionFind::new(&ids);
        
        // Find overlapping pairs
        let spatial_index = SpatialIndex::new(&clusters);
        
        for cluster in &clusters {
            let overlapping = spatial_index.find_overlapping(
                &cluster.bbox,
                self.options.merge_overlap_threshold,
            );
            
            for &other_id in &overlapping {
                if other_id != cluster.id {
                    uf.union(cluster.id, other_id);
                }
            }
        }
        
        // Group clusters by root
        let groups = uf.get_groups();
        
        // Merge each group
        let mut merged_clusters = Vec::new();
        for (_root_id, group_ids) in groups {
            let group_clusters: Vec<&Cluster> = group_ids
                .iter()
                .filter_map(|&id| clusters.iter().find(|c| c.id == id))
                .collect();
            
            let merged = self.merge_cluster_group(&group_clusters)?;
            merged_clusters.push(merged);
        }
        
        Ok(merged_clusters)
    }
    
    /// Merge a group of clusters into one
    fn merge_cluster_group(&self, group: &[&Cluster]) -> Result<Cluster> {
        if group.is_empty() {
            return Err(TransmutationError::EngineError {
                engine: "layout-postprocessor".to_string(),
                message: "Cannot merge empty group".to_string(),
                source: None,
            });
        }
        
        if group.len() == 1 {
            return Ok((*group[0]).clone());
        }
        
        // Compute merged bounding box
        let mut min_l = f64::MAX;
        let mut min_t = f64::MAX;
        let mut max_r = f64::MIN;
        let mut max_b = f64::MIN;
        
        for cluster in group {
            min_l = min_l.min(cluster.bbox.l);
            min_t = min_t.min(cluster.bbox.t);
            max_r = max_r.max(cluster.bbox.r);
            max_b = max_b.max(cluster.bbox.b);
        }
        
        // Choose label with highest priority
        let label = self.choose_dominant_label(group);
        
        // Merge cells
        let mut all_cells = Vec::new();
        for cluster in group {
            all_cells.extend(cluster.cells.clone());
        }
        
        // Remove duplicate cells (by index)
        let mut seen_indices = HashSet::new();
        all_cells.retain(|cell| seen_indices.insert(cell.index));
        
        // Average confidence
        let avg_confidence = group.iter().map(|c| c.confidence).sum::<f32>() / group.len() as f32;
        
        Ok(Cluster {
            id: group[0].id, // Use first ID
            label,
            bbox: BoundingBox::new(min_l, min_t, max_r, max_b, group[0].bbox.origin),
            cells: all_cells,
            confidence: avg_confidence,
        })
    }
    
    /// Choose dominant label based on priority hierarchy
    fn choose_dominant_label(&self, group: &[&Cluster]) -> DocItemLabel {
        // Priority order (higher = more important)
        let priority = |label: DocItemLabel| -> usize {
            match label {
                DocItemLabel::Title => 100,
                DocItemLabel::SectionHeader => 90,
                DocItemLabel::Table => 85,
                DocItemLabel::Figure | DocItemLabel::Picture => 80,
                DocItemLabel::Formula => 75,
                DocItemLabel::Code => 70,
                DocItemLabel::ListItem => 60,
                DocItemLabel::Caption => 55,
                DocItemLabel::Footnote => 50,
                DocItemLabel::PageHeader | DocItemLabel::PageFooter => 40,
                DocItemLabel::Paragraph | DocItemLabel::Text => 30,
                _ => 10,
            }
        };
        
        group
            .iter()
            .max_by_key(|c| priority(c.label))
            .map(|c| c.label)
            .unwrap_or(DocItemLabel::Text)
    }
    
    /// Remove duplicate clusters (one completely contained in another)
    fn remove_duplicate_clusters(&self, mut clusters: Vec<Cluster>) -> Result<Vec<Cluster>> {
        let mut to_remove = HashSet::new();
        
        for i in 0..clusters.len() {
            for j in 0..clusters.len() {
                if i == j || to_remove.contains(&i) {
                    continue;
                }
                
                let containment = clusters[i]
                    .bbox
                    .intersection_over_self(&clusters[j].bbox);
                
                if containment >= self.options.deduplicate_threshold {
                    // Cluster i is contained in j, remove i
                    to_remove.insert(i);
                }
            }
        }
        
        clusters = clusters
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !to_remove.contains(i))
            .map(|(_, c)| c)
            .collect();
        
        Ok(clusters)
    }
    
    /// Sort clusters in reading order (top-to-bottom, left-to-right)
    fn sort_reading_order(&self, mut clusters: Vec<Cluster>) -> Result<Vec<Cluster>> {
        // Detect columns (groups with similar X range)
        let columns = self.detect_columns(&clusters);
        
        if columns.len() <= 1 {
            // Single column - simple sort
            clusters.sort_by(|a, b| {
                let y_cmp = a.bbox.t.partial_cmp(&b.bbox.t).unwrap();
                if y_cmp == std::cmp::Ordering::Equal {
                    a.bbox.l.partial_cmp(&b.bbox.l).unwrap()
                } else {
                    y_cmp
                }
            });
        } else {
            // Multi-column - sort within each column, then by column order
            clusters.sort_by(|a, b| {
                let col_a = self.get_column_index(&columns, &a.bbox);
                let col_b = self.get_column_index(&columns, &b.bbox);
                
                if col_a != col_b {
                    col_a.cmp(&col_b)
                } else {
                    // Same column - sort by Y then X
                    let y_cmp = a.bbox.t.partial_cmp(&b.bbox.t).unwrap();
                    if y_cmp == std::cmp::Ordering::Equal {
                        a.bbox.l.partial_cmp(&b.bbox.l).unwrap()
                    } else {
                        y_cmp
                    }
                }
            });
        }
        
        Ok(clusters)
    }
    
    /// Detect columns based on X-coordinate clustering
    fn detect_columns(&self, clusters: &[Cluster]) -> Vec<(f64, f64)> {
        // Simplified column detection - group by X ranges
        // TODO: Implement more sophisticated algorithm
        
        if clusters.is_empty() {
            return Vec::new();
        }
        
        // For now, assume single column
        let min_x = clusters.iter().map(|c| c.bbox.l).fold(f64::MAX, f64::min);
        let max_x = clusters.iter().map(|c| c.bbox.r).fold(f64::MIN, f64::max);
        
        vec![(min_x, max_x)]
    }
    
    /// Get column index for a bounding box
    fn get_column_index(&self, columns: &[(f64, f64)], bbox: &BoundingBox) -> usize {
        let center_x = (bbox.l + bbox.r) / 2.0;
        
        for (i, (col_l, col_r)) in columns.iter().enumerate() {
            if center_x >= *col_l && center_x <= *col_r {
                return i;
            }
        }
        
        0 // Default to first column
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::types_extended::CoordOrigin;
    
    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(&[1, 2, 3, 4]);
        uf.union(1, 2);
        uf.union(3, 4);
        
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(3), uf.find(4));
        assert_ne!(uf.find(1), uf.find(3));
        
        let groups = uf.get_groups();
        assert_eq!(groups.len(), 2);
    }
    
    #[test]
    fn test_merge_overlapping() {
        let postprocessor = LayoutPostprocessor::new(LayoutPostprocessorOptions::default());
        
        let clusters = vec![
            Cluster {
                id: 1,
                label: DocItemLabel::Text,
                bbox: BoundingBox::new(0.0, 0.0, 10.0, 10.0, CoordOrigin::TopLeft),
                cells: Vec::new(),
                confidence: 0.9,
            },
            Cluster {
                id: 2,
                label: DocItemLabel::Text,
                bbox: BoundingBox::new(5.0, 5.0, 15.0, 15.0, CoordOrigin::TopLeft),
                cells: Vec::new(),
                confidence: 0.8,
            },
        ];
        
        let result = postprocessor.merge_overlapping_clusters(clusters).unwrap();
        
        // Should be merged into one cluster
        assert_eq!(result.len(), 1);
    }
}

