# Bevy 2D物理碰撞系统设计方案

## 1. 系统架构

### 1.1 核心组件 (Components)

```rust
// 基础物理属性
#[derive(Component)]
struct PhysicsBody {
    mass: f32,
    restitution: f32,
    friction: f32,
    is_static: bool,
}

// 形状组件
#[derive(Component)]
enum CollisionShape {
    Point(Vec2),
    Line(Vec2, Vec2),
    Circle {
        center: Vec2,
        radius: f32,
    },
    Rectangle {
        center: Vec2,
        size: Vec2,
        rotation: f32,
    },
    Polygon {
        vertices: Vec<Vec2>,
        center: Vec2,
        rotation: f32,
    },
}

// 运动状态
#[derive(Component)]
struct Motion {
    velocity: Vec2,
    angular_velocity: f32,
    acceleration: Vec2,
}

// 碰撞标记
#[derive(Component)]
struct CollisionFlag {
    is_trigger: bool,
    collision_layer: u32,
    collision_mask: u32,
}
```

### 1.2 资源 (Resources)

```rust
// 物理世界配置
#[derive(Resource)]
struct PhysicsConfig {
    gravity: Vec2,
    time_step: f32,
    velocity_iterations: i32,
    position_iterations: i32,
}

// 碰撞矩阵
#[derive(Resource)]
struct CollisionMatrix {
    layer_masks: HashMap<u32, u32>,
}

// 调试配置
#[derive(Resource)]
struct PhysicsDebugConfig {
    show_colliders: bool,
    show_velocity: bool,
    show_contacts: bool,
}
```

### 1.3 事件 (Events)

```rust
// 碰撞事件
#[derive(Event)]
enum CollisionEvent {
    Started(Entity, Entity),
    Ongoing(Entity, Entity),
    Ended(Entity, Entity),
}

// 触发器事件
#[derive(Event)]
enum TriggerEvent {
    Enter(Entity, Entity),
    Stay(Entity, Entity),
    Exit(Entity, Entity),
}
```

### 1.4 系统 (Systems)

```rust
// 物理更新系统组
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum QPhysicsUpdateSet {
    PreUpdate,
    CollisionDetection,
    Resolution,
    PostUpdate,
}

// 主要系统实现
fn physics_update_system(
    mut query: Query<(&PhysicsBody, &mut Motion)>,
    time: Res<Time>,
    config: Res<PhysicsConfig>,
) {
    // 物理状态更新逻辑
}

fn collision_detection_system(
    query: Query<(Entity, &CollisionShape, &CollisionFlag)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // 碰撞检测逻辑
}

fn collision_resolution_system(
    mut query: Query<(&PhysicsBody, &mut Motion)>,
    collision_events: EventReader<CollisionEvent>,
) {
    // 碰撞响应处理
}

fn debug_render_system(
    query: Query<(&CollisionShape, &Transform)>,
    config: Res<PhysicsDebugConfig>,
) {
    // 调试渲染逻辑
}
```

## 2. 使用方式

### 2.1 插件注册

```rust
// 插件实现
pub struct Physics2DPlugin;

impl Plugin for Physics2DPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PhysicsConfig>()
            .init_resource::<CollisionMatrix>()
            .init_resource::<PhysicsDebugConfig>()
            .add_event::<CollisionEvent>()
            .add_event::<TriggerEvent>()
            .configure_sets((
                QPhysicsUpdateSet::PreUpdate,
                QPhysicsUpdateSet::CollisionDetection,
                QPhysicsUpdateSet::Resolution,
                QPhysicsUpdateSet::PostUpdate,
            ).chain())
            .add_systems((
                physics_update_system.in_set(QPhysicsUpdateSet::PreUpdate),
                collision_detection_system.in_set(QPhysicsUpdateSet::CollisionDetection),
                collision_resolution_system.in_set(QPhysicsUpdateSet::Resolution),
                debug_render_system.in_set(QPhysicsUpdateSet::PostUpdate),
            ));
    }
}
```

### 2.2 实体创建示例

```rust
// 创建物理实体
fn spawn_physics_entity(
    commands: &mut Commands,
    shape: CollisionShape,
    physics_props: PhysicsBody,
    position: Vec2,
) {
    commands.spawn((
        shape,
        physics_props,
        Motion::default(),
        CollisionFlag {
            is_trigger: false,
            collision_layer: 1,
            collision_mask: 0xFFFFFFFF,
        },
        Transform::from_translation(position.extend(0.0)),
    ));
}
```

### 2.3 碰撞事件监听

```rust
// 碰撞事件处理系统
fn handle_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<&Transform>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(e1, e2) => {
                // 处理碰撞开始
            }
            CollisionEvent::Ongoing(e1, e2) => {
                // 处理持续碰撞
            }
            CollisionEvent::Ended(e1, e2) => {
                // 处理碰撞结束
            }
        }
    }
}
```

### 2.4 配置更新

```rust
// 更新物理配置
fn update_physics_config(
    mut config: ResMut<PhysicsConfig>,
) {
    config.gravity = Vec2::new(0.0, -9.81);
    config.time_step = 1.0 / 60.0;
}
```
