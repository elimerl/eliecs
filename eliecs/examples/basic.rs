use std::{
    cell::{RefCell, UnsafeCell},
    num::NonZeroU32,
};

use eliecs::{components, Entity, Pool};

// components! {
#[derive(Debug)]
struct CPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug)]
struct CName(pub String);
// }

enum ComponentType {
    CPosition,
    CName,
}

// // struct ECS {
// //     position: Pool<CPosition>,
// //     name: Pool<CName>,
// // }

#[derive(Default, Debug)]
struct FatEntity {
    pub position: Option<CPosition>,
    pub name: Option<CName>,
}

impl FatEntity {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn position(mut self, v: CPosition) -> Self {
        self.position = Some(v);
        self
    }
    pub fn name(mut self, v: CName) -> Self {
        self.name = Some(v);
        self
    }
}

struct ECS {
    existence: Pool<std::num::NonZeroU32>,
    free_list: Vec<Entity>,
    position: UnsafeCell<Pool<CPosition>>,
    position_borrowed_mut: bool,
    name: UnsafeCell<Pool<CName>>,
    name_borrowed_mut: bool,
}
impl ECS {
    pub fn new() -> Self {
        Self {
            existence: Pool::new(),
            free_list: Vec::new(),
            position: Pool::new().into(),
            position_borrowed_mut: false,
            name: Pool::new().into(),
            name_borrowed_mut: false,
        }
    }
    pub fn alive(&self, e: eliecs::Entity) -> bool {
        self.existence.get(e.id).copied() == Some(e.version)
    }
    pub fn get_entity_from_id(&self, id: u32) -> Option<std::num::NonZeroU32> {
        self.existence.get(id).copied()
    }
    pub fn spawn(&mut self, data: FatEntity) -> eliecs::Entity {
        let e: eliecs::Entity;
        if let Some(v) = self.free_list.pop() {
            e = v;
        } else {
            e = eliecs::Entity::new(self.existence.len(), std::num::NonZeroU32::MIN);
        }
        self.existence.insert(e.id, e.version);

        if let Some(v) = data.position {
            self.position.get_mut().insert(e.id, v);
        }

        if let Some(v) = data.name {
            self.name.get_mut().insert(e.id, v);
        }

        e
    }

    pub fn despawn(&mut self, e: eliecs::Entity) {
        self.existence.remove(e.id);

        self.position.get_mut().remove(e.id);
        self.name.get_mut().remove(e.id);

        let mut v = e;
        v.version = if let Some(v) = v.version.checked_add(1) {
            v
        } else {
            std::num::NonZeroU32::MIN
        };
        // if let Some(d) = self.destroyed {
        //     self.entities[e.id as usize].id = d.id;
        // } else {
        //     self.entities[e.id as usize].id = u32::MAX;
        // }
        // self.destroyed = Some(e);
    }

    pub fn position(&self, id: u32) -> Option<&CPosition> {
        unsafe { (*(self.position.get())).get(id) }
    }

    pub fn position_mut(&self, id: u32) -> Option<&mut CPosition> {
        unsafe { (*(self.position.get())).get_mut(id) }
    }

    pub fn query_position(&self) -> impl Iterator<Item = (u32, &CPosition)> {
        unsafe { &mut *(self.position.get()) }.iter()
    }
    pub fn query_position_mut(&self) -> impl Iterator<Item = (u32, &mut CPosition)> {
        unsafe { &mut *(self.position.get()) }.iter_mut()
    }
}

fn main() {
    dbg!(CName("Hello".into()));
    let mut ecs = ECS::new();
    ecs.spawn(FatEntity::new().position(CPosition {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));
    let e = ecs.spawn(
        FatEntity::new()
            .position(CPosition {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            })
            .name(CName("hello world".to_string())),
    );
    // dbg!(ecs.destroyed);
    for (id, v) in ecs.query_position_mut() {
        let pos_again = ecs.position_mut(id);
        dbg!(v, pos_again);
    }

    ecs.despawn(e);

    dbg!(ecs.spawn(FatEntity::new()));
    dbg!(ecs.spawn(FatEntity::new()));
    dbg!(ecs.spawn(FatEntity::new()));
}
