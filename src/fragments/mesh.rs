use libeq_wld::parser::{
    DmSprite, DmSpriteDef2, DmTrackDef2, FragmentRef, FragmentType, MaterialDef, WldDoc,
};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};
use std::sync::Arc;
extern crate owning_ref;
use crate::util::{u32_to_color, wld_f32_pos_to_gd, wld_i16_pos_to_py};

use super::{create_fragment_ref, S3DFragment};
use owning_ref::ArcRef;

#[pyclass]
pub struct S3DMesh {
    fragment: ArcRef<WldDoc, DmSpriteDef2>,
}

impl S3DFragment for S3DMesh {
    fn new(wld: &Arc<WldDoc>, index: u32) -> PyResult<Self> {
        Ok(S3DMesh {
            fragment: create_fragment_ref(wld.clone(), index)?,
        })
    }
}

impl S3DMesh {
    fn get_wld(&self) -> &Arc<WldDoc> {
        self.fragment.as_owner()
    }

    fn get_frag(&self) -> &DmSpriteDef2 {
        self.fragment.as_ref()
    }

    fn materials(&self) -> Vec<&MaterialDef> {
        let wld = self.get_wld();
        wld.get(&self.get_frag().material_list_ref)
            .expect("Invalid material list reference")
            .fragments
            .iter()
            .map(|fragment_ref| {
                wld.get(fragment_ref)
                    .expect("Material should exist - it's in the material list")
            })
            .collect()
    }

    fn get_dmtrackdef(&self) -> Option<&DmTrackDef2> {
        let wld = self.get_wld();
        let fragment = wld.get(&self.get_frag().animation_ref)?;
        wld.get(&fragment.reference)
    }

    pub fn from_reference(wld: &Arc<WldDoc>, mesh_reference: &DmSprite) -> PyResult<Self> {
        match mesh_reference.reference {
            FragmentRef::Index(index, _) => {
                let fragment = wld.at(index as usize - 1).ok_or_else(|| {
                    PyValueError::new_err(format!("Invalid fragment index: {index}"))
                })?;
                match fragment {
                    FragmentType::DmSpriteDef2(_) => S3DMesh::new(&wld, index),
                    _ => Err(PyValueError::new_err("Invalid fragment for this class")),
                }
            }
            FragmentRef::Name(_, _) => Err(PyValueError::new_err("Name references not supported")),
        }
    }
}

#[pyclass]
pub struct S3DFace {
    flags: u16,
    indices: (u16, u16, u16),
}

#[pymethods]
impl S3DFace {
    #[getter]
    pub fn indices(&self) -> (u16, u16, u16) {
        self.indices
    }

    #[getter]
    pub fn flags(&self) -> u16 {
        self.flags
    }
}

#[pymethods]
impl S3DMesh {
    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Ok(String::from(
            self.get_wld()
                .get_string(self.get_frag().name_reference)
                .expect("Failed to get string from WLD!"),
        ))
    }

    #[getter]
    fn flags(&self) -> u32 {
        self.fragment.as_ref().flags
    }

    #[getter]
    fn bounds(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        (
            wld_f32_pos_to_gd(&self.fragment.as_ref().min),
            wld_f32_pos_to_gd(&self.fragment.as_ref().max),
        )
    }

    #[getter]
    fn bounds_radius(&self) -> f32 {
        self.fragment.as_ref().max_distance
    }

    #[getter]
    fn center(&self) -> (f32, f32, f32) {
        wld_f32_pos_to_gd(&self.get_frag().center)
    }

    #[getter]
    pub fn vertices(&self) -> Vec<(f32, f32, f32)> {
        let frag = self.get_frag();
        let scale = 1.0 / (1 << frag.scale) as f32;
        frag.positions
            .iter()
            .map(|p| wld_i16_pos_to_py(&p, scale))
            .collect()
    }

    #[getter]
    pub fn normals(&self) -> Vec<(f32, f32, f32)> {
        self.get_frag()
            .vertex_normals
            .iter()
            .map(|p| (p.0 as f32 / 127., p.2 as f32 / 127., p.1 as f32 / 127.))
            .collect()
    }

    #[getter]
    pub fn vertex_colors(&self) -> Vec<(f32, f32, f32, f32)> {
        self.get_frag()
            .vertex_colors
            .iter()
            .map(u32_to_color)
            .collect()
    }

    #[getter]
    pub fn uvs(&self) -> Vec<(f32, f32)> {
        self.get_frag()
            .texture_coordinates
            .iter()
            .map(|v| ((v.0 as f32) / 256.0, (v.1 as f32) / 256.0))
            .collect()
    }

    // fn bone_indices(&self) -> Vec<u32> {
    //     self.get_frag()
    //         .skin_assignment_groups
    //         .iter()
    //         .flat_map(|(num_verts, bone_idx)| {
    //             vec![*bone_idx as u32, 0, 0, 0].repeat(*num_verts as usize)
    //         })
    //         .collect()
    // }

    // fn bone_weights(&self) -> PackedFloat32Array {
    //     self.get_frag()
    //         .skin_assignment_groups
    //         .iter()
    //         .flat_map(|(num_verts, _bone_idx)| vec![1., 0., 0., 0.].repeat(*num_verts as usize))
    //         .collect()
    // }
    #[getter]
    pub fn faces(&self) -> Vec<S3DFace> {
        let frag = self.get_frag();
        frag.faces
            .iter()
            .map(|face| S3DFace {
                flags: face.flags,
                indices: (
                    face.vertex_indexes.2,
                    face.vertex_indexes.1,
                    face.vertex_indexes.0,
                ),
            })
            .collect()
    }

    /// The first element of the tuple is the number of faces that use the same material. All
    /// polygon entries are sorted by material index so that faces use the same material are
    /// grouped together.
    ///
    /// The second element of the tuple is the name of the material.
    #[getter]
    pub fn face_material_groups(&self) -> Vec<(u16, String)> {
        let wld = self.get_wld();
        let materials = self.materials();
        let frag = self.get_frag();
        frag.face_material_groups
            .iter()
            .enumerate()
            .map(|(_, (poly_count, ref material_idx))| {
                let material = materials[*material_idx as usize];
                let material_name = String::from(
                    wld.get_string(material.name_reference)
                        .expect("Material name should be a valid string"),
                );
                (*poly_count, material_name)
            })
            .collect()
    }
}
