//! Topology enumeration — counting possible network configurations

use serde::{Deserialize, Serialize};
use crate::{cayley, binomial, bell_number, stirling_second, stars_and_bars};

/// topology: how nodes are connected in a network.
/// Models different ways to arrange n nodes.

/// Count the number of possible communication topologies for n nodes.
/// This equals the number of labeled trees (Cayley's formula): n^{n-2}.
pub fn count_communication_topologies(n: u64) -> u64 {
    cayley(n)
}

/// Count the number of ways to assign n nodes to k teams (non-empty).
/// Uses Stirling numbers of the second kind.
pub fn count_team_assignments(n: u64, k: u64) -> u64 {
    stirling_second(n, k)
}

/// Count the number of ways to assign n nodes to any number of teams.
/// Uses Bell numbers.
pub fn count_all_team_assignments(n: u64) -> u64 {
    bell_number(n)
}

/// Count the number of ways to distribute n tasks among k nodes
/// (tasks are identical, nodes are distinct).
pub fn count_task_distributions(n: u64, k: u64) -> u64 {
    stars_and_bars(n, k)
}

/// Count the number of ways to assign n distinct tasks to k nodes (each task to one node).
pub fn count_distinct_task_assignments(n: u64, k: u64) -> u64 {
    k.pow(n as u32)
}

/// Count the number of possible hierarchical structures (rooted labeled trees) for n nodes.
/// This is n^{n-1} (rooted Cayley).
pub fn count_hierarchies(n: u64) -> u64 {
    if n == 0 { return 0; }
    n.pow((n - 1) as u32)
}

/// Count the number of possible pipeline configurations:
/// n nodes arranged in a linear pipeline, where order matters (permutations).
pub fn count_pipeline_configs(n: u64) -> u64 {
    crate::basic::factorial(n)
}

/// Count the number of ways to assign roles from a set of r distinct roles
/// to n nodes, where each gets exactly one role and multiple nodes
/// can share the same role.
pub fn count_role_assignments(n: u64, r: u64) -> u64 {
    r.pow(n as u32)
}

/// Count the number of fault-tolerant configurations:
/// Choose k nodes out of n to be primary, and the rest are backups.
pub fn count_fault_tolerant_configs(n: u64, k: u64) -> u64 {
    binomial(n, k)
}

/// Summary of all topology counts for n nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologySummary {
    pub n_nodes: u64,
    pub communication_topologies: u64,
    pub all_team_partitions: u64,
    pub hierarchies: u64,
    pub pipeline_configs: u64,
    pub task_distributions_2_nodes: u64,
}

/// Generate a summary of topology counts for n nodes.
pub fn topology_summary(n: u64) -> TopologySummary {
    TopologySummary {
        n_nodes: n,
        communication_topologies: count_communication_topologies(n),
        all_team_partitions: count_all_team_assignments(n),
        hierarchies: count_hierarchies(n),
        pipeline_configs: count_pipeline_configs(n),
        task_distributions_2_nodes: count_task_distributions(n, 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_topologies() {
        assert_eq!(count_communication_topologies(1), 1);
        assert_eq!(count_communication_topologies(3), 3);
        assert_eq!(count_communication_topologies(4), 16);
    }

    #[test]
    fn test_team_assignments() {
        // 4 nodes into 2 teams: S(4,2) = 7
        assert_eq!(count_team_assignments(4, 2), 7);
        assert_eq!(count_all_team_assignments(4), 15);
    }

    #[test]
    fn test_task_distributions() {
        // 5 identical tasks to 3 nodes: C(7,2) = 21
        assert_eq!(count_task_distributions(5, 3), 21);
    }

    #[test]
    fn test_distinct_task_assignments() {
        // 3 tasks, 2 nodes: 2^3 = 8
        assert_eq!(count_distinct_task_assignments(3, 2), 8);
    }

    #[test]
    fn test_hierarchies() {
        assert_eq!(count_hierarchies(1), 1);
        assert_eq!(count_hierarchies(3), 9); // 3^2
        assert_eq!(count_hierarchies(4), 64); // 4^3
    }

    #[test]
    fn test_pipeline_configs() {
        assert_eq!(count_pipeline_configs(3), 6);
        assert_eq!(count_pipeline_configs(4), 24);
    }

    #[test]
    fn test_role_assignments() {
        assert_eq!(count_role_assignments(3, 2), 8);
    }

    #[test]
    fn test_fault_tolerant_configs() {
        assert_eq!(count_fault_tolerant_configs(5, 3), 10);
    }

    #[test]
    fn test_topology_summary() {
        let s = topology_summary(3);
        assert_eq!(s.communication_topologies, 3);
        assert_eq!(s.all_team_partitions, 5); // B(3) = 5
        assert_eq!(s.hierarchies, 9);
        assert_eq!(s.pipeline_configs, 6);
    }
}
