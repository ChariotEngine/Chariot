// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use empires::resource::*;
use error::*;

use identifier::*;
use chariot_io_tools::*;

use std::io::prelude::*;

type UnitResourceStorage = ResourceCost<f32, u8>;
type UnitResourceCost = ResourceCost<i16, i16>;

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct DamageGraphic {
    graphic_id: GraphicId,
    damage_percent: u8,
    old_apply_mode: u8,
    apply_mode: u8,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InteractionMode {
    NonInteracting,
    Flag,
    Resource,
    Building,
    Movable,
}

impl Default for InteractionMode {
    fn default() -> InteractionMode {
        InteractionMode::NonInteracting
    }
}

impl InteractionMode {
    pub fn from_u8(val: u8) -> Result<InteractionMode> {
        use self::InteractionMode::*;
        match val {
            0 => Ok(NonInteracting),
            1 => Ok(Flag),
            2 => Ok(Resource),
            3 => Ok(Building),
            4 => Ok(Movable),
            _ => Err(ErrorKind::InvalidInteractionMode(val).into()),
        }
    }
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum UnitType {
    GraphicEffect,
    Flag,
    Unknown25,
    Moveable,
    Commandable,
    BattleReady,
    Projectile,
    Trainable,
    Building,
    Tree,
}

impl Default for UnitType {
    fn default() -> UnitType {
        UnitType::Unknown25
    }
}

impl UnitType {
    pub fn from_u8(val: u8) -> Result<UnitType> {
        use self::UnitType::*;
        match val {
            10 => Ok(GraphicEffect),
            20 => Ok(Flag),
            25 => Ok(Unknown25),
            30 => Ok(Moveable),
            40 => Ok(Commandable),
            50 => Ok(BattleReady),
            60 => Ok(Projectile),
            70 => Ok(Trainable),
            80 => Ok(Building),
            90 => Ok(Tree),
            _ => Err(ErrorKind::InvalidUnitType(val).into()),
        }
    }

    pub fn has_motion_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Moveable | Commandable | BattleReady | Projectile | Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_commandable_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Commandable | BattleReady | Projectile | Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_battle_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            BattleReady | Projectile | Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_projectile_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Projectile => true,
            _ => false,
        }
    }

    pub fn has_trainable_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_building_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Building => true,
            _ => false,
        }
    }
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct UnitCommand {
    id: UnitCommandId,
    enabled: bool,
    type_id: i16,
    class_id: i16,
    unit_id: Option<UnitId>,
    terrain_id: Option<TerrainId>,
    resource_in: i16,
    resource_productivity_multiplier: i16,
    resource_out: i16,
    resource: i16,
    quantity: f32,
    execution_radius: f32,
    extra_range: f32,
    selection_enabler: i8,
    plunder_source: i16,
    selection_mode: i8,
    right_click_mode: i8,
    tool_graphic_id: Option<GraphicId>,
    proceeding_graphic_id: Option<GraphicId>,
    action_graphic_id: Option<GraphicId>,
    carrying_graphic_id: Option<GraphicId>,
    execution_sound_id: Option<SoundGroupId>,
    resource_deposit_sound_id: Option<SoundGroupId>,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct MotionParams {
    pub speed: f32,
    pub walking_graphics: [Option<GraphicId>; 2],
    rotation_speed: f32,
    tracking_unit: Option<UnitId>,
    tracking_unit_used: bool,
    tracking_unit_density: f32,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct CommandableParams {
    action_when_discovered_id: i16,
    search_radius: f32,
    work_rate: f32,
    drop_sites: [i16; 2],
    task_swap_id: i8,
    attack_sound: i16,
    move_sound: i16,
    animal_mode: i8,
    commands: Vec<UnitCommand>,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct BattleParams {
    default_armor: u8,
    attacks: Vec<(i16, i16)>, // class, amount
    armors: Vec<(i16, i16)>, // class, amount
    terrain_restriction_for_damage_multiplier: i16,
    max_range: f32,
    blast_width: f32,
    reload_time: f32,
    projectile_unit_id: Option<UnitId>,
    accuracy_percent: i16,
    tower_mode: i8,
    frame_delay: i16,
    graphic_displacements: [f32; 3],
    blast_attack_level: i8,
    min_range: f32,
    attack_graphic_id: Option<GraphicId>,
    displayed_melee_armour: i16,
    displayed_attack: i16,
    displayed_range: f32,
    displayed_reload_time: f32,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct BuildingParams {
    construction_graphic_id: GraphicId,
    adjacent_mode: i8,
    graphics_angle: i16,
    disappears_when_built: bool,
    stack_unit_id: Option<UnitId>,
    foundation_terrain_id: Option<TerrainId>,
    old_terrain_id: Option<TerrainId>,
    research_id: Option<ResearchId>,
    construction_sound: i16,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct ProjectileParams {
    stretch_mode: i8,
    smart_mode: i8,
    drop_animation_mode: i8,
    penetration_mode: i8,
    projectile_arc: f32,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct TrainableParams {
    resource_costs: Vec<UnitResourceCost>,
    train_time: i16,
    train_location_id: Option<UnitId>,
    button_id: i8,
    displayed_pierce_armor: i16,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct Unit {
    pub id: UnitId,

    // TODO: There's probably a better name for this since it indicates
    // which param groups used instead of unit type
    unit_type: UnitType,

    pub name: String,
    name_id: Option<LocalizationId>,
    creation_id: LocalizationId,
    class_id: i16,
    pub standing_graphic: Option<GraphicId>,

    /// Graphic IDs for when unit is dying; second one is never used
    pub dying_graphic: Option<GraphicId>,

    /// Always zero; use unknown
    death_mode: i8,

    hit_points: i16,
    line_of_sight: f32,
    garrison_capability: i8,
    pub collision_size_x: f32,
    pub collision_size_y: f32,
    pub collision_size_z: f32,

    /// Sound played when unit is trained
    train_sound_id: Option<SoundGroupId>,

    /// Replacement unit id for when the unit is dead and dying animation is completed
    dead_unit_id: Option<UnitId>,

    /// 0 = unit can be placed on other units in the map editor, 5 = it can't
    placement_mode: i8,

    /// Whether a unit is in the sky (for the map editor; doesn't affect gameplay)
    air_mode: bool,

    /// Frame number in 50730.slp in interfac.drs to use for the button graphic
    icon_id: i16,

    /// Whether or not to show the unit in the editor unit list
    hide_in_editor: bool,

    pub enabled: bool,

    /// The terrain type of one of the tiles near the placement location of the unit
    /// (in both editor and in-game); think docks next to water
    placement_side_terrain_ids: [Option<TerrainId>; 2],

    /// When placing the unit in the editor, the unit must be placed on a tile that has a
    /// terrain type with the same ID as one of these values
    placement_terrain_ids: [Option<TerrainId>; 2],

    clearance_size_x: f32,
    clearance_size_y: f32,
    hill_mode: i8,
    visible_in_fog: bool,

    /// Which terrains the unit can walk on
    pub terrain_restriction: UnitTerrainRestrictionId,

    fly_mode: bool,
    resource_capacity: i16,
    resource_decay: f32,

    /// Unit would only be affected by a blast attack with the same or lower level
    blast_defense_level: i8,

    sub_type: i8,
    pub interaction_mode: InteractionMode,
    minimap_mode: i8,
    command_attribute: i8,
    minimap_color: u8,
    help_id: Option<LocalizationId>,
    hotkey_text_id: Option<LocalizationId>,
    hotkey: i32,
    pub unselectable: bool,
    enable_auto_gather: bool,
    auto_gather_mode: i8,
    auto_gather_id: i8,

    selection_effect: i8,
    editor_selection_color: u8,
    pub selection_shape_size_x: f32,
    pub selection_shape_size_y: f32,
    pub selection_shape_size_z: f32,

    resource_storage: Vec<UnitResourceStorage>,
    damage_graphics: Vec<DamageGraphic>,

    selection_sound: i16,
    dying_sound: i16,
    attack_mode: i8,

    id2: i16,

    pub motion_params: Option<MotionParams>,
    commandable_params: Option<CommandableParams>,
    battle_params: Option<BattleParams>,
    projectile_params: Option<ProjectileParams>,
    trainable_params: Option<TrainableParams>,
    building_params: Option<BuildingParams>,
}

pub fn read_unit<R: Read + Seek>(stream: &mut R) -> Result<Unit> {
    let mut unit: Unit = Default::default();

    unit.unit_type = try!(UnitType::from_u8(try!(stream.read_u8())));
    let name_length = try!(stream.read_u16()) as usize;
    unit.id = required_id!(try!(stream.read_i16()));
    unit.name_id = optional_id!(try!(stream.read_i16()));
    unit.creation_id = required_id!(try!(stream.read_i16()));
    unit.class_id = try!(stream.read_i16());
    unit.standing_graphic = optional_id!(try!(stream.read_i16()));
    unit.dying_graphic = optional_id!(try!(stream.read_i16()));
    try!(stream.read_i16()); // unused (dying graphic 2)
    unit.death_mode = try!(stream.read_i8());
    unit.hit_points = try!(stream.read_i16());
    unit.line_of_sight = try!(stream.read_f32());
    unit.garrison_capability = try!(stream.read_i8());
    unit.collision_size_x = try!(stream.read_f32());
    unit.collision_size_y = try!(stream.read_f32());
    unit.collision_size_z = try!(stream.read_f32());
    unit.train_sound_id = optional_id!(try!(stream.read_i16()));
    unit.dead_unit_id = optional_id!(try!(stream.read_i16()));
    unit.placement_mode = try!(stream.read_i8());
    unit.air_mode = try!(stream.read_u8()) != 0;
    unit.icon_id = try!(stream.read_i16());
    unit.hide_in_editor = try!(stream.read_u8()) != 0;
    try!(stream.read_u16()); // unknown
    unit.enabled = try!(stream.read_u8()) != 0;

    unit.placement_side_terrain_ids[0] = optional_id!(try!(stream.read_i16()));
    unit.placement_side_terrain_ids[1] = optional_id!(try!(stream.read_i16()));
    unit.placement_terrain_ids[0] = optional_id!(try!(stream.read_i16()));
    unit.placement_terrain_ids[1] = optional_id!(try!(stream.read_i16()));
    unit.clearance_size_x = try!(stream.read_f32());
    unit.clearance_size_y = try!(stream.read_f32());
    unit.hill_mode = try!(stream.read_i8());
    unit.visible_in_fog = try!(stream.read_u8()) != 0;
    unit.terrain_restriction = UnitTerrainRestrictionId::from_index(try!(stream.read_i16()) as usize);
    unit.fly_mode = try!(stream.read_i8()) != 0;
    unit.resource_capacity = try!(stream.read_i16());
    unit.resource_decay = try!(stream.read_f32());
    unit.blast_defense_level = try!(stream.read_i8());
    unit.sub_type = try!(stream.read_i8());
    unit.interaction_mode = try!(InteractionMode::from_u8(try!(stream.read_u8())));
    unit.minimap_mode = try!(stream.read_i8());
    unit.command_attribute = try!(stream.read_i8());
    try!(stream.read_f32()); // unknown
    unit.minimap_color = try!(stream.read_u8());
    unit.help_id = optional_id!(try!(stream.read_i32()));
    unit.hotkey_text_id = optional_id!(try!(stream.read_i32()));
    unit.hotkey = try!(stream.read_i32());
    unit.unselectable = try!(stream.read_u8()) != 0;
    unit.enable_auto_gather = try!(stream.read_u8()) != 0;
    unit.auto_gather_mode = try!(stream.read_i8());
    unit.auto_gather_id = try!(stream.read_i8());

    unit.selection_effect = try!(stream.read_i8());
    unit.editor_selection_color = try!(stream.read_u8());
    unit.selection_shape_size_x = try!(stream.read_f32());
    unit.selection_shape_size_y = try!(stream.read_f32());
    unit.selection_shape_size_z = try!(stream.read_f32());

    unit.resource_storage = read_resource_costs!(f32, u8, stream, 3);

    let damage_graphic_count = try!(stream.read_u8()) as usize;
    unit.damage_graphics = try!(stream.read_array(damage_graphic_count, |c| read_damage_graphic(c)));

    unit.selection_sound = try!(stream.read_i16());
    unit.dying_sound = try!(stream.read_i16());
    unit.attack_mode = try!(stream.read_i8());
    try!(stream.read_u8()); // Unknown

    unit.name = try!(stream.read_sized_str(name_length));
    unit.id2 = try!(stream.read_i16());

    match unit.unit_type {
        UnitType::Tree | UnitType::GraphicEffect => {
            // No params on these types
            return Ok(unit);
        }
        UnitType::Flag |
        UnitType::Unknown25 => {
            // Skip what may be the speed; but on a non-moveable
            try!(stream.read_f32());
        }
        _ => {}
    }

    if unit.unit_type.has_motion_params() {
        unit.motion_params = Some(try!(read_motion_params(stream)));
    }
    if unit.unit_type.has_commandable_params() {
        unit.commandable_params = Some(try!(read_commandable_params(stream)));
    }
    if unit.unit_type.has_battle_params() {
        unit.battle_params = Some(try!(read_battle_params(stream)));
    }
    if unit.unit_type.has_projectile_params() {
        unit.projectile_params = Some(try!(read_projectile_params(stream)));
    }
    if unit.unit_type.has_trainable_params() {
        unit.trainable_params = Some(try!(read_trainable_params(stream)));
    }
    if unit.unit_type.has_building_params() {
        unit.building_params = Some(try!(read_building_params(stream)));
    }

    Ok(unit)
}

fn read_damage_graphic<R: Read>(stream: &mut R) -> Result<DamageGraphic> {
    let mut damage_graphic: DamageGraphic = Default::default();
    damage_graphic.graphic_id = required_id!(try!(stream.read_i16()));
    damage_graphic.damage_percent = try!(stream.read_u8());
    damage_graphic.old_apply_mode = try!(stream.read_u8());
    damage_graphic.apply_mode = try!(stream.read_u8());
    Ok(damage_graphic)
}

fn read_motion_params<R: Read>(stream: &mut R) -> Result<MotionParams> {
    let mut params: MotionParams = Default::default();
    params.speed = try!(stream.read_f32());
    params.walking_graphics[0] = optional_id!(try!(stream.read_i16()));
    params.walking_graphics[1] = optional_id!(try!(stream.read_i16()));
    params.rotation_speed = try!(stream.read_f32());
    try!(stream.read_u8()); // unknown
    params.tracking_unit = optional_id!(try!(stream.read_i16()));
    params.tracking_unit_used = try!(stream.read_u8()) != 0;
    params.tracking_unit_density = try!(stream.read_f32());
    try!(stream.read_u8()); // unknown
    Ok(params)
}

fn read_commandable_params<R: Read>(stream: &mut R) -> Result<CommandableParams> {
    let mut params: CommandableParams = Default::default();
    params.action_when_discovered_id = try!(stream.read_i16());
    params.search_radius = try!(stream.read_f32());
    params.work_rate = try!(stream.read_f32());
    params.drop_sites[0] = try!(stream.read_i16());
    params.drop_sites[1] = try!(stream.read_i16());
    params.task_swap_id = try!(stream.read_i8());
    params.attack_sound = try!(stream.read_i16());
    params.move_sound = try!(stream.read_i16());
    params.animal_mode = try!(stream.read_i8());

    let command_count = try!(stream.read_u16()) as usize;
    params.commands = try!(stream.read_array(command_count, |c| read_unit_command(c)));
    Ok(params)
}

fn read_unit_command<R: Read>(stream: &mut R) -> Result<UnitCommand> {
    let mut command: UnitCommand = Default::default();
    command.enabled = try!(stream.read_u16()) != 0;
    command.id = required_id!(try!(stream.read_i16()));
    try!(stream.read_u8()); // unknown
    command.type_id = try!(stream.read_i16());
    command.class_id = try!(stream.read_i16());
    command.unit_id = optional_id!(try!(stream.read_i16()));
    command.terrain_id = optional_id!(try!(stream.read_i16()));
    command.resource_in = try!(stream.read_i16());
    command.resource_productivity_multiplier = try!(stream.read_i16());
    command.resource_out = try!(stream.read_i16());
    command.resource = try!(stream.read_i16());
    command.quantity = try!(stream.read_f32());
    command.execution_radius = try!(stream.read_f32());
    command.extra_range = try!(stream.read_f32());
    try!(stream.read_u8()); // unknown
    try!(stream.read_f32()); // unknown
    command.selection_enabler = try!(stream.read_i8());
    try!(stream.read_u8()); // unknown
    command.plunder_source = try!(stream.read_i16());
    try!(stream.read_i16()); // unknown
    command.selection_mode = try!(stream.read_i8());
    command.right_click_mode = try!(stream.read_i8());
    try!(stream.read_u8()); // unknown
    command.tool_graphic_id = optional_id!(try!(stream.read_i16()));
    command.proceeding_graphic_id = optional_id!(try!(stream.read_i16()));
    command.action_graphic_id = optional_id!(try!(stream.read_i16()));
    command.carrying_graphic_id = optional_id!(try!(stream.read_i16()));
    command.execution_sound_id = optional_id!(try!(stream.read_i16()));
    command.resource_deposit_sound_id = optional_id!(try!(stream.read_i16()));
    Ok(command)
}

fn read_battle_params<R: Read>(stream: &mut R) -> Result<BattleParams> {
    let mut params: BattleParams = Default::default();
    params.default_armor = try!(stream.read_u8());

    let attack_count = try!(stream.read_u16()) as usize;
    params.attacks = try!(stream.read_array(attack_count, |c| -> Result<(i16, i16)> {
        Ok((try!(c.read_i16()), try!(c.read_i16())))
    }));

    let armor_count = try!(stream.read_u16()) as usize;
    params.armors = try!(stream.read_array(armor_count, |c| -> Result<(i16, i16)> {
        Ok((try!(c.read_i16()), try!(c.read_i16())))
    }));

    params.terrain_restriction_for_damage_multiplier = try!(stream.read_i16());
    params.max_range = try!(stream.read_f32());
    params.blast_width = try!(stream.read_f32());
    params.reload_time = try!(stream.read_f32());
    params.projectile_unit_id = optional_id!(try!(stream.read_i16()));
    params.accuracy_percent = try!(stream.read_i16());
    params.tower_mode = try!(stream.read_i8());
    params.frame_delay = try!(stream.read_i16());
    for i in 0..3 {
        params.graphic_displacements[i] = try!(stream.read_f32());
    }
    params.blast_attack_level = try!(stream.read_i8());
    params.min_range = try!(stream.read_f32());
    params.attack_graphic_id = optional_id!(try!(stream.read_i16()));
    params.displayed_melee_armour = try!(stream.read_i16());
    params.displayed_attack = try!(stream.read_i16());
    params.displayed_range = try!(stream.read_f32());
    params.displayed_reload_time = try!(stream.read_f32());
    Ok(params)
}

fn read_projectile_params<R: Read>(stream: &mut R) -> Result<ProjectileParams> {
    let mut params: ProjectileParams = Default::default();
    params.stretch_mode = try!(stream.read_i8());
    params.smart_mode = try!(stream.read_i8());
    params.drop_animation_mode = try!(stream.read_i8());
    params.penetration_mode = try!(stream.read_i8());
    try!(stream.read_u8()); // unknown
    params.projectile_arc = try!(stream.read_f32());
    Ok(params)
}

fn read_trainable_params<R: Read>(stream: &mut R) -> Result<TrainableParams> {
    let mut params: TrainableParams = Default::default();
    params.resource_costs = read_resource_costs!(i16, i16, stream, 3);
    params.train_time = try!(stream.read_i16());
    params.train_location_id = optional_id!(try!(stream.read_i16()));
    params.button_id = try!(stream.read_i8());
    params.displayed_pierce_armor = try!(stream.read_i16());
    Ok(params)
}

fn read_building_params<R: Read>(stream: &mut R) -> Result<BuildingParams> {
    let mut params: BuildingParams = Default::default();
    params.construction_graphic_id = required_id!(try!(stream.read_i16()));
    params.adjacent_mode = try!(stream.read_i8());
    params.graphics_angle = try!(stream.read_i16());
    params.disappears_when_built = try!(stream.read_u8()) != 0;
    params.stack_unit_id = optional_id!(try!(stream.read_i16()));
    params.foundation_terrain_id = optional_id!(try!(stream.read_i16()));
    params.old_terrain_id = optional_id!(try!(stream.read_i16()));
    params.research_id = optional_id!(try!(stream.read_i16()));
    params.construction_sound = try!(stream.read_i16());
    Ok(params)
}
