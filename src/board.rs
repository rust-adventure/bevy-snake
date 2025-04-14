use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};

use crate::assets::ImageAssets;

#[derive(Component)]
pub struct SnakeLayer;

pub fn spawn_board(
    mut commands: Commands,
    images: Res<ImageAssets>,
) -> Result {
    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights)?;

    let map_size = TilemapSize { x: 20, y: 20 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(
                        dist.sample(&mut rng) as u32,
                    ),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 136., y: 136. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(
            images.grass.clone(),
        ),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..default()
    });

    // snake layer
    let tilemap_entity = commands.spawn_empty().id();
    let tile_storage = TileStorage::empty(map_size);
    let tile_size = TilemapTileSize { x: 136.0, y: 136.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(
                images.snake.clone(),
            ),
            tile_size,
            anchor: TilemapAnchor::Center,
            ..default()
        },
        SnakeLayer,
    ));

    Ok(())
}
