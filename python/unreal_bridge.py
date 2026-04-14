import unreal
import json
from typing import Dict, List

class PrismaUnrealBridge:
    """
    Puente entre PRISMA (Rust) y Unreal Engine.
    Permite importar geometría de alta calidad y configurar
    materiales/iluminación generados por Daithon.
    """
    
    def __init__(self):
        self.editor_util = unreal.EditorAssetLibrary()
        self.editor_actor_subsystem = unreal.EditorActorSubsystem()
        
    def import_mesh_from_prisma(self, mesh_data: Dict) -> unreal.StaticMesh:
        """
        Importa una malla generada por VERTEX.
        
        Args:
            mesh_data: {
                'vertices': [[x,y,z, nx,ny,nz, u,v], ...],
                'indices': [i0, i1, i2, ...],
                'material_slots': [{'name': str, 'material_id': int}, ...]
            }
        """
        # Crear FBX temporal
        fbx_path = self._export_to_fbx(mesh_data)
        
        # Configurar opciones de importación
        import_options = unreal.FbxImportUI()
        import_options.set_editor_property('import_mesh', True)
        import_options.set_editor_property('import_as_skeletal', False)
        import_options.set_editor_property('import_materials', False)
        import_options.set_editor_property('import_textures', False)
        
        # Configuración de malla estática
        static_mesh_import_data = import_options.static_mesh_import_data
        static_mesh_import_data.set_editor_property('combine_meshes', True)
        static_mesh_import_data.set_editor_property('generate_lightmap_u_vs', True)
        static_mesh_import_data.set_editor_property('auto_generate_collision', True)
        
        # Importar
        import_task = unreal.AssetImportTask()
        import_task.set_editor_property('filename', fbx_path)
        import_task.set_editor_property('destination_path', '/Game/Daithon/GeneratedMeshes')
        import_task.set_editor_property('automated', True)
        import_task.set_editor_property('save', True)
        import_task.set_editor_property('options', import_options)
        
        unreal.AssetToolsHelpers.get_asset_tools().import_asset_tasks([import_task])
        
        # Obtener el mesh importado
        imported_assets = import_task.get_editor_property('imported_object_paths')
        if imported_assets:
            mesh = unreal.load_asset(imported_assets[0])
            
            # Configurar LODs si existen
            if 'lod_levels' in mesh_data:
                self._setup_lods(mesh, mesh_data['lod_levels'])
            
            # Configurar Nanite si es apropiado
            if mesh_data.get('enable_nanite', False):
                mesh.set_editor_property('nanite_settings.enabled', True)
            
            return mesh
        
        return None
    
    def _export_to_fbx(self, mesh_data: Dict) -> str:
        """Exporta mesh_data a FBX temporal."""
        import tempfile
        import os
        
        temp_dir = tempfile.gettempdir()
        fbx_path = os.path.join(temp_dir, f'daithon_mesh_{id(mesh_data)}.fbx')
        
        # Aquí se llamaría a una función de Rust/C++ que genere el FBX
        # Por ahora, placeholder
        
        return fbx_path
    
    def create_pbr_material_instance(
        self,
        material_data: Dict,
        base_material_path: str = '/Game/Daithon/Materials/M_DaithonPBR_Master'
    ) -> unreal.MaterialInstanceConstant:
        """
        Crea una instancia de material PBR desde datos de MATERIAL.
        """
        # Cargar material maestro
        master_material = unreal.load_asset(base_material_path)
        
        if not master_material:
            unreal.log_warning(f"Master material not found: {base_material_path}")
            return None
        
        # Crear instancia
        asset_tools = unreal.AssetToolsHelpers.get_asset_tools()
        material_name = material_data.get('name', 'DaithonMaterial')
        
        material_instance = asset_tools.create_asset(
            asset_name=f'MI_{material_name}',
            package_path='/Game/Daithon/Materials/Instances',
            asset_class=unreal.MaterialInstanceConstant,
            factory=unreal.MaterialInstanceConstantFactoryNew()
        )
        
        material_instance.set_editor_property('parent', master_material)
        
        # Configurar parámetros
        if 'base_color' in material_data:
            rgb = material_data['base_color']
            color = unreal.LinearColor(rgb[0], rgb[1], rgb[2], 1.0)
            unreal.MaterialEditingLibrary.set_material_instance_vector_parameter_value(
                material_instance,
                'BaseColor',
                color
            )
        
        if 'metallic' in material_data:
            unreal.MaterialEditingLibrary.set_material_instance_scalar_parameter_value(
                material_instance,
                'Metallic',
                material_data['metallic']
            )
        
        if 'roughness' in material_data:
            unreal.MaterialEditingLibrary.set_material_instance_scalar_parameter_value(
                material_instance,
                'Roughness',
                material_data['roughness']
            )
        
        if 'normal_strength' in material_data:
            unreal.MaterialEditingLibrary.set_material_instance_scalar_parameter_value(
                material_instance,
                'NormalStrength',
                material_data['normal_strength']
            )
        
        # Subsurface scattering
        if 'subsurface_color' in material_data:
            sss = material_data['subsurface_color']
            color = unreal.LinearColor(sss[0], sss[1], sss[2], 1.0)
            unreal.MaterialEditingLibrary.set_material_instance_vector_parameter_value(
                material_instance,
                'SubsurfaceColor',
                color
            )
            unreal.MaterialEditingLibrary.set_material_instance_scalar_parameter_value(
                material_instance,
                'SubsurfaceRadius',
                material_data.get('subsurface_radius', 0.0)
            )
        
        # Guardar
        unreal.EditorAssetLibrary.save_asset(material_instance.get_path_name())
        
        return material_instance
    
    def setup_radiance_lighting(self, lighting_config: Dict):
        """
        Configura el sistema de iluminación desde RADIANCE.
        """
        # Limpiar luces existentes de Daithon
        self._cleanup_daithon_lights()
        
        # Crear luces
        for light_data in lighting_config.get('lights', []):
            light_type = light_data.get('type', 'Point')
            
            if light_type == 'Directional':
                light_actor = self.editor_actor_subsystem.spawn_actor_from_class(
                    unreal.DirectionalLight,
                    unreal.Vector(0, 0, 0)
                )
                
            elif light_type == 'Point':
                light_actor = self.editor_actor_subsystem.spawn_actor_from_class(
                    unreal.PointLight,
                    unreal.Vector(*light_data.get('position', [0, 0, 0]))
                )
                
                light_component = light_actor.get_component_by_class(unreal.PointLightComponent)
                light_component.set_editor_property(
                    'attenuation_radius',
                    light_data.get('range', 1000.0)
                )
            
            elif light_type == 'Spot':
                light_actor = self.editor_actor_subsystem.spawn_actor_from_class(
                    unreal.SpotLight,
                    unreal.Vector(*light_data.get('position', [0, 0, 0]))
                )
                
                light_component = light_actor.get_component_by_class(unreal.SpotLightComponent)
                light_component.set_editor_property(
                    'inner_cone_angle',
                    light_data.get('inner_angle', 30.0)
                )
                light_component.set_editor_property(
                    'outer_cone_angle',
                    light_data.get('outer_angle', 45.0)
                )
            
            elif light_type == 'Skylight':
                light_actor = self.editor_actor_subsystem.spawn_actor_from_class(
                    unreal.SkyLight,
                    unreal.Vector(0, 0, 0)
                )
            
            # Propiedades comunes
            if light_actor:
                light_component = light_actor.get_component_by_class(unreal.LightComponent)
                
                light_component.set_editor_property(
                    'intensity',
                    light_data.get('intensity', 1.0)
                )
                
                if 'color' in light_data:
                    rgb = light_data['color']
                    light_component.set_editor_property(
                        'light_color',
                        unreal.LinearColor(rgb[0], rgb[1], rgb[2], 1.0)
                    )
                
                light_component.set_editor_property(
                    'cast_shadows',
                    light_data.get('cast_shadows', True)
                )
                
                light_actor.set_actor_label(f'Daithon_{light_type}_Light')
                light_actor.tags.append('DaithonLight')
        
        self._setup_post_process(lighting_config)
        
        if lighting_config.get('enable_lumen', True):
            self._enable_lumen()
    
    def _cleanup_daithon_lights(self):
        """Elimina luces previamente creadas por Daithon."""
        all_actors = unreal.EditorLevelLibrary.get_all_level_actors()
        
        for actor in all_actors:
            if 'DaithonLight' in actor.tags:
                self.editor_actor_subsystem.destroy_actor(actor)
    
    def _setup_post_process(self, config: Dict):
        """Configura Post Process Volume."""
        ppv = None
        all_actors = unreal.EditorLevelLibrary.get_all_level_actors()
        
        for actor in all_actors:
            if isinstance(actor, unreal.PostProcessVolume) and 'Daithon' in actor.tags:
                ppv = actor
                break
        
        if not ppv:
            ppv = self.editor_actor_subsystem.spawn_actor_from_class(
                unreal.PostProcessVolume,
                unreal.Vector(0, 0, 0)
            )
            ppv.tags.append('DaithonPPV')
            ppv.set_actor_label('Daithon_PostProcess')
        
        settings = ppv.settings
        
        if config.get('enable_bloom', True):
            settings.set_editor_property('bloom_intensity', config.get('bloom_intensity', 0.5))
            settings.set_editor_property('bloom_threshold', config.get('bloom_threshold', 1.0))
        
        if config.get('enable_ssao', True):
            settings.set_editor_property('ambient_occlusion_intensity', 1.0)
            settings.set_editor_property('ambient_occlusion_radius', config.get('ssao_radius', 100.0))
        
        if config.get('enable_ssr', True):
            settings.set_editor_property('screen_space_reflection_intensity', 100.0)
        
        ppv.set_editor_property('settings', settings)
        ppv.set_editor_property('unbound', True)
    
    def _enable_lumen(self):
        """Activa Lumen Global Illumination."""
        unreal.log("Lumen debe activarse en Project Settings > Rendering > Dynamic Global Illumination")
    
    def create_showcase_scene(
        self,
        mesh: unreal.StaticMesh,
        material: unreal.MaterialInstanceConstant,
        lighting_preset: str = 'studio'
    ):
        """
        Crea una escena de showcase lista para render.
        """
        mesh_actor = self.editor_actor_subsystem.spawn_actor_from_class(
            unreal.StaticMeshActor,
            unreal.Vector(0, 0, 0)
        )
        
        mesh_component = mesh_actor.get_component_by_class(unreal.StaticMeshComponent)
        mesh_component.set_static_mesh(mesh)
        
        if material:
            mesh_component.set_material(0, material)
        
        camera_actor = self.editor_actor_subsystem.spawn_actor_from_class(
            unreal.CameraActor,
            unreal.Vector(300, 300, 200)
        )
        camera_actor.set_actor_label('Daithon_ShowcaseCamera')
        
        # _create_turntable_sequence is stubbed
        unreal.log("Showcase scene created successfully")

if __name__ == "__main__":
    pass
