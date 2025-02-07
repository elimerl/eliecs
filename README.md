# eliecs

My ECS library.

```rust
use eliecs::{components, ECS};

components! {
	struct CPosition {
		pub x: f32,
		pub y: f32,
		pub z: f32
	}
	struct CName(pub String);
}

fn main() {
	let mut ecs = ECS::new();
	ecs.spawn(FatEntity::new().position(CPosition { x: 0.0, y: 0.0, z: 0.0 }).name(CName("what up")));

	for (e, pos) in ecs.query_position() {
		if let Some(name) = ecs.name(e) {
			println!("entity {} with name {} at {} {} {}", e, name.0, pos.x, pos.y, pos.z);
		} else {
			println!("entity {} at {} {} {}", e, pos.x, pos.y, pos.z);
		}
	}
}
```
