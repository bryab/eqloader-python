use libeq_wld::parser::{Actor, Location, WldDoc};
use num_quaternion::{Quaternion, UnitQuaternion};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::sync::Arc;
extern crate owning_ref;
use super::{create_fragment_ref, S3DFragment};
#[cfg(feature = "serde")]
use super::{frag_to_dict, SerdeResult};
use crate::util::{
    quat_to_quat_vec, u32_to_color, wld_degrees_rot_to_quat, wld_f32_pos_to_gd, QuatVec, Vector3,
};
use owning_ref::ArcRef;

#[pyclass]
pub struct S3DActorInstance {
    fragment: ArcRef<WldDoc, Actor>,
    index: u32,
}

impl S3DFragment for S3DActorInstance {
    fn new(wld: &Arc<WldDoc>, index: u32) -> PyResult<Self> {
        Ok(S3DActorInstance {
            fragment: create_fragment_ref(wld.clone(), index)?,
            index,
        })
    }
}

#[pymethods]
impl S3DActorInstance {
    // FIXME: This appears to be empty
    #[getter]
    pub fn name(&self) -> String {
        String::from(
            self.get_wld()
                .get_string(self.get_frag().name_reference)
                .expect("Failed to get string from WLD!"),
        )
    }

    /// The index of the fragment within the WLD.
    #[getter]
    pub fn index(&self) -> u32 {
        self.index
    }

    #[getter]
    pub fn actordef_name(&self) -> String {
        // Note - If this is an invalid string reference,
        // Then it is probably actually a fragment reference.
        // The referenced fragment can be obtained via zone_actordef()
        String::from(
            self.get_wld()
                .get_string(self.get_frag().actor_def_reference)
                .unwrap_or(""),
        )
    }

    /// In a Zone WLD, the Actor can be obtained directly from this fragment,
    /// Unlike in placeable objects that refer to an actor defined in a different WLd
    /// by name.
    /// This method returns S3DActorDef or nil
    // #[getter]
    // pub fn zone_actordef(&self) -> Variant {
    //     let wld = self.get_wld();
    //     let index = self.get_frag().actor_def_reference.0;
    //     if index <= 0 {
    //         return Variant::nil();
    //     }
    //     gd_from_frag(wld, index as u32)
    // }

    /// Returns the vertex colors to be used for this instance, converted into Godot format.
    #[getter]
    pub fn vertex_colors(&self) -> PyResult<Vec<(f32, f32, f32, f32)>> {
        let wld = self.get_wld();
        let reference = self
            .get_frag()
            .vertex_color_reference
            .as_ref()
            .ok_or_else(|| PyValueError::new_err("Invalid fragment reference."))?;
        let fragment = wld
            .get(reference)
            .ok_or_else(|| PyValueError::new_err("Failed to resolve fragment reference."))?;
        let vertex_color_fragment = wld
            .get(&fragment.reference)
            .ok_or_else(|| PyValueError::new_err("Failed to resolve fragment reference."))?;
        Ok(vertex_color_fragment
            .vertex_colors
            .iter()
            .map(u32_to_color)
            .collect())
    }

    #[getter]
    pub fn position(&self) -> Vector3 {
        let loc = self.get_loc();
        wld_f32_pos_to_gd(&(loc.x, loc.y, loc.z))
    }

    #[getter]
    pub fn scale(&self) -> Vector3 {
        let frag = self.get_frag();
        let scale_factor = frag
            .scale_factor
            .expect("EQ ActorInstance should have scale_factor");
        let bounding_radius = frag
            .scale_factor
            .expect("EQ ActorInstance should have bounding_radius");
        (scale_factor, bounding_radius, scale_factor)
    }

    #[getter]
    pub fn quaternion(&self) -> QuatVec {
        quat_to_quat_vec(self._quaternion())
    }

    #[getter]
    pub fn rotation(&self) -> Vector3 {
        let euler = self
            ._quaternion()
            .normalize()
            .unwrap_or_else(|| UnitQuaternion::i())
            .to_euler_angles();
        (euler.roll, euler.pitch, euler.yaw)
    }

    #[cfg(feature = "serde")]
    #[getter]
    pub fn as_dict(&self) -> SerdeResult {
        let frag = self.get_frag();
        let wld = self.get_wld();
        frag_to_dict(wld, frag)
    }
}

impl S3DActorInstance {
    fn _quaternion(&self) -> Quaternion<f32> {
        let loc = self.get_loc();
        wld_degrees_rot_to_quat(loc.rotate_x, loc.rotate_y, loc.rotate_z)
    }

    fn get_loc(&self) -> &Location {
        self.get_frag()
            .location
            .as_ref()
            .expect("ActorInstanceFragment should always have Location")
    }

    fn get_wld(&self) -> &Arc<WldDoc> {
        self.fragment.as_owner()
    }

    fn get_frag(&self) -> &Actor {
        self.fragment.as_ref()
    }
}
