import bpy
import bmesh
import math

# -------------------------------------------------------------------
# 1. Clean Scene & Preferences
# -------------------------------------------------------------------
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

# Ensure linear interpolation for looping keyframes (Blender 4.x+)
bpy.context.preferences.edit.keyframe_new_interpolation_type = 'LINEAR'

# -------------------------------------------------------------------
# 2. Colors & Materials (Matching Reference Image)
# -------------------------------------------------------------------
def create_pbr_material(name, hex_color, roughness=0.5, metallic=0.0):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    
    hex_clean = hex_color.lstrip('#')
    rgb = [int(hex_clean[i:i+2], 16) / 255.0 for i in (0, 2, 4)]
    
    bsdf.inputs['Base Color'].default_value = (*rgb, 1.0)
    bsdf.inputs['Roughness'].default_value = roughness
    bsdf.inputs['Metallic'].default_value = metallic
    return mat

mat_tan = create_pbr_material("Mat_Tan", "D1AD7E", roughness=0.5)      # Tan Side Pods & Barrel
mat_dark = create_pbr_material("Mat_Dark", "242629", roughness=0.3)     # Charcoal Deck & Turret
mat_tread = create_pbr_material("Mat_Tread", "141517", roughness=0.8)   # Treads & Wheels

# Root object for Bevy positioning
root = bpy.data.objects.new("Tank_Root", None)
bpy.context.collection.objects.link(root)

# -------------------------------------------------------------------
# 3. Center Chassis Deck (Dark Charcoal)
# -------------------------------------------------------------------
bpy.ops.mesh.primitive_cube_add(size=1.0, location=(0, -0.05, 0.45))
chassis_deck = bpy.context.active_object
chassis_deck.name = "Chassis_Deck"
chassis_deck.scale = (1.3, 2.2, 0.35)
bpy.ops.object.transform_apply(scale=True)
chassis_deck.data.materials.append(mat_dark)

# Slope front (+Y) and rear (-Y) deck downwards
bm = bmesh.new()
bm.from_mesh(chassis_deck.data)
for v in bm.verts:
    if v.co.y > 0.6:   # Front slope
        v.co.z -= 0.12
    elif v.co.y < -0.6: # Rear slope
        v.co.z -= 0.12
bm.to_mesh(chassis_deck.data)
bm.free()

chassis_deck.parent = root
chassis_deck.matrix_parent_inverse = root.matrix_world.inverted()

# -------------------------------------------------------------------
# 4. Tan Side Armor Pods & Recessed Treads
# -------------------------------------------------------------------
wheel_objects = []

for side_name, x_pos in [("L", -0.85), ("R", 0.85)]:
    # Outer Tan Armor Pod
    bpy.ops.mesh.primitive_cube_add(size=1.0, location=(x_pos, 0, 0.35))
    pod = bpy.context.active_object
    pod.name = f"Armor_Pod_{side_name}"
    pod.scale = (0.55, 2.5, 0.5)
    bpy.ops.object.transform_apply(scale=True)
    pod.data.materials.append(mat_tan)
    
    # Chamfer pod ends (matching cartoon slope)
    bm = bmesh.new()
    bm.from_mesh(pod.data)
    for v in bm.verts:
        if v.co.z > 0 and abs(v.co.y) > 0.7:
            v.co.y *= 0.85
            v.co.z -= 0.1
        if v.co.z < 0 and abs(v.co.y) > 0.7:
            v.co.y *= 0.85
            v.co.z += 0.1
    bm.to_mesh(pod.data)
    bm.free()
    
    pod.parent = root
    pod.matrix_parent_inverse = root.matrix_world.inverted()

    # Inner Dark Tread Housing
    bpy.ops.mesh.primitive_cube_add(size=1.0, location=(x_pos, 0, 0.28))
    tread_back = bpy.context.active_object
    tread_back.name = f"Tread_Back_{side_name}"
    tread_back.scale = (0.48, 2.1, 0.3)
    bpy.ops.object.transform_apply(scale=True)
    tread_back.data.materials.append(mat_tread)
    
    tread_back.parent = pod
    tread_back.matrix_parent_inverse = pod.matrix_world.inverted()

    # 3 Road Wheels per side recessed in the slot
    for y_pos in [-0.65, 0.0, 0.65]:
        bpy.ops.mesh.primitive_cylinder_add(
            radius=0.2, depth=0.52, vertices=16,
            location=(x_pos, y_pos, 0.25)
        )
        wheel = bpy.context.active_object
        wheel.name = f"Wheel_{side_name}_{y_pos}"
        wheel.rotation_euler = (0, math.radians(90), 0)
        bpy.ops.object.transform_apply(rotation=True)  # Lock rotation transform
        wheel.data.materials.append(mat_tread)
        
        wheel.parent = pod
        wheel.matrix_parent_inverse = pod.matrix_world.inverted()
        wheel_objects.append(wheel)

