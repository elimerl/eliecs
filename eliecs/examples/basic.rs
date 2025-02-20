use std::{
    cell::{RefCell, UnsafeCell},
    num::NonZeroU32,
    time::Instant,
};

use bincode::Options;
use eliecs::components;

components! {
    #[derive(Debug, Serialize, Deserialize)]
    struct CPosition {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    #[derive(Debug, Serialize, Deserialize)]
        struct CName(pub String);

    #[derive(Debug, Serialize, Deserialize)]
    struct CRot(pub f32);
}

fn main() {
    dbg!(CName("Hello".into()));
    let mut ecs = Ecs::new();
    ecs.spawn(
        FatEntity::new()
            .position(CPosition {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            })
            .rot(CRot(0.0)),
    );
    let e = ecs.spawn(
        FatEntity::new()
            .position(CPosition {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            })
            .name(CName("hello world".to_string())),
    );
    dbg!(ecs.rot(e.id));
    ecs.add_rot(e.id, CRot(100.0));
    dbg!(ecs.rot(e.id));
    // ecs.remove_rot(e.id);
    dbg!(ecs.rot(e.id));

    // dbg!(ecs.destroyed);
    for (id, v) in ecs.query_position_mut() {
        v.x += 1.0;
    }

    // ecs.despawn(e);

    // ecs.despawn(dbg!(e));
    let e = ecs.spawn(FatEntity::new().name(CName("regina george".to_string())));
    ecs.spawn(FatEntity::new().name(CName("vegina george".to_string())));
    ecs.spawn(FatEntity::new().name(CName("vegina george".to_string())));
    ecs.spawn(FatEntity::new().name(CName("vegina george".to_string())));

    ecs.despawn(e);
    // for i in 0..100 {
    //     (ecs.spawn(FatEntity::new().position(CPosition {
    //         x: 0.0,
    //         y: 1.0,
    //         z: 10.0,
    //     })));
    // }
    let e = ecs.spawn(FatEntity::new().name(CName("name at the end".to_string())));
    ecs.despawn(e);

    println!("{}", serde_json::to_string_pretty(&ecs).unwrap());

    println!(
        "{}",
        bincode::DefaultOptions::new()
            .serialize(&ecs)
            .unwrap()
            .len()
    );
    let time = Instant::now();
    let compressed =
        lz4_flex::compress(&bincode::DefaultOptions::new().serialize(&ecs).unwrap()).len();
    println!("{}", compressed);
}
