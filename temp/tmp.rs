use nalgebra::{Point2, Vector3, Vector2, Isometry2};
    use ncollide2d::shape::{Cuboid, ShapeHandle, Ball};
    use ncollide2d::world::CollisionGroups;
    use nphysics2d::object::{BodyHandle, BodyStatus, RigidBodyDesc, ColliderDesc};
    use nphysics2d::material::{MaterialHandle, BasicMaterial};
    use nphysics2d::world::World;
    use nphysics_testbed2d::Testbed;
    use nphysics2d::math::{Velocity, Inertia};
    use std::f32::consts::PI;
    use ncollide2d::events::ContactEvent;
    use std::thread;
    use std::time::Duration;
    use std::rc::Rc;
    use std::cell::RefCell;

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    /*
     * World
     */
    let mut world = World::new();
    world.set_timestep(1.0/60.0);
    // world.set_gravity(Vector2::new(0.0, -9.81));


    //墙面
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(10.0, 0.1)));
    ColliderDesc::new(ground_shape)
    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
    .translation(-Vector2::y()*5.).build(&mut world);

    let top_wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(10.0, 0.1)));
    ColliderDesc::new(top_wall_shape)
    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
    .translation(Vector2::y()*5.).build(&mut world);

    let left_wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(10.0, 0.1)));
    ColliderDesc::new(left_wall_shape)
    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
    .rotation(PI/2.0).translation(-Vector2::x()*5.).build(&mut world);

    let right_wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(10.0, 0.1)));
    ColliderDesc::new(right_wall_shape)
    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
    .rotation(PI/2.0).translation(Vector2::x()*5.).build(&mut world);

    //------------------------------------------------------


    let ball = ShapeHandle::new(Ball::new(0.5));
    let collider_ball = ColliderDesc::new(ball)
        .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
        .density(1.0);

    let rigid_ball_handle = RigidBodyDesc::new()
    .velocity(Velocity::linear(4.0, 7.0))
    .collider(&collider_ball)
    .build(&mut world).handle();

    let mut cuboid_handles = Rc::new(RefCell::new(vec![]));

    for i in -2..3{
        let cuboid = ShapeHandle::new(Cuboid::new(Vector2::new(0.70, 0.25)));

        let collider_cuboid = ColliderDesc::new(cuboid)
            .translation(Vector2::new(i as f32*1.5, 4.0))
            .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
            .density(1.0);

        let rigid_cuboid_handle = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .collider(&collider_cuboid)
            .user_data(format!("方块{}", i))
            .build(&mut world).handle();
        cuboid_handles.borrow_mut().push(rigid_cuboid_handle);
    }

    // world.remove_bodies(remove_list_bodys.as_slice());
    
    let cuboid_handles_clone = cuboid_handles.clone();

    loop{
        thread::sleep(Duration::from_millis(16));
        world.step();
        let mut body = None;
        for contact in world.contact_events() {
            match contact{
                ContactEvent::Started(handle1, handle2) => {
                   match (world.collider_body_handle(*handle1), world.collider_body_handle(*handle2)){
                        (Some(body_handle1), Some(body_handle2)) => {
                            println!("cuboid_handles_clone={:?}", cuboid_handles_clone.borrow());
                            println!("{:?},{:?}", body_handle1, body_handle2);
                            cuboid_handles_clone.borrow_mut().retain(|ch|{
                                if body_handle1==rigid_ball_handle && body_handle2 == *ch{
                                    debug!("球和{:?}碰撞!", world.rigid_body(body_handle2).unwrap().user_data().unwrap().downcast_ref::<String>());
                                    // remove_list_bodys.push(body_handle2);
                                    // remove_list.push(*handle2);
                                    body = Some(body_handle2);
                                    false
                                }else{
                                    true
                                }
                            });
                        }
                        _ => ()
                    };
                }
                _ => ()
            };
        }
        if let Some(body) = body{
            world.remove_bodies(&[body]);
        }
    }

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);
    // testbed.look_at(Point2::new(0.0, -2.5), 95.0);
    
    testbed.add_callback(move |world, _, _|{
        let mut world = world.get_mut();
        let mut remove_list = vec![];
        let mut remove_list_bodys = vec![];
        let mut cl = false;
        println!(">>>>>>>>>>in callback");
        for contact in world.contact_events() {
            match contact{
                ContactEvent::Started(handle1, handle2) => {
                    match (world.collider_body_handle(*handle1), world.collider_body_handle(*handle2)){
                        (Some(body_handle1), Some(body_handle2)) => {
                            println!("cuboid_handles_clone={:?}", cuboid_handles_clone.borrow());
                            cuboid_handles_clone.borrow_mut().retain(|ch|{
                                if body_handle1==rigid_ball_handle && body_handle2 == *ch{
                                    debug!("球和{:?}碰撞!", world.rigid_body(body_handle2).unwrap().user_data().unwrap().downcast_ref::<String>());
                                    remove_list_bodys.push(body_handle2);
                                    remove_list.push(*handle2);
                                    cl = true;
                                    false
                                }else{
                                    true
                                }
                            });
                        }
                        _ => ()
                    };
                }
                _ => ()
            };
        }

        // world.remove_bodies(remove_list_bodys.as_slice());
        // world.remove_colliders(remove_list.as_slice());
        if cl{
            world.remove_bodies(&[cuboid_handles.borrow()[3]]);
        }
        println!("ok.");
        
        // world.remove_colliders(remove_list.as_slice());
        // if remove_list.len()>0{
        //     println!("remove body {:?}", remove_list);
        //     world.remove_bodies(remove_list.as_slice());
        //     remove_list.clear();
        //     println!("remove body ok!!");
        // }
    });
    testbed.run();