# -------------------------------------------------------------------
# 5. Turret, Barrel & Muzzle
# -------------------------------------------------------------------
# Dark Octagonal Turret
bpy.ops.mesh.primitive_cylinder_add(
    radius=0.6, depth=0.42, vertices=8,
    location=(0, -0.1, 0.8)
)
turret = bpy.context.active_object
turret.name = "Turret_Main"
turret.data.materials.append(mat_dark)

# Taper turret top face
bm = bmesh.new()
bm.from_mesh(turret.data)
for v in bm.verts:
    if v.co.z > 0:
        v.co.x *= 0.85
        v.co.y *= 0.85
bm.to_mesh(turret.data)
bm.free()

turret.parent = root
turret.matrix_parent_inverse = root.matrix_world.inverted()

# Tan Gun Barrel (Pointing Forward along +Y)
bpy.ops.mesh.primitive_cylinder_add(
    radius=0.1, depth=0.85, vertices=16,
    location=(0, 0.65, 0.76)
)
barrel = bpy.context.active_object
barrel.name = "Turret_Barrel"
barrel.rotation_euler = (math.radians(90), 0, 0)
bpy.ops.object.transform_apply(rotation=True)  # Lock horizontal orientation
barrel.data.materials.append(mat_tan)

barrel.parent = turret
barrel.matrix_parent_inverse = turret.matrix_world.inverted()

# Tan Muzzle Ring
bpy.ops.mesh.primitive_cylinder_add(
    radius=0.13, depth=0.14, vertices=16,
    location=(0, 1.1, 0.76)
)
muzzle = bpy.context.active_object
muzzle.name = "Turret_Muzzle"
muzzle.rotation_euler = (math.radians(90), 0, 0)
bpy.ops.object.transform_apply(rotation=True)
muzzle.data.materials.append(mat_tan)

muzzle.parent = turret
muzzle.matrix_parent_inverse = turret.matrix_world.inverted()

# -------------------------------------------------------------------
# 6. Keyframe Animations
# -------------------------------------------------------------------
# Turret 360 Yaw Rotation
turret.animation_data_create()
action_turret = bpy.data.actions.new(name="Anim_TurretTurn")
turret.animation_data.action = action_turret

turret.rotation_euler = (0, 0, 0)
turret.keyframe_insert(data_path="rotation_euler", index=2, frame=1)
turret.rotation_euler = (0, 0, math.radians(360))
turret.keyframe_insert(data_path="rotation_euler", index=2, frame=60)

## Wheel Drive Animation
#for i, wheel in enumerate(wheel_objects):
#    wheel.animation_data_create()
#    action_wheel = bpy.data.actions.new(name=f"Anim_WheelSpin_{i}")
#    wheel.animation_data.action = action_wheel
#    
#    wheel.keyframe_insert(data_path="rotation_euler", index=0, frame=1)
#    wheel.rotation_euler.x += math.radians(360)
#    wheel.keyframe_insert(data_path="rotation_euler", index=0, frame=30)

# Push turret animation to NLA track for Bevy glTF exporter
track = turret.animation_data.nla_tracks.new()
track.name = "TurretTrack"
track.strips.new(action_turret.name, 1, action_turret)

# -------------------------------------------------------------------
# 7. Export to GLB
# -------------------------------------------------------------------
bpy.ops.object.select_all(action='SELECT')
bpy.ops.export_scene.gltf(
    filepath="cartoon_tank.glb",
    export_format='GLB',
    use_selection=True,
    export_yup=True,
    export_materials='EXPORT',
    export_animations=True,
    export_nla_strips=True
)
print("Cartoon tank generated and exported successfully.")