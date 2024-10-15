
#![cfg(test)]

use struct_metadata::{Described, Descriptor, Kind, MetadataKind};
use nalgebra::{Matrix3, Vector3, Rotation3, Isometry3};

#[test]
fn matrix3_metadata() {
    let data: Descriptor<()> = Matrix3::<f32>::metadata();
    assert_eq!(data.docs, Some(vec!["A matrix from nalgebra"]));
    match data.kind {
        Kind::Sequence(ref boxed) => {
            assert!(matches!(boxed.kind, Kind::F32));
        },
        _ => panic!("Expected Matrix3 to be described as a sequence"),
    }
}


#[test]
fn vector3_metadata() {
    let data: Descriptor<()> = Vector3::<f32>::metadata();
    assert_eq!(data.docs, Some(vec!["A matrix from nalgebra"]));
    match data.kind {
        Kind::Sequence(boxed) => {
            assert!(matches!(boxed.kind, Kind::F32));
        },
        _ => panic!("Expected Vector3 to be described as a sequence"),
    }
}

#[test]
fn rotation3_metadata() {
    let data: Descriptor<()> = Rotation3::<f32>::metadata();
    assert_eq!(data.docs, Some(vec!["A 3D rotation matrix from nalgebra"]));
    
    match data.kind {
        Kind::Sequence(ref boxed) => {
            // Check that the sequence is a sequence of rows, and that the inner type is also a sequence
            match boxed.kind {
                Kind::Sequence(ref inner_boxed) => {
                    assert!(matches!(inner_boxed.kind, Kind::F32));
                },
                _ => panic!("Expected Rotation3 to be described as a sequence of sequences"),
            }
        },
        _ => panic!("Expected Rotation3 to be described as a sequence of elements"),
    }
}

#[test]
fn isometry3_metadata() {
    // let data = Isometry3::<f32>::metadata::<()>();
    let data: Descriptor<()> = Isometry3::<f32>::metadata();
    
    assert_eq!(data.docs, Some(vec!["A 3D isometry from nalgebra"]));
    
    if let Kind::Struct { children, .. } = data.kind {
        let rotation_entry = children.iter().find(|e| e.label == "rotation").expect("Rotation metadata not found");
        assert_eq!(rotation_entry.docs, Some(vec!["The rotation component of the isometry"]));
        match rotation_entry.type_info.kind {
            Kind::Sequence(_) => {}, // Expected sequence of rotation matrix elements
            _ => panic!("Expected rotation to be described as a sequence"),
        }

        let translation_entry = children.iter().find(|e| e.label == "translation").expect("Translation metadata not found");
        assert_eq!(translation_entry.docs, Some(vec!["The translation component of the isometry"]));
        match translation_entry.type_info.kind {
            Kind::Sequence(_) => {}, // Expected sequence of translation vector elements
            _ => panic!("Expected translation to be described as a vector"),
        }
    } else {
        panic!("Expected isometry to be described as a struct");
    }
}

#[derive(Default, MetadataKind)]
pub struct Meta {
    pub display_name: &'static str,
    pub description: &'static str,
}

/// # Transformation
/// A 3D transformation structure with a rotation and translation.
/// Includes metadata for `nalgebra` types.
#[derive(Debug, PartialEq, Default, Described)]
#[metadata_type(Meta)]
#[metadata(display_name = "Transformation", description = "A 3D transformation with rotation and translation")]
pub struct Transformation {
    /// Rotation component as a 3D rotation matrix
    #[metadata(display_name = "Rotation Matrix", description = "Rotation component as a 3D rotation matrix")]
    rotation: Rotation3<f32>,
    
    /// Translation component as a 3D vector
    #[metadata(display_name = "Translation Vector", description = "Translation component as a 3D vector")]
    translation: Vector3<f32>,
}

#[test]
fn transformation_metadata_with_meta() {
    let data: Descriptor<Meta> = Transformation::metadata();
    
    // Test struct-level metadata
    assert_eq!(data.metadata.display_name, "Transformation");
    assert_eq!(data.metadata.description, "A 3D transformation with rotation and translation");
    
    if let Kind::Struct { children, .. } = data.kind {
        // Check rotation field
        let rotation_entry = children.iter().find(|e| e.label == "rotation").expect("Rotation metadata not found");
        assert_eq!(rotation_entry.metadata.display_name, "Rotation Matrix");
        assert_eq!(rotation_entry.metadata.description, "Rotation component as a 3D rotation matrix");
        match rotation_entry.type_info.kind {
            Kind::Sequence(_) => {}, // Rotation is described as a sequence
            _ => panic!("Expected rotation to be described as a sequence"),
        }
        
        // Check translation field
        let translation_entry = children.iter().find(|e| e.label == "translation").expect("Translation metadata not found");
        assert_eq!(translation_entry.metadata.display_name, "Translation Vector");
        assert_eq!(translation_entry.metadata.description, "Translation component as a 3D vector");
        match translation_entry.type_info.kind {
            Kind::Sequence(_) => {}, // Translation is described as a sequence
            _ => panic!("Expected translation to be described as a sequence"),
        }
    } else {
        panic!("Expected Transformation to be described as a struct");
    }
}


/// # LinearSegment
/// A linear segment with start and end points represented as `Isometry3<f32>`.
#[derive(Debug, PartialEq, Described)]
#[metadata_type(Meta)]
#[metadata(display_name = "Linear Segment", description = "A linear segment with start and end points")]
pub struct LinearSegment {
    /// The start point of the segment
    #[metadata(display_name = "Start Point", description = "The start point of the segment")]
    pub start: Isometry3<f32>,
    
    /// The end point of the segment
    #[metadata(display_name = "End Point", description = "The end point of the segment")]
    pub end: Isometry3<f32>,
}

#[test]
fn linear_segment_metadata() {
    let data: Descriptor<Meta> = LinearSegment::metadata();
    
    // Test struct-level metadata
    assert_eq!(data.metadata.display_name, "Linear Segment");
    assert_eq!(data.metadata.description, "A linear segment with start and end points");
    
    if let Kind::Struct { children, .. } = data.kind {
        // Check the start field
        let start_entry = children.iter().find(|e| e.label == "start").expect("Start metadata not found");
        assert_eq!(start_entry.metadata.display_name, "Start Point");
        assert_eq!(start_entry.metadata.description, "The start point of the segment");
        match start_entry.type_info.kind {
            Kind::Struct { .. } => {}, // Expected to be a struct (Isometry3 is described as a struct)
            _ => panic!("Expected start point to be described as a struct"),
        }
        
        // Check the end field
        let end_entry = children.iter().find(|e| e.label == "end").expect("End metadata not found");
        assert_eq!(end_entry.metadata.display_name, "End Point");
        assert_eq!(end_entry.metadata.description, "The end point of the segment");
        match end_entry.type_info.kind {
            Kind::Struct { .. } => {}, // Expected to be a struct (Isometry3 is described as a struct)
            _ => panic!("Expected end point to be described as a struct"),
        }
    } else {
        panic!("Expected LinearSegment to be described as a struct");
    }
}
