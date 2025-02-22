import pytest
from pathlib import Path
import blender_eqloader

fixtures_dir = Path(__file__).parent / 'fixtures'

def test_load_archive():
    test_filename = fixtures_dir / 'rivervale.s3d'
    archive = blender_eqloader.S3DArchive(str(test_filename))
    #print(archive.name)
    assert 'boathouse1.bmp' in archive.get_filenames()
    print(len(archive.get_filenames()))
    assert len(archive.get_filenames()) >= 87


    for filename in archive.get_filenames():
        if filename.endswith('.bmp'):
            image_bytes = archive.get_bytes(filename)
            assert len(image_bytes)
            assert isinstance(image_bytes, bytes)

    wld = archive.get_main_wld()
    assert wld
    print(wld)
    meshes = wld.meshes()
    assert len(meshes)
    #print(meshes)

    mesh = meshes[0]

    assert mesh.name
    print(mesh.name)

    x,y,z = mesh.vertices[0]
    assert x
    assert y
    assert z

    assert len(mesh.uvs)
    assert len(mesh.normals)

    for mesh in meshes:
        print(f"Mesh: {mesh.name} {mesh.center}")
        for material_name, indices in mesh.face_material_groups.items():
            print(f"Material: {material_name}. {len(indices)} indices")