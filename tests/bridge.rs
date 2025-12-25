use cognitive_topology::prelude::*;
use cognitive_topology::{BioState, BioCogBridge};

#[test]
fn test_vmem_mapping() {
    let bio = BioState { vmem: vec![-69.0, -65.0, -67.0] };  // Sample gradients
    // Use constructor because `superposition` field is private
    let manifold = CognitiveManifold::flat(3);
    
    let mut bridge = BioCogBridge { bio_state: bio, manifold };
    let curvature = bridge.map_vmem_to_curvature();
    
    // Golden scaling applied: tau_k * 1.618
    assert!(curvature > 0.0);
}

#[test]
fn test_goal_injection() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let mut bridge = BioCogBridge { bio_state: bio, manifold };
    
    // Updated to pass a Coordinate instead of a string
    let target = Coordinate::new(vec![1.0, 1.0, 1.0]);
    let result = bridge.inject_goal(5.0, target);
    assert!(result.is_ok());
}
