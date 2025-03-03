import pytest
from pathlib import Path
import eqloader

fixtures_dir = Path(__file__).parent / "fixtures"

ST = eqloader.ShaderType


def test_load_zone_s3d():
    assert eqloader.WORLD_SCALE
    test_filename = fixtures_dir / "rivervale.s3d"
    archive = eqloader.S3DArchive(str(test_filename))
    # print(archive.name)
    assert "boathouse1.bmp" in archive.filenames
    print(len(archive.filenames))
    assert len(archive.filenames) >= 87

    # Need a transparent texture

    for filename in archive.filenames:
        if not filename.endswith(".bmp"):
            continue
        mask_color = archive.get_bmp_mask_color(filename)
        assert mask_color

    bmp_filename: str = next((f for f in archive.filenames if f.endswith(".bmp")))
    image_bytes = archive.get_bytes(bmp_filename)
    assert len(image_bytes)
    assert isinstance(image_bytes, bytes)

    # tmp_output = fixtures_dir / bmp_filename
    # if tmp_output.exists():
    #     tmp_output.unlink()
    # with open(tmp_output, 'wb') as f:
    #     f.write(image_bytes)

    wld = archive.main_wld
    assert wld
    print(wld)

    materials = wld.materials
    assert materials
    assert len(materials)
    material = materials[0]
    assert material.name == "RKGRASS_MDF", material.name
    assert material.texture_filename == "rkgrass.bmp", material.texture_filename
    assert material.shader_type_id == 1
    print(material.shader_type.name)
    assert material.shader_type == ST.Diffuse
    assert material.shader_type in (ST.Diffuse, ST.Diffuse2)
    meshes = wld.meshes
    assert len(meshes)
    # print(meshes)

    mesh = meshes[0]

    assert mesh.name
    print(mesh.name)

    x, y, z = mesh.vertices[0]
    assert x
    assert y
    assert z

    assert len(mesh.uvs)
    assert len(mesh.normals)

    for mesh in meshes:
        # verts = set()
        # for vert in mesh.vertices:
        #     if vert in verts:
        #         raise ValueError(f"Duplicate vert found: {vert}")
        #     verts.add(vert)

        print(f"Mesh: {mesh.name} {mesh.center}")
        for face in mesh.faces:
            print(f"Face: {face.flags} {face.indices}")

        for poly_count, material_name in mesh.face_material_groups:
            print(f"Material: {material_name}. {poly_count} faces")
            assert isinstance(material_name, str)
            assert isinstance(poly_count, int)

    actorinst_wld = archive.actorinst_wld

    actorinsts = actorinst_wld.actorinstances
    assert actorinsts
    assert len(actorinsts)

    for actorinst in actorinsts:
        assert actorinst.actordef_name
        print(f"{actorinst.name} - {actorinst.actordef_name} - {actorinst.index}")
        print(f"{actorinst.rotation}")

    # Now actually check out the actors themselves, which is a different file


def test_load_objects_s3d():
    test_filename = fixtures_dir / "rivervale_obj.s3d"
    archive = eqloader.S3DArchive(str(test_filename))
    wld = archive.main_wld

    actors = wld.actordefs
    assert actors
    assert len(actors) > 50

    for actor in actors:
        print(f"{actor.name}")
        for mesh in actor.meshes:
            print(f"{mesh.name} {mesh.center}")


def test_get_doors():
    pytest.skip("Need to provide means to get the PEQ db")
    sqlite_db_filepath = fixtures_dir / "peq_minimal.db"
    import sqlite3
    import json

    con = sqlite3.connect(sqlite_db_filepath)
    cur = con.cursor()
    res = cur.execute("SELECT * FROM doors")
    fields = [column[0] for column in res.description]
    items = res.fetchall()
    json_filepath = fixtures_dir / "peq.json"
    with open(json_filepath, "w") as f:
        json.dump(
            {
                "doors": [
                    {key: value for key, value in zip(fields, item)} for item in items
                ]
            },
            f,
        )
