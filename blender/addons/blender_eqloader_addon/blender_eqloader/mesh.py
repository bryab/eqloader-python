import eqloader
import bpy
import logging
from .blender_util import _convert_position
from mathutils import Matrix

logger = logging.getLogger(__name__)

def build_meshes(wld: eqloader.S3DWld,bake_position: bool = False) -> list[bpy.types.Mesh]:
    meshes = []
    for eqmesh in wld.meshes:
        meshes.append(build_mesh(eqmesh, bake_position))
    return meshes
                
def build_mesh(eqmesh: eqloader.S3DMesh, bake_position: bool = False) -> bpy.types.Mesh:
    mesh = bpy.data.meshes.new(eqmesh.name)  # add a new mesh
    
    material_index = 0
    verts = [_convert_position(v) for v in eqmesh.vertices]
    mesh.from_pydata(verts, [], [f.indices for f in eqmesh.faces])

    # Add UV map
    
    
    eq_uvs = eqmesh.uvs
    if len(eq_uvs):

        uvmap = mesh.uv_layers.new(name='uv')
    
        for loop in mesh.loops:
            vi = loop.vertex_index
            uvmap.data[loop.index].uv = eq_uvs[vi]
    
    poly_index = 0
    material_index = 0
    for poly_count, material_name in eqmesh.face_material_groups:
        material = bpy.data.materials[material_name]
        mesh.materials.append(material)
        for polygon in mesh.polygons[poly_index: poly_index+poly_count]:
            polygon.material_index = material_index
        material_index += 1
        poly_index += poly_count
    
    #mesh.use_fake_user = True

    # Offset the mesh
    if bake_position:
        mesh.transform(Matrix.Translation(_convert_position(eqmesh.center)))
    
    return mesh




