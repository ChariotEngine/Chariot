//
// OpenAOE: An open source reimplementation of Age of Empires (1997)
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

use empires::EmpiresDb;
use error::*;

use io_tools::*;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct ResourceStorage {
    type_id: i16,
    amount: f32,
    enabled: bool,
}

impl ResourceStorage {
    pub fn new() -> ResourceStorage {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ResourceCost {
    type_id: i16,
    amount: i16,
    enabled: bool,
}

#[derive(Default, Debug)]
pub struct DamageGraphic {
    graphic_id: i16,
    damage_percent: u8,
    old_apply_mode: u8,
    apply_mode: u8,
}

impl DamageGraphic {
    pub fn new() -> DamageGraphic {
        Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnitType {
    GraphicEffect,
    Flag,
    Unknown25,
    DeadFish,
    Bird,
    Unknown50,
    Projectile,
    Trainable,
    Building,
    Tree
}

impl Default for UnitType {
    fn default() -> UnitType {
        UnitType::Unknown25
    }
}

impl UnitType {
    pub fn from_u8(val: u8) -> EmpiresDbResult<UnitType> {
        use self::UnitType::*;
        match val {
            10 => Ok(GraphicEffect),
            20 => Ok(Flag),
            25 => Ok(Unknown25),
            30 => Ok(DeadFish),
            40 => Ok(Bird),
            50 => Ok(Unknown50),
            60 => Ok(Projectile),
            70 => Ok(Trainable),
            80 => Ok(Building),
            90 => Ok(Tree),
            _ => Err(EmpiresDbError::InvalidUnitType(val)),
        }
    }

    pub fn has_dead_fish_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            DeadFish | Bird | Unknown50 | Projectile | Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_bird_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Bird | Unknown50 | Projectile | Trainable | Building | Tree => true,
            _ => false,
        }
    }

    pub fn has_unknown50_params(&self) -> bool {
        use self::UnitType::*;
        match *self {
            Unknown50 | Projectile | Trainable | Building | Tree => true,
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

#[derive(Default, Debug)]
pub struct UnitCommand {
    enabled: bool,
    id: i16,
    type_id: i16,
    class_id: i16,
    unit_id: i16,
    terrain_id: i16,
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
    tool_graphic_id: i16,
    proceeding_graphic_id: i16,
    action_graphic_id: i16,
    carrying_graphic_id: i16,
    execution_sound_id: i16,
    resource_deposit_sound_id: i16,
}

#[derive(Default, Debug)]
pub struct DeadFishParams {
    walking_graphics: [i16; 2],
    rotation_speed: f32,
    tracking_unit: i16,
    tracking_unit_used: bool,
    tracking_unit_density: f32,
}

#[derive(Default, Debug)]
pub struct BirdParams {
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

#[derive(Default, Debug)]
pub struct Unknown50Params {
    default_armor: u8,
    attacks: Vec<(i16, i16)>, // class, amount
    armors: Vec<(i16, i16)>, // class, amount
    terrain_restriction_for_damage_multiplier: i16,
    max_range: f32,
    blast_width: f32,
    reload_time: f32,
    projectile_unit_id: i16,
    accuracy_percent: i16,
    tower_mode: i8,
    frame_delay: i16,
    graphic_displacements: [f32; 3],
    blast_attack_level: i8,
    min_range: f32,
    attack_graphic: i16,
    displayed_melee_armour: i16,
    displayed_attack: i16,
    displayed_range: f32,
    displayed_reload_time: f32,
}

#[derive(Default, Debug)]
pub struct BuildingParams {
    construction_graphic_id: i16,
    adjacent_mode: i8,
    graphics_angle: i16,
    disappears_when_built: bool,
    stack_unit_id: i16,
    foundation_terrain_id: i16,
    old_terrain_id: i16,
    research_id: i16,
    construction_sound: i16,
}

#[derive(Default, Debug)]
pub struct ProjectileParams {
    stretch_mode: i8,
    smart_mode: i8,
    drop_animation_mode: i8,
    penetration_mode: i8,
    projectile_arc: f32,
}

#[derive(Default, Debug)]
pub struct TrainableParams {
    resource_costs: Vec<ResourceCost>,
    train_time: i16,
    train_location_id: i16,
    button_id: i8,
    displayed_pierce_armor: i16,
}

#[derive(Default, Debug)]
pub struct Unit {
    unit_type: UnitType,
    name: String,
    id: i16,
    language_dll_name: u16,
    language_dll_creation: u16,
    class: i16,
    standing_graphic: i16,
    dying_graphics: [i16; 2],
    death_mode: i8,
    hit_points: i16,
    line_of_sight: f32,
    garrison_capability: i8,
    collision_size_x: f32,
    collision_size_y: f32,
    collision_size_z: f32,
    train_sound: i16,
    dead_unit_id: i16,
    placement_mode: i8,
    air_mode: i8,
    icon_id: i16,
    hide_in_editor: bool,
    enabled: bool,

    placement_side_terrain: [i16; 2],
    placement_terrain: [i16; 2],
    clearance_size_x: f32,
    clearance_size_y: f32,
    hill_mode: i8,
    visible_in_fog: bool,
    terrain_restriction: i16,
    fly_mode: i8,
    resource_capacity: i16,
    resource_decay: f32,
    blast_defense_level: i8,
    sub_type: i8,
    interaction_mode: i8,
    minimap_mode: i8,
    command_attribute: i8,
    minimap_color: u8,
    language_dll_help: i32,
    language_dll_hotkey_text: i32,
    hotkey: i32,
    unselectable: bool,
    enable_auto_gather: bool,
    auto_gather_mode: i8,
    auto_gather_id: i8,

    selection_effect: i8,
    editor_selection_color: u8,
    selection_shape_size_x: f32,
    selection_shape_size_y: f32,
    selection_shape_size_z: f32,

    resource_storage: Vec<ResourceStorage>,
    damage_graphics: Vec<DamageGraphic>,

    selection_sound: i16,
    dying_sound: i16,
    attack_mode: i8,

    id2: i16, // TODO: wtf?

    speed: f32,
    dead_fish: DeadFishParams,
    bird: BirdParams,
    unknown50: Unknown50Params,
    projectile: ProjectileParams,
    trainable: TrainableParams,
    building: BuildingParams,
}

impl Unit {
    pub fn new() -> Unit {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct Civilization {
    enabled: bool,
    name: String,
    resources: Vec<f32>,
    tech_tree_id: i16,
    icon_set: i8,
    units: Vec<Unit>,
}

impl Civilization {
    pub fn new() -> Civilization {
        Default::default()
    }
}

impl EmpiresDb {
    pub fn read_civs<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let civ_count = try!(cursor.read_u16()) as usize;
        self.civilizations = try!(cursor.read_array(civ_count, |c| EmpiresDb::read_civ(c)));
        Ok(())
    }

    fn read_civ<R: Read + Seek>(cursor: &mut R) -> EmpiresDbResult<Civilization> {
        let mut civ = Civilization::new();
        civ.enabled = try!(cursor.read_byte()) != 0;
        civ.name = try!(cursor.read_sized_str(20));

        let resource_count = try!(cursor.read_u16()) as usize;
        civ.tech_tree_id = try!(cursor.read_i16());
        civ.resources = try!(cursor.read_array(resource_count, |c| c.read_f32()));
        civ.icon_set = try!(cursor.read_byte()) as i8;

        let unit_count = try!(cursor.read_u16()) as usize;
        let unit_pointers = try!(cursor.read_array(unit_count, |c| c.read_i32()));
        for i in 0..unit_count {
            if unit_pointers[i] != 0 {
                civ.units.push(try!(EmpiresDb::read_unit(cursor)));
            }
        }
        Ok(civ)
    }

    fn read_unit<R: Read + Seek>(cursor: &mut R) -> EmpiresDbResult<Unit> {
        let mut unit = Unit::new();

        unit.unit_type = try!(UnitType::from_u8(try!(cursor.read_byte())));
        let name_length = try!(cursor.read_u16()) as usize;
        unit.id = try!(cursor.read_i16());
        unit.language_dll_name = try!(cursor.read_u16());
        unit.language_dll_creation = try!(cursor.read_u16());
        unit.class = try!(cursor.read_i16());
        unit.standing_graphic = try!(cursor.read_i16());
        unit.dying_graphics[0] = try!(cursor.read_i16());
        unit.dying_graphics[1] = try!(cursor.read_i16());
        unit.death_mode = try!(cursor.read_byte()) as i8;
        unit.hit_points = try!(cursor.read_i16());
        unit.line_of_sight = try!(cursor.read_f32());
        unit.garrison_capability = try!(cursor.read_byte()) as i8;
        unit.collision_size_x = try!(cursor.read_f32());
        unit.collision_size_y = try!(cursor.read_f32());
        unit.collision_size_z = try!(cursor.read_f32());
        unit.train_sound = try!(cursor.read_i16());
        unit.dead_unit_id = try!(cursor.read_i16());
        unit.placement_mode = try!(cursor.read_byte()) as i8;
        unit.air_mode = try!(cursor.read_byte()) as i8;
        unit.icon_id = try!(cursor.read_i16());
        unit.hide_in_editor = try!(cursor.read_byte()) != 0;
        try!(cursor.read_u16()); // unknown
        unit.enabled = try!(cursor.read_byte()) != 0;

        unit.placement_side_terrain[0] = try!(cursor.read_i16());
        unit.placement_side_terrain[1] = try!(cursor.read_i16());
        unit.placement_terrain[0] = try!(cursor.read_i16());
        unit.placement_terrain[1] = try!(cursor.read_i16());
        unit.clearance_size_x = try!(cursor.read_f32());
        unit.clearance_size_y = try!(cursor.read_f32());
        unit.hill_mode = try!(cursor.read_byte()) as i8;
        unit.visible_in_fog = try!(cursor.read_byte()) != 0;
        unit.terrain_restriction = try!(cursor.read_i16());
        unit.fly_mode = try!(cursor.read_byte()) as i8;
        unit.resource_capacity = try!(cursor.read_i16());
        unit.resource_decay = try!(cursor.read_f32());
        unit.blast_defense_level = try!(cursor.read_byte()) as i8;
        unit.sub_type = try!(cursor.read_byte()) as i8;
        unit.interaction_mode = try!(cursor.read_byte()) as i8;
        unit.minimap_mode = try!(cursor.read_byte()) as i8;
        unit.command_attribute = try!(cursor.read_byte()) as i8;
        try!(cursor.read_f32()); // unknown
        unit.minimap_color = try!(cursor.read_byte());
        unit.language_dll_help = try!(cursor.read_i32());
        unit.language_dll_hotkey_text = try!(cursor.read_i32());
        unit.hotkey = try!(cursor.read_i32());
        unit.unselectable = try!(cursor.read_byte()) != 0;
        unit.enable_auto_gather = try!(cursor.read_byte()) != 0;
        unit.auto_gather_mode = try!(cursor.read_byte()) as i8;
        unit.auto_gather_id = try!(cursor.read_byte()) as i8;

        unit.selection_effect = try!(cursor.read_byte()) as i8;
        unit.editor_selection_color = try!(cursor.read_byte()) as u8;
        unit.selection_shape_size_x = try!(cursor.read_f32());
        unit.selection_shape_size_y = try!(cursor.read_f32());
        unit.selection_shape_size_z = try!(cursor.read_f32());

        unit.resource_storage = try!(cursor.read_array(3, |c| EmpiresDb::read_resource_storage(c)));

        let damage_graphic_count = try!(cursor.read_byte()) as usize;
        unit.damage_graphics = try!(cursor.read_array(damage_graphic_count, |c| EmpiresDb::read_damage_graphic(c)));

        unit.selection_sound = try!(cursor.read_i16());
        unit.dying_sound = try!(cursor.read_i16());
        unit.attack_mode = try!(cursor.read_byte()) as i8;
        try!(cursor.read_byte()); // Unknown

        unit.name = try!(cursor.read_sized_str(name_length));
        unit.id2 = try!(cursor.read_i16());

        if let UnitType::Tree = unit.unit_type {
            return Ok(unit)
        }

        if let UnitType::GraphicEffect = unit.unit_type {
            return Ok(unit)
        }

        unit.speed = try!(cursor.read_f32());
        if unit.unit_type.has_dead_fish_params() {
            unit.dead_fish = try!(EmpiresDb::read_dead_fish_params(cursor));
        }
        if unit.unit_type.has_bird_params() {
            unit.bird = try!(EmpiresDb::read_bird_params(cursor));
        }
        if unit.unit_type.has_unknown50_params() {
            unit.unknown50 = try!(EmpiresDb::read_unknown50_params(cursor));
        }
        if unit.unit_type.has_projectile_params() {
            unit.projectile = try!(EmpiresDb::read_projectile_params(cursor));
        }
        if unit.unit_type.has_trainable_params() {
            unit.trainable = try!(EmpiresDb::read_trainable_params(cursor));
        }
        if unit.unit_type.has_building_params() {
            unit.building = try!(EmpiresDb::read_building_params(cursor));
        }

        Ok(unit)
    }

    fn read_damage_graphic<R: Read>(cursor: &mut R) -> EmpiresDbResult<DamageGraphic> {
        let mut damage_graphic = DamageGraphic::new();
        damage_graphic.graphic_id = try!(cursor.read_i16());
        damage_graphic.damage_percent = try!(cursor.read_byte());
        damage_graphic.old_apply_mode = try!(cursor.read_byte());
        damage_graphic.apply_mode = try!(cursor.read_byte());
        Ok(damage_graphic)
    }

    fn read_resource_storage<R: Read>(cursor: &mut R) -> EmpiresDbResult<ResourceStorage> {
        let mut storage = ResourceStorage::new();
        storage.type_id = try!(cursor.read_i16());
        storage.amount = try!(cursor.read_f32());
        storage.enabled = try!(cursor.read_byte()) != 0;
        Ok(storage)
    }

    fn read_dead_fish_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<DeadFishParams> {
        let mut dead_fish: DeadFishParams = Default::default();
        dead_fish.walking_graphics[0] = try!(cursor.read_i16());
        dead_fish.walking_graphics[1] = try!(cursor.read_i16());
        dead_fish.rotation_speed = try!(cursor.read_f32());
        try!(cursor.read_byte()); // unknown
        dead_fish.tracking_unit = try!(cursor.read_i16());
        dead_fish.tracking_unit_used = try!(cursor.read_byte()) != 0;
        dead_fish.tracking_unit_density = try!(cursor.read_f32());
        try!(cursor.read_byte()); // unknown
        Ok(dead_fish)
    }

    fn read_bird_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<BirdParams> {
        let mut bird: BirdParams = Default::default();
        bird.action_when_discovered_id = try!(cursor.read_i16());
        bird.search_radius = try!(cursor.read_f32());
        bird.work_rate = try!(cursor.read_f32());
        bird.drop_sites[0] = try!(cursor.read_i16());
        bird.drop_sites[1] = try!(cursor.read_i16());
        bird.task_swap_id = try!(cursor.read_byte()) as i8;
        bird.attack_sound = try!(cursor.read_i16());
        bird.move_sound = try!(cursor.read_i16());
        bird.animal_mode = try!(cursor.read_byte()) as i8;

        let command_count = try!(cursor.read_u16()) as usize;
        bird.commands = try!(cursor.read_array(command_count, |c| EmpiresDb::read_unit_command(c)));
        Ok(bird)
    }

    fn read_unit_command<R: Read>(cursor: &mut R) -> EmpiresDbResult<UnitCommand> {
        let mut command: UnitCommand = Default::default();
        command.enabled = try!(cursor.read_u16()) != 0;
        command.id = try!(cursor.read_i16());
        try!(cursor.read_byte()); // unknown
        command.type_id = try!(cursor.read_i16());
        command.class_id = try!(cursor.read_i16());
        command.unit_id = try!(cursor.read_i16());
        command.terrain_id = try!(cursor.read_i16());
        command.resource_in = try!(cursor.read_i16());
        command.resource_productivity_multiplier = try!(cursor.read_i16());
        command.resource_out = try!(cursor.read_i16());
        command.resource = try!(cursor.read_i16());
        command.quantity = try!(cursor.read_f32());
        command.execution_radius = try!(cursor.read_f32());
        command.extra_range = try!(cursor.read_f32());
        try!(cursor.read_byte()); // unknown
        try!(cursor.read_f32()); // unknown
        command.selection_enabler = try!(cursor.read_byte()) as i8;
        try!(cursor.read_byte()); // unknown
        command.plunder_source = try!(cursor.read_i16());
        try!(cursor.read_i16()); // unknown
        command.selection_mode = try!(cursor.read_byte()) as i8;
        command.right_click_mode = try!(cursor.read_byte()) as i8;
        try!(cursor.read_byte()); // unknown
        command.tool_graphic_id = try!(cursor.read_i16());
        command.proceeding_graphic_id = try!(cursor.read_i16());
        command.action_graphic_id = try!(cursor.read_i16());
        command.carrying_graphic_id = try!(cursor.read_i16());
        command.execution_sound_id = try!(cursor.read_i16());
        command.resource_deposit_sound_id = try!(cursor.read_i16());
        Ok(command)
    }

    fn read_unknown50_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<Unknown50Params> {
        let mut params: Unknown50Params = Default::default();
        params.default_armor = try!(cursor.read_byte());

        let attack_count = try!(cursor.read_u16()) as usize;
        params.attacks = try!(cursor.read_array(attack_count, |c| -> EmpiresDbResult<(i16, i16)> {
            Ok((try!(c.read_i16()), try!(c.read_i16())))
        }));

        let armor_count = try!(cursor.read_u16()) as usize;
        params.armors = try!(cursor.read_array(armor_count, |c| -> EmpiresDbResult<(i16, i16)> {
            Ok((try!(c.read_i16()), try!(c.read_i16())))
        }));

        params.terrain_restriction_for_damage_multiplier = try!(cursor.read_i16());
        params.max_range = try!(cursor.read_f32());
        params.blast_width = try!(cursor.read_f32());
        params.reload_time = try!(cursor.read_f32());
        params.projectile_unit_id = try!(cursor.read_i16());
        params.accuracy_percent = try!(cursor.read_i16());
        params.tower_mode = try!(cursor.read_byte()) as i8;
        params.frame_delay = try!(cursor.read_i16());
        for i in 0..3 {
            params.graphic_displacements[i] = try!(cursor.read_f32());
        }
        params.blast_attack_level = try!(cursor.read_byte()) as i8;
        params.min_range = try!(cursor.read_f32());
        params.attack_graphic = try!(cursor.read_i16());
        params.displayed_melee_armour = try!(cursor.read_i16());
        params.displayed_attack = try!(cursor.read_i16());
        params.displayed_range = try!(cursor.read_f32());
        params.displayed_reload_time = try!(cursor.read_f32());
        Ok(params)
    }

    fn read_projectile_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<ProjectileParams> {
        let mut params: ProjectileParams = Default::default();
        params.stretch_mode = try!(cursor.read_byte()) as i8;
        params.smart_mode = try!(cursor.read_byte()) as i8;
        params.drop_animation_mode = try!(cursor.read_byte()) as i8;
        params.penetration_mode = try!(cursor.read_byte()) as i8;
        try!(cursor.read_byte()) as i8; // unknown
        params.projectile_arc = try!(cursor.read_f32());
        Ok(params)
    }

    fn read_trainable_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<TrainableParams> {
        let mut params: TrainableParams = Default::default();
        for _ in 0..3 {
            let mut cost: ResourceCost = Default::default();
            cost.type_id = try!(cursor.read_i16());
            cost.amount = try!(cursor.read_i16());
            cost.enabled = try!(cursor.read_i16()) != 0;
            params.resource_costs.push(cost);
        }
        params.train_time = try!(cursor.read_i16());
        params.train_location_id = try!(cursor.read_i16());
        params.button_id = try!(cursor.read_byte()) as i8;
        params.displayed_pierce_armor = try!(cursor.read_i16());
        Ok(params)
    }

    fn read_building_params<R: Read>(cursor: &mut R) -> EmpiresDbResult<BuildingParams> {
        let mut params: BuildingParams = Default::default();
        params.construction_graphic_id = try!(cursor.read_i16());
        params.adjacent_mode = try!(cursor.read_byte()) as i8;
        params.graphics_angle = try!(cursor.read_i16());
        params.disappears_when_built = try!(cursor.read_byte()) != 0;
        params.stack_unit_id = try!(cursor.read_i16());
        params.foundation_terrain_id = try!(cursor.read_i16());
        params.old_terrain_id = try!(cursor.read_i16());
        params.research_id = try!(cursor.read_i16());
        params.construction_sound = try!(cursor.read_i16());
        Ok(params)
    }
}
