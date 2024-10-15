//! Metadata extensions for `nalgebra` types using `struct-metadata`.
//!
//! This module provides implementations of the `Described` trait for several types
//! in the `nalgebra` crate, allowing metadata to be attached to key mathematical
//! structures such as matrices, vectors, rotations, and isometries.
//!
//! The `nalgebra` library is a widely-used linear algebra library in Rust, and
//! this feature allows users to maintain a single source of truth for metadata
//! describing the structure of these types. Rather than storing metadata separately,
//! this extension integrates metadata directly into the types using `struct-metadata`.
//!
//! Supported Types:
//! - `Matrix<T, R, C>`: Describes a matrix with rows `R`, columns `C`, and elements of type `T`.
//! - `Vector3<T>`: Describes a 3D vector as a matrix with 3 rows and 1 column.
//! - `Rotation3<T>`: Describes a 3D rotation as a 3x3 matrix.
//! - `Isometry3<T>`: Describes an isometry (a combination of rotation and translation) in 3D space, 
//!   with metadata for both the rotation and translation components.
//!
//! This feature is intended to be used with the `nalgebra` library and can be enabled by
//! adding the `nalgebra` feature flag to the `struct-metadata` crate.

use crate::{Described, Descriptor, Kind, Entry};

use nalgebra::{ArrayStorage, Const, Isometry3, Scalar, Vector3, Rotation3};
use nalgebra::{Matrix, Dim, Storage};

impl<M: Default, T: Described<M> + Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> crate::Described<M> for Matrix<T, R, C, S> {
    fn metadata() -> Descriptor<M> {
        Descriptor {
            docs: Some(vec!["A matrix from nalgebra"]),
            metadata: M::default(),
            kind: Kind::Sequence(Box::new(T::metadata())),  // Each element is of type T, described by T::metadata()
        }
    }
}

impl<M: Default, T: Described<M> + Scalar> crate::Described<M> for Rotation3<T> {
    fn metadata() -> Descriptor<M> {
        Descriptor {
            docs: Some(vec!["A 3D rotation matrix from nalgebra"]),
            metadata: M::default(),
            kind: Kind::Sequence(Box::new(Matrix::<T, Const<3>, Const<3>, ArrayStorage<T, 3, 3>>::metadata())), // Describe as a 3x3 matrix
        }
    }
}

impl<M: Default, T: Described<M> + Scalar> crate::Described<M> for Isometry3<T> {
    fn metadata() -> Descriptor<M> {
        Descriptor {
            docs: Some(vec!["A 3D isometry from nalgebra"]),
            metadata: M::default(),
            kind: Kind::Struct {
                name: "Isometry3",
                children: vec![
                    Entry {
                        label: "rotation",
                        docs: Some(vec!["The rotation component of the isometry"]),
                        metadata: M::default(),
                        type_info: Rotation3::<T>::metadata(),// Rotation is a 3x3 matrix
                        has_default: false,
                        aliases: &[],
                    },
                    Entry {
                        label: "translation",
                        docs: Some(vec!["The translation component of the isometry"]),
                        metadata: M::default(),
                        type_info: Vector3::<T>::metadata(), // Translation is a 3D vector
                        has_default: false,
                        aliases: &[],
                    },
                ],
            },
        }
    }
}
