use super::{Candidate, CandidateId};
use crate::routing::paths::Path;
use log::{debug, trace};

pub(super) struct Config {
    pub dim: usize,
}

pub(super) struct ConvexHull {
    candidates: Vec<Candidate>,
    // return these paths in the end
    found_paths: Vec<Path>,
    dim: usize,
}

impl ConvexHull {
    pub fn with(cfg: Config) -> ConvexHull {
        ConvexHull {
            candidates: Vec::new(),
            found_paths: Vec::new(),
            dim: cfg.dim,
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn found_paths(&self) -> &Vec<Path> {
        &self.found_paths
    }

    pub fn path_from(&self, id: CandidateId) -> &Path {
        &self.found_paths[*id]
    }

    pub fn init_query(&mut self, cfg: Config) {
        self.candidates.clear();
        self.found_paths.clear();
        self.dim = cfg.dim;
    }

    pub fn has_volume(&self) -> bool {
        // +1 because a convex-hull (volume) needs dim+1 points
        // For imagination:
        // - line vs triangle in 2D
        // - triangle vs tetrahedron in 3D
        self.dim > 1 && self.found_paths.len() >= self.dim // TODO cyclops: (self.dim + 1)
    }

    pub fn pop_candidate(&mut self) -> Option<Candidate> {
        debug!("Pop from {} possible candidate(s)", self.candidates.len());
        self.candidates.pop()
    }

    pub fn contains(&self, path: &Path) -> bool {
        self.found_paths.contains(path)
    }

    pub fn push_path(&mut self, best_path: Path) {
        // if metric should be considered and path has not been added
        // -> remember path

        if self.found_paths.contains(&best_path) {
            trace!("already found path {}", best_path);
        } else {
            debug!("pushed {}", best_path);
            debug!("number of found paths: {}", self.found_paths.len());
        }

        self.found_paths.push(best_path);
    }

    pub fn init_candidates(&mut self) {
        // If not enough different paths have been found
        // -> return already found paths by keeping candidates empty

        let candidate = Candidate {
            ids: (0..self.found_paths.len()).map(CandidateId).collect(),
        };
        self.candidates.push(candidate);
    }

    pub fn update(&mut self, new_p: Path, candidate: Candidate) {
        // remember path
        if !self.contains(&new_p) {
            self.found_paths.push(new_p);

            // Add new facets by replacing every cost with the new path's cost.
            for i in 0..candidate.len() {
                let mut new_candidate = candidate.clone();
                new_candidate.ids[i] = CandidateId(self.found_paths.len() - 1);
                self.candidates.push(new_candidate);
            }
        }
    }
}
