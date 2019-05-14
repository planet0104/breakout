use nphysics_testbed2d::Testbed;

fn test(){
    let mut world = World::new();

    // let transform = |x:f32, y:f32, width:f32, height:f32| -> (f32, f32, f32, f32){
    //     // (x+width/2.0, -y-height/2.0, width/2.0, height/2.0)
    //     (x, y, width, height)
    // };

    //创建墙体
    let add_wall = |world: &mut World<f32>, x:f32, y:f32, width:f32, height:f32|{
        let top_wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(width, height)));
        ColliderDesc::new(top_wall_shape)
        .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
        .translation(Vector2::new(x, y))
        .build(world);
    };

    //x 中心点, y: 中心点, width: 半径, height: 半径
    add_wall(&mut world, 160., 416.0-8.0, 160.0, 8.0);//顶部
    add_wall(&mut world, 160., 0.0, 160.0, 8.0);//底部
    add_wall(&mut world, 8.0, 176.0+48.0, 8.0, 176.0); //左边
    add_wall(&mut world, 320.0-8.0, 176.0+48.0, 8.0, 176.0); //左边

    add_wall(&mut world, 8.0, 16.0, 8.0, 16.0);//左下
    add_wall(&mut world, 320.0-8.0, 16.0, 8.0, 16.0);//右下

    add_wall(&mut world, -8.0, 40.0, 8.0, 10.0);//左边堵口
    add_wall(&mut world, 320.0+8.0, 40.0, 8.0, 10.0);//右边堵口

    let x = WINDOW_WIDTH as f32/2.0;
    let y = 40.0;
    let x_radius = 24.;
    let y_radius = 8.;

    let cuboid = ShapeHandle::new(Cuboid::new(Vector2::new(x_radius, y_radius)));
    ColliderDesc::new(cuboid)
        .translation(Vector2::new(x, y))
        .density(1.0)
        .build(&mut world);

    
    let cuboid = ShapeHandle::new(NBall::new(x_radius));
    ColliderDesc::new(cuboid)
        .translation(Vector2::new(x, y-y_radius*2.0))
        .density(1.0)
        .build(&mut world);


    //创建一个球
    // let capsule = Capsule::new(10., 80.);
    // let paddle = ShapeHandle::new(capsule);
    // ColliderDesc::new(paddle)
    //     // .rotation(PI as f32/2.0)
    //     .translation(Vector2::new(150.0, 150.0))
    //     .density(1.0)
    //     .build(&mut world);

    // let capsule = Capsule::new(24., 8.);
    // let paddle = ShapeHandle::new(capsule);
    // ColliderDesc::new(paddle)
    //     // .rotation(PI as f32/2.0)
    //     .translation(Vector2::new(150.0, 150.0))
    //     .density(1.0)
    //     .build(&mut world);

    // let handle1 = RigidBodyDesc::new()
    // // .velocity(Velocity::between_positions(&Isometry2::new(Vector2::new(0.0, 0.0), 3.1415), &Isometry2::new(Vector2::new(3.0, 3.0), 3.1415), 1.0/60.0))
    // // .status(BodyStatus::Static)
    // // .position(Isometry2::new(Vector2::new(0.0, 0.0), 3.1415))
    // .collider(&collider_ball)
    // .build(&mut world).handle();

    // //创建一个球
    // let (x, y, width, _height) = transform(50.0, 250.0, 16.0, 16.0);
    // let ball = ShapeHandle::new(NBall::new(width));
    // let collider_ball = ColliderDesc::new(ball)
    //     // .material(MaterialHandle::new(BasicMaterial::new(1.0, 3.0)))
    //     .translation(Vector2::new(x, y))
    //     // .position(Isometry2::new(Vector2::new(60.0, -60.0), 0.0))
    //     .density(1.0);

    // let handle = RigidBodyDesc::new()
    // // .velocity(Velocity::linear(20.0, 20.0))
    // // .velocity(Velocity::between_positions(&Isometry2::new(Vector2::new(0.0, 0.0), 3.1415), &Isometry2::new(Vector2::new(3.0, 3.0), 3.1415), 1.0/60.0))
    // // .status(BodyStatus::Static)
    // // .position(Isometry2::new(Vector2::new(0.0, 0.0), 3.1415))
    // .collider(&collider_ball)
    // .build(&mut world).handle();

    // let body = world.collider_world().body_colliders(handle).next().unwrap();
    // let body1 = world.collider_world().body_colliders(handle1).next().unwrap();

    // if let ColliderAnchor::OnBodyPart { body_part, .. } = body.anchor(){
    //     let body_part0 = body_part;
    //     if let ColliderAnchor::OnBodyPart { body_part, .. } = body1.anchor(){
    //         let body_part1 = body_part;
    //         let joint = MouseConstraint::new(
    //             *body_part0,
    //             *body_part1,
    //             Point2::new(0.0f32, 0.0),
    //             Point2::new(0.0f32, 0.0),
    //             1.0,
    //         );
    //         let constraint_handle = world.add_constraint(joint);
    //         println!("constraint_handle={:?}", constraint_handle);
    //     }
    // }
    

    // 写几个固定的锚点  对比测试
    // add_point(&mut world, 0., 0., 20.); //直径40
    // add_point(&mut world, 320., 416., 8.); //直径16
    // add_point(&mut world, 0., 416., 8.); //直径16
    // add_point(&mut world, 320., 0., 8.); //直径16

    //320x416
    let mut testbed = Testbed::new(world);
    testbed.look_at(Point2::new(0., 0.), 0.5);//左上方移动
    testbed.run();
}


fn add_point(world: &mut World<f32>, x:f32, y:f32, size:f32){
    ColliderDesc::new(ShapeHandle::new(NBall::new(size)))
        .translation(Vector2::new(x, y))//y值要负值
        .build(world);
}
