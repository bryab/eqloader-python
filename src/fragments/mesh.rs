use libeq_wld::parser::{DmSpriteDef2, DmTrackDef2, FragmentType, MaterialDef, WldDoc};
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Arc};
extern crate owning_ref;
use crate::util::{u32_to_color, wld_f32_pos_to_gd, wld_i16_pos_to_py};

use super::create_fragment_ref;
use owning_ref::ArcRef;

#[pyclass]
pub struct S3DMesh {
    fragment: ArcRef<WldDoc, DmSpriteDef2>,
}

impl S3DMesh {
    pub fn new(wld: &Arc<WldDoc>, index: u32) -> Self {
        let fragment = wld.as_ref().at(index as usize - 1).unwrap();
        match fragment {
            FragmentType::DmSpriteDef2(_) => S3DMesh {
                fragment: create_fragment_ref(wld.clone(), index),
            },
            _ => panic!("S3DMesh trying to wrap a non-mesh fragment!"),
        }
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
}

// #[pyclass]
// pub struct S3DFace {
//     pub flags: u32,
//     pub vertices: Vec<(f32, f32, f32)>,
// }

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
            .map(|p| (1.0 - p.0 as f32 / 256. * -1., 1.0 - p.1 as f32 / 256.))
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
    pub fn face_material_groups(&self) -> HashMap<String, Vec<u16>> {
        let wld = self.get_wld();
        let materials = self.materials();
        let mut pos = 0;
        let frag = self.get_frag();
        frag.face_material_groups
            .iter()
            .enumerate()
            .map(|(_, (poly_count, ref material_idx))| {
                let material = materials[*material_idx as usize];

                let count = *poly_count as usize;
                let next_pos = pos + count;
                let batch = pos..next_pos;
                pos = next_pos;

                // If the material flags are 0, this is an invisible material.
                // Since we are dealing with collision separately, we can simply omit these polygons as they serve no purpose for rendering.
                // FIXME: It may be desirable to keep these for debugging purposes.  It would be wise to provide a flag for this.
                // if material.render_method.as_u32() == 0 {
                //     return None;
                // }

                let indices: Vec<u16> = frag
                    .faces
                    .get(batch)
                    .expect("Tried to get a Face from a Mesh that does not exist!")
                    .iter()
                    .flat_map(|face| {
                        vec![
                            face.vertex_indexes.0,
                            face.vertex_indexes.1,
                            face.vertex_indexes.2,
                        ]
                    })
                    .collect();

                let material_name = String::from(
                    wld.get_string(material.name_reference)
                        .expect("Material name should be a valid string"),
                );

                return (material_name, indices);
            })
            .collect()
    }
}
