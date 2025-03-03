import logging
import bpy
import eqloader
import typing


logger = logging.getLogger(__name__)


def _buid_default_material_template(name: str) -> bpy.types.Material:
    logger.info(f"Building diffuse material: {name}")
    mat = bpy.data.materials.new(name)
    mat.use_fake_user = True
    mat.use_nodes = True
    image_node = mat.node_tree.nodes.new("ShaderNodeTexImage")
    bsdf = mat.node_tree.nodes["Principled BSDF"]
    bsdf.inputs["Roughness"].default_value = 1.0
    mat.node_tree.links.new(bsdf.inputs[0], image_node.outputs[0])
    return mat


def _build_transparent_material(name: str) -> bpy.types.Material:
    logger.info(f"Building transparent material: {name}")
    mat = bpy.data.materials.new(name)
    mat.use_fake_user = True
    mat.use_nodes = True
    mat.node_tree.links.clear()
    mat.node_tree.nodes.clear()
    transparent = mat.node_tree.nodes.new("ShaderNodeBsdfTransparent")
    output = mat.node_tree.nodes.new(type="ShaderNodeOutputMaterial")
    mat.node_tree.links.new(transparent.outputs["BSDF"], output.inputs["Surface"])
    return mat


def _build_masked_material(name: str) -> bpy.types.Material:
    logger.info(f"Building masked material: {name}")
    mat = bpy.data.materials.new(name)
    mat.use_fake_user = True
    mat.use_nodes = True
    nt = mat.node_tree
    image_node = nt.nodes.new("ShaderNodeTexImage")
    bsdf = nt.nodes["Principled BSDF"]
    bsdf.inputs["Roughness"].default_value = 1.0
    mix = mat.node_tree.nodes.new(type="ShaderNodeMixShader")
    transparent = nt.nodes.new("ShaderNodeBsdfTransparent")
    output = nt.nodes["Material Output"]

    # color to bsdf
    nt.links.new(image_node.outputs[0], bsdf.inputs[0])
    # alpha to mix
    nt.links.new(image_node.outputs[1], mix.inputs[0])
    # bsdf to mix
    nt.links.new(bsdf.outputs[0], mix.inputs[2])
    # transp to mix
    nt.links.new(transparent.outputs[0], mix.inputs[1])
    # mix to ouput
    nt.links.new(mix.outputs[0], output.inputs[0])
    return mat


def _find_node_type(
    nodes: bpy.types.Nodes, typename: str
) -> typing.Optional[bpy.types.Node]:
    for node in nodes:
        if typename in (node.name, node.bl_static_type):
            return node


def _get_material_template(shader_type: eqloader.ShaderType) -> bpy.types.Material:
    # logger.info(f"Shader type: {shader_type}")
    template_name = f"template_{shader_type.name}"
    if template_name in bpy.data.materials:
        return bpy.data.materials[template_name]
    logger.warning(f"Unsupported shader: {template_name}")
    assert type(shader_type) == eqloader.ShaderType, type(shader_type)
    if shader_type in (
        eqloader.ShaderType.Boundary,
        eqloader.ShaderType.TransparentSkydome,
    ):
        tmpl = _build_transparent_material(template_name)
    elif _is_masked_material(shader_type):
        tmpl = _build_masked_material(template_name)
    else:
        tmpl = _buid_default_material_template(template_name)
    tmpl["shader_type_id"] = shader_type.value
    tmpl["shader_type"] = shader_type.name
    return tmpl


tmp_test = False


def _is_masked_material(shader_type: eqloader.ShaderType):
    return shader_type in (
        eqloader.ShaderType.TransparentMasked,
        eqloader.ShaderType.TransparentMaskedPassable,
    )


def build_material(eqmat: eqloader.S3DMaterial) -> bpy.types.Material:
    global tmp_test
    template = _get_material_template(eqmat.shader_type)
    mat = template.copy()
    mat.name = eqmat.name
    mat.use_fake_user = False
    image_node = _find_node_type(mat.node_tree.nodes, "TEX_IMAGE")
    if not image_node:
        logger.error(f"No image input in material: {mat.name}")
    else:
        image = bpy.data.images[eqmat.texture_filename]
        image_node.image = image

        if _is_masked_material(eqmat.shader_type):
            add_mask_as_alpha(image, image["eq_mask_color"])
            # Here we copy the EQ mask color into the material for easier access
            if "eq_mask_color" in image:
                mat["eq_mask_color"] = image["eq_mask_color"]
    return mat


def _cmp_color(
    a: typing.Tuple[float, float, float], b: typing.Tuple[float, float, float]
):
    for i in range(3):
        if abs(a[i] - b[i]) > 0.001:
            return False
    return True


def add_mask_as_alpha(image: bpy.types.Image, color: typing.Tuple[float, float, float]):
    width = image.size[0]
    height = image.size[1]

    pixels = list(image.pixels)

    for x in range(width):
        for y in range(height):
            src_idx = (x + y * width) * 4
            rgb = (
                pixels[src_idx],
                pixels[src_idx + 1],
                pixels[src_idx + 2],
            )
            if _cmp_color(rgb, color):
                pixels[src_idx + 3] = 0.0

    image.pixels[:] = pixels
    image.update()


def build_materials(wld: eqloader.S3DWld) -> dict[str, bpy.types.Material]:
    materials: dict[str, bpy.types.Material] = {}
    for eqmat in wld.materials:
        if eqmat.name in bpy.data.materials:
            mat = bpy.data.materials[eqmat.name]
            # FIXME: Here we assume the existing material is setup correctly.
            # Need a flag to delete existing materials and recreate them.g
        else:
            mat = build_material(eqmat)
        materials[eqmat.name] = mat
    logger.info(f"Done loading materials.")
    return materials
