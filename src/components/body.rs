extern crate specs;
use self::specs::*;
use components::*;

// Material of part
#[derive(Debug, Clone)]
pub enum PartMaterial {
    Organic,
    Robotic,
}

/// Type of limb
///     Should eventually contain stats about the limb type
#[derive(Debug, Clone)]
pub enum PartType {
    Head,
    Eyes,
    Ears,
    Nose,
    Mouth,
    Neck,
    Torso,
    Arm,
    Hand,
    Finger,
    Leg,
    Foot,
    Toe,
}

/// A single body part.
/// Also has attached body parts.
///
#[derive(Debug, Clone)]
pub struct BodyPart {
    part_type: PartType,
    material: PartMaterial,
    health: i32,
    max_health: i32,
    attached: Vec<Box<BodyPart>>,
}

impl BodyPart {
    pub fn is_dead(&self) -> bool {
        return self.health <= 0;
    }

    pub fn get_attached(&self) -> &Vec<Box<BodyPart>> {
        return &self.attached;
    }

    pub fn get_attached_mut(&mut self) -> &mut Vec<Box<BodyPart>> {
        return &mut self.attached;
    }

    pub fn set_material(&mut self, material: PartMaterial) { 
        self.material = material;
    }

    pub fn print_repr(&self) {
        if self.attached.len() > 0 {
            print!("{:?} has: ", self.part_type);

            for part in self.attached.iter() {
                print!("{:?}, ", part.part_type);
            }
            println!("");

            for part in self.attached.iter() {
                part.print_repr();
            }
        }
    }
}

pub struct BodyPartBuilder {
    part: BodyPart,
}

impl BodyPartBuilder {
    pub fn new(part_type: PartType) -> Self {
        return Self {
            part: BodyPart {
                part_type: part_type,
                material: PartMaterial::Organic,
                health: 100,
                max_health: 100,
                attached: Vec::new(),
            }
        }
    }

    pub fn attach_part(mut self, part: BodyPart) -> Self {
        self.part.get_attached_mut().push(Box::new(part));
        return self;
    }

    pub fn attach_many_of_part(mut self, part: BodyPart, number: u32) -> Self {
        for n in 0..number {
            self.part.get_attached_mut().push(Box::new(part.clone()));
        }
        return self;
    }

    pub fn clone(mut self) -> BodyPart {
        return self.part.clone();
    }

    pub fn finalize(mut self) -> BodyPart {
        return self.part;
    }
}

/// Body component.
///     Contains a root_part (usually the head)
///     If this part is destroyed the entity is _dead_
#[derive(Debug)]
pub struct BodyComponent {
    pub root_part: BodyPart,
    pub health: i32,
}

impl Component for BodyComponent {
    type Storage = VecStorage<Self>;
}

impl BodyComponent {
    pub fn is_dead(&self) -> bool {
        return self.health <= 0;
        //return self.root_part.is_dead();
    }
}
