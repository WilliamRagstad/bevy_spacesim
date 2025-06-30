use bevy::prelude::*;
use big_space::prelude::*;

#[test]
fn test_big_space_components_exist() {
    // This test verifies that the big_space types are available and can be used
    let _grid = Grid::default();
    let _cell = GridCell::default();
    let _big_space = BigSpace { floating_origin: None };
    
    // Test that bundle types work
    let _spatial_bundle = BigSpatialBundle::default();
    let _root_bundle = BigSpaceRootBundle::default();
    
    println!("big_space integration components are available!");
}

#[test]
fn test_big_space_precision_demonstration() {
    // Demonstrate the precision advantage of big_space
    
    // Traditional approach - precision loss at large coordinates
    let large_position = Vec3::new(1_000_000_000.0, 0.0, 0.0);
    let small_offset = Vec3::new(0.1, 0.0, 0.0);
    let traditional_result = large_position + small_offset;
    
    // The small offset might be lost due to floating point precision
    let precision_lost = (traditional_result - large_position).length() < small_offset.length();
    
    println!("Large position: {:?}", large_position);
    println!("Small offset: {:?}", small_offset);
    println!("Traditional result: {:?}", traditional_result);
    println!("Precision lost in traditional approach: {}", precision_lost);
    
    // With big_space, the GridCell + Transform approach maintains precision
    // by keeping large coordinates in the grid cell and small local coordinates in transform
    let _grid_cell = GridCell::default(); // Would store the large coordinate part
    let local_transform = Transform::from_translation(small_offset); // Small local offset
    
    println!("BigSpace approach maintains precision by separating large grid coordinates from local transform");
    println!("Local transform: {:?}", local_transform.translation);
}