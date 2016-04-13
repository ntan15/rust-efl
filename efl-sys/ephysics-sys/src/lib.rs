extern crate libc;
extern crate evas_sys;
extern crate eina_sys;

use libc::*;
use evas_sys::*;
use eina_sys::*;
/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct EPhysicsQuaternion {
    pub w: c_double,
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}
pub enum EPhysicsShape { }
pub enum EPhysicsBody { }
pub enum EPhysicsCamera { }
pub enum EPhysicsWorld { }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsCallbackWorldType {
    EPHYSICS_CALLBACK_WORLD_DEL = 0,
    EPHYSICS_CALLBACK_WORLD_STOPPED = 1,
    EPHYSICS_CALLBACK_WORLD_CAMERA_MOVED = 2,
    EPHYSICS_CALLBACK_WORLD_UPDATE = 3,
    EPHYSICS_CALLBACK_WORLD_LAST = 4,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsWorldSolverMode {
    EPHYSICS_WORLD_SOLVER_RANDMIZE_ORDER = 1,
    EPHYSICS_WORLD_SOLVER_USE_WARMSTARTING = 4,
    EPHYSICS_WORLD_SOLVER_USE_2_FRICTION_DIRECTIONS = 16,
    EPHYSICS_WORLD_SOLVER_SIMD = 256,
}
pub type EPhysicsWorldEventCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                            world: *mut EPhysicsWorld,
                                                            event_info: *mut c_void)>;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsBodyClothAnchorSide {
    EPHYSICS_BODY_CLOTH_ANCHOR_SIDE_LEFT = 0,
    EPHYSICS_BODY_CLOTH_ANCHOR_SIDE_RIGHT = 1,
    EPHYSICS_BODY_CLOTH_ANCHOR_SIDE_TOP = 2,
    EPHYSICS_BODY_CLOTH_ANCHOR_SIDE_BOTTOM = 3,
    EPHYSICS_BODY_CLOTH_ANCHOR_SIDE_LAST = 4,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsBodyFace {
    EPHYSICS_BODY_BOX_FACE_MIDDLE_FRONT = 0,
    EPHYSICS_BODY_BOX_FACE_MIDDLE_BACK = 1,
    EPHYSICS_BODY_BOX_FACE_FRONT = 2,
    EPHYSICS_BODY_BOX_FACE_BACK = 3,
    EPHYSICS_BODY_BOX_FACE_LEFT = 4,
    EPHYSICS_BODY_BOX_FACE_RIGHT = 5,
    EPHYSICS_BODY_BOX_FACE_TOP = 6,
    EPHYSICS_BODY_BOX_FACE_BOTTOM = 7,
    EPHYSICS_BODY_CLOTH_FACE_FRONT = 8,
    EPHYSICS_BODY_CLOTH_FACE_BACK = 9,
    EPHYSICS_BODY_CYLINDER_FACE_MIDDLE_FRONT = 10,
    EPHYSICS_BODY_CYLINDER_FACE_MIDDLE_BACK = 11,
    EPHYSICS_BODY_CYLINDER_FACE_FRONT = 12,
    EPHYSICS_BODY_CYLINDER_FACE_BACK = 13,
    EPHYSICS_BODY_CYLINDER_FACE_CURVED = 14,
    EPHYSICS_BODY_SPHERE_FACE_FRONT = 15,
    EPHYSICS_BODY_SPHERE_FACE_BACK = 16,
    EPHYSICS_BODY_FACE_LAST = 17,
}
pub enum EPhysicsBodyCollision { }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsCallbackBodyType {
    EPHYSICS_CALLBACK_BODY_UPDATE = 0,
    EPHYSICS_CALLBACK_BODY_COLLISION = 1,
    EPHYSICS_CALLBACK_BODY_DEL = 2,
    EPHYSICS_CALLBACK_BODY_STOPPED = 3,
    EPHYSICS_CALLBACK_BODY_LAST = 4,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EPhysicsBodyMaterial {
    EPHYSICS_BODY_MATERIAL_CUSTOM = 0,
    EPHYSICS_BODY_MATERIAL_CONCRETE = 1,
    EPHYSICS_BODY_MATERIAL_IRON = 2,
    EPHYSICS_BODY_MATERIAL_PLASTIC = 3,
    EPHYSICS_BODY_MATERIAL_POLYSTYRENE = 4,
    EPHYSICS_BODY_MATERIAL_RUBBER = 5,
    EPHYSICS_BODY_MATERIAL_WOOD = 6,
    EPHYSICS_BODY_MATERIAL_LAST = 7,
}
pub type EPhysicsBodyEventCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                           body: *mut EPhysicsBody,
                                                           event_info: *mut c_void)>;
pub enum EPhysicsConstraint { }

#[link(name = "ephysics")]
extern "C" {
    pub fn ephysics_init() -> c_int;
    pub fn ephysics_shutdown() -> c_int;
    pub fn ephysics_quaternion_new() -> *mut EPhysicsQuaternion;
    pub fn ephysics_quaternion_get(quat: *const EPhysicsQuaternion,
                                   x: *mut c_double,
                                   y: *mut c_double,
                                   z: *mut c_double,
                                   w: *mut c_double);
    pub fn ephysics_quaternion_axis_angle_get(quat: *const EPhysicsQuaternion,
                                              nx: *mut c_double,
                                              ny: *mut c_double,
                                              nz: *mut c_double,
                                              a: *mut c_double);
    pub fn ephysics_quaternion_set(quat: *mut EPhysicsQuaternion,
                                   x: c_double,
                                   y: c_double,
                                   z: c_double,
                                   w: c_double);
    pub fn ephysics_quaternion_axis_angle_set(quat: *mut EPhysicsQuaternion,
                                              nx: c_double,
                                              ny: c_double,
                                              nz: c_double,
                                              a: c_double);
    pub fn ephysics_quaternion_euler_set(quat: *mut EPhysicsQuaternion,
                                         yaw: c_double,
                                         pitch: c_double,
                                         roll: c_double);
    pub fn ephysics_quaternion_normalize(quat: *mut EPhysicsQuaternion);
    pub fn ephysics_quaternion_invert(quat: *mut EPhysicsQuaternion);
    pub fn ephysics_quaternion_scale(quat: *mut EPhysicsQuaternion,
                                     scale: c_double);
    pub fn ephysics_quaternion_inverse_scale(quat: *mut EPhysicsQuaternion,
                                             scale: c_double);
    pub fn ephysics_quaternion_sum(quat1: *const EPhysicsQuaternion,
                                   quat2: *const EPhysicsQuaternion,
                                   result: *mut EPhysicsQuaternion)
     -> *mut EPhysicsQuaternion;
    pub fn ephysics_quaternion_diff(quat1: *const EPhysicsQuaternion,
                                    quat2: *const EPhysicsQuaternion,
                                    result: *mut EPhysicsQuaternion)
     -> *mut EPhysicsQuaternion;
    pub fn ephysics_quaternion_multiply(quat1: *const EPhysicsQuaternion,
                                        quat2: *const EPhysicsQuaternion,
                                        result: *mut EPhysicsQuaternion)
     -> *mut EPhysicsQuaternion;
    pub fn ephysics_quaternion_slerp(quat1: *const EPhysicsQuaternion,
                                     quat2: *const EPhysicsQuaternion,
                                     ratio: c_double,
                                     result: *mut EPhysicsQuaternion)
     -> *mut EPhysicsQuaternion;
    pub fn ephysics_quaternion_dot(quat1: *const EPhysicsQuaternion,
                                   quat2: *const EPhysicsQuaternion)
     -> c_double;
    pub fn ephysics_quaternion_angle_get(quat1: *const EPhysicsQuaternion,
                                         quat2: *const EPhysicsQuaternion)
     -> c_double;
    pub fn ephysics_quaternion_length_get(quat: *const EPhysicsQuaternion)
     -> c_double;
    pub fn ephysics_quaternion_length2_get(quat: *const EPhysicsQuaternion)
     -> c_double;
    pub fn ephysics_shape_new() -> *mut EPhysicsShape;
    pub fn ephysics_shape_del(shape: *mut EPhysicsShape);
    pub fn ephysics_shape_point_add(shape: *mut EPhysicsShape,
                                    x: c_double,
                                    y: c_double,
                                    z: c_double) -> EinaBool;
    pub fn ephysics_shape_load(filename: *const c_char)
     -> *mut EPhysicsShape;
    pub fn ephysics_shape_save(shape: *const EPhysicsShape,
                               filename: *const c_char)
     -> EinaBool;
    pub fn ephysics_camera_position_set(camera: *mut EPhysicsCamera,
                                        x: EvasCoord, y: EvasCoord);
    pub fn ephysics_camera_position_get(camera: *const EPhysicsCamera,
                                        x: *mut EvasCoord,
                                        y: *mut EvasCoord);
    pub fn ephysics_camera_body_track(camera: *mut EPhysicsCamera,
                                      body: *mut EPhysicsBody,
                                      horizontal: EinaBool,
                                      vertical: EinaBool);
    pub fn ephysics_camera_tracked_body_get(camera: *mut EPhysicsCamera,
                                            body: *mut *mut EPhysicsBody,
                                            horizontal: *mut EinaBool,
                                            vertical: *mut EinaBool);
    pub fn ephysics_camera_perspective_set(camera: *mut EPhysicsCamera,
                                           px: EvasCoord, py: EvasCoord,
                                           z0: EvasCoord, foc: EvasCoord);
    pub fn ephysics_camera_perspective_get(camera: *const EPhysicsCamera,
                                           px: *mut EvasCoord,
                                           py: *mut EvasCoord,
                                           z0: *mut EvasCoord,
                                           foc: *mut EvasCoord);
    pub fn ephysics_camera_perspective_enabled_set(camera: *mut EPhysicsCamera,
                                                   enabled: EinaBool);
    pub fn ephysics_camera_perspective_enabled_get(camera: *const EPhysicsCamera)
     -> EinaBool;
    pub fn ephysics_world_new() -> *mut EPhysicsWorld;
    pub fn ephysics_world_render_geometry_set(world: *mut EPhysicsWorld,
                                              x: EvasCoord, y: EvasCoord,
                                              z: EvasCoord, w: EvasCoord,
                                              h: EvasCoord, d: EvasCoord);
    pub fn ephysics_world_render_geometry_get(world: *const EPhysicsWorld,
                                              x: *mut EvasCoord,
                                              y: *mut EvasCoord,
                                              z: *mut EvasCoord,
                                              w: *mut EvasCoord,
                                              h: *mut EvasCoord,
                                              d: *mut EvasCoord);
    pub fn ephysics_world_serialize(world: *mut EPhysicsWorld,
                                    path: *const c_char)
     -> EinaBool;
    pub fn ephysics_world_del(world: *mut EPhysicsWorld);
    pub fn ephysics_world_running_set(world: *mut EPhysicsWorld,
                                      running: EinaBool);
    pub fn ephysics_world_running_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_max_sleeping_time_set(world: *mut EPhysicsWorld,
                                                sleeping_time: c_double);
    pub fn ephysics_world_max_sleeping_time_get(world: *const EPhysicsWorld)
     -> c_double;
    pub fn ephysics_world_gravity_set(world: *mut EPhysicsWorld,
                                      gx: c_double,
                                      gy: c_double,
                                      gz: c_double);
    pub fn ephysics_world_constraint_solver_iterations_set(world: *mut EPhysicsWorld,
                                                           iterations: c_int);
    pub fn ephysics_world_constraint_solver_iterations_get(world: *const EPhysicsWorld)
     -> c_int;
    pub fn ephysics_world_constraint_solver_mode_enable_set(world: *mut EPhysicsWorld,
                                                            solver_mode: EPhysicsWorldSolverMode,
                                                            enable: EinaBool);
    pub fn ephysics_world_constraint_solver_mode_enable_get(world: *const EPhysicsWorld,
                                                            solver_mode: EPhysicsWorldSolverMode)
     -> EinaBool;
    pub fn ephysics_world_gravity_get(world: *const EPhysicsWorld,
                                      gx: *mut c_double,
                                      gy: *mut c_double,
                                      gz: *mut c_double);
    pub fn ephysics_world_rate_set(world: *mut EPhysicsWorld,
                                   rate: c_double);
    pub fn ephysics_world_rate_get(world: *const EPhysicsWorld)
     -> c_double;
    pub fn ephysics_world_bodies_get(world: *const EPhysicsWorld)
     -> *mut EinaList;
    pub fn ephysics_world_camera_get(world: *const EPhysicsWorld)
     -> *mut EPhysicsCamera;
    pub fn ephysics_world_event_callback_add(world: *mut EPhysicsWorld,
                                             _type: EPhysicsCallbackWorldType,
                                             func: EPhysicsWorldEventCb,
                                             data: *const c_void);
    pub fn ephysics_world_event_callback_del(world: *mut EPhysicsWorld,
                                             _type: EPhysicsCallbackWorldType,
                                             func: EPhysicsWorldEventCb)
     -> *mut c_void;
    pub fn ephysics_world_event_callback_del_full(world: *mut EPhysicsWorld,
                                                  _type: EPhysicsCallbackWorldType,
                                                  func: EPhysicsWorldEventCb,
                                                  data: *mut c_void)
     -> *mut c_void;
    pub fn ephysics_world_linear_slop_set(world: *mut EPhysicsWorld,
                                          linear_slop: c_double);
    pub fn ephysics_world_linear_slop_get(world: *const EPhysicsWorld)
     -> c_double;
    pub fn ephysics_world_bodies_outside_top_autodel_set(world: *mut EPhysicsWorld,
                                                         autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_top_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_bodies_outside_bottom_autodel_set(world: *mut EPhysicsWorld,
                                                            autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_bottom_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_bodies_outside_right_autodel_set(world: *mut EPhysicsWorld,
                                                           autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_right_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_bodies_outside_left_autodel_set(world: *mut EPhysicsWorld,
                                                          autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_left_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_bodies_outside_front_autodel_set(world: *mut EPhysicsWorld,
                                                           autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_front_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_bodies_outside_back_autodel_set(world: *mut EPhysicsWorld,
                                                          autodel: EinaBool);
    pub fn ephysics_world_bodies_outside_back_autodel_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_simulation_set(world: *mut EPhysicsWorld,
                                         fixed_time_step: c_double,
                                         max_sub_steps: c_int);
    pub fn ephysics_world_simulation_get(world: *const EPhysicsWorld,
                                         fixed_time_step: *mut c_double,
                                         max_sub_steps: *mut c_int);
    pub fn ephysics_world_point_light_position_set(world: *mut EPhysicsWorld,
                                                   lx: EvasCoord,
                                                   ly: EvasCoord,
                                                   lz: EvasCoord);
    pub fn ephysics_world_point_light_color_set(world: *mut EPhysicsWorld,
                                                lr: c_int,
                                                lg: c_int,
                                                lb: c_int);
    pub fn ephysics_world_ambient_light_color_set(world: *mut EPhysicsWorld,
                                                  ar: c_int,
                                                  ag: c_int,
                                                  ab: c_int);
    pub fn ephysics_world_point_light_position_get(world: *const EPhysicsWorld,
                                                   lx: *mut EvasCoord,
                                                   ly: *mut EvasCoord,
                                                   lz: *mut EvasCoord);
    pub fn ephysics_world_point_light_color_get(world: *const EPhysicsWorld,
                                                lr: *mut c_int,
                                                lg: *mut c_int,
                                                lb: *mut c_int);
    pub fn ephysics_world_ambient_light_color_get(world: *const EPhysicsWorld,
                                                  ar: *mut c_int,
                                                  ag: *mut c_int,
                                                  ab: *mut c_int);
    pub fn ephysics_world_light_all_bodies_set(world: *mut EPhysicsWorld,
                                               enable: EinaBool);
    pub fn ephysics_world_light_all_bodies_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_world_stack_enable_set(world: *mut EPhysicsWorld,
                                           enabled: EinaBool);
    pub fn ephysics_world_stack_enable_get(world: *const EPhysicsWorld)
     -> EinaBool;
    pub fn ephysics_body_soft_body_hardness_set(body: *mut EPhysicsBody,
                                                hardness: c_double);
    pub fn ephysics_body_soft_body_hardness_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_soft_body_anchor_hardness_set(body: *mut EPhysicsBody,
                                                       hardness: c_double);
    pub fn ephysics_body_soft_body_anchor_hardness_get(body: *mut EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_soft_body_drag_coefficient_set(body: *mut EPhysicsBody,
                                                        coefficient: c_double);
    pub fn ephysics_body_soft_body_drag_coefficient_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_soft_body_dragging_set(body: *mut EPhysicsBody,
                                                triangle:
                                                    c_int);
    pub fn ephysics_body_soft_body_dragging_unset(body: *mut EPhysicsBody);
    pub fn ephysics_body_soft_body_triangle_index_get(body: *mut EPhysicsBody,
                                                      x: EvasCoord,
                                                      y: EvasCoord)
     -> c_int;
    pub fn ephysics_body_soft_body_slice_index_get(body: *mut EPhysicsBody,
                                                   slice: *mut EvasObject)
     -> c_int;
    pub fn ephysics_body_soft_sphere_add(world: *mut EPhysicsWorld,
                                         granularity: c_int)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_soft_body_triangles_inside_get(body:
                                                            *const EPhysicsBody,
                                                        x: EvasCoord,
                                                        y: EvasCoord,
                                                        z: EvasCoord,
                                                        w: EvasCoord,
                                                        h: EvasCoord,
                                                        d: EvasCoord)
     -> *mut EinaList;
    pub fn ephysics_body_soft_body_triangle_impulse_apply(body: *mut EPhysicsBody,
                                                          idx: c_int,
                                                          x: c_double,
                                                          y: c_double,
                                                          z: c_double);
    pub fn ephysics_body_soft_body_triangle_list_impulse_apply(body: *mut EPhysicsBody,
                                                               triangles: *mut EinaList,
                                                               x: c_double,
                                                               y: c_double,
                                                               z: c_double);
    pub fn ephysics_body_soft_body_position_iterations_set(body: *mut EPhysicsBody,
                                                           iterations: c_int);
    pub fn ephysics_body_soft_body_position_iterations_get(body: *mut EPhysicsBody)
     -> c_int;
    pub fn ephysics_body_soft_body_triangle_move(body: *mut EPhysicsBody,
                                                 idx: c_int,
                                                 x: EvasCoord, y: EvasCoord,
                                                 z: EvasCoord);
    pub fn ephysics_body_soft_body_bending_constraints_add(body: *mut EPhysicsBody,
                                                           number: c_int);
    pub fn ephysics_body_sphere_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_cylinder_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_soft_cylinder_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_box_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_soft_box_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_cloth_add(world: *mut EPhysicsWorld,
                                   rows: c_ushort,
                                   columns: c_ushort)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_cloth_anchor_full_add(body1: *mut EPhysicsBody,
                                               body2: *mut EPhysicsBody,
                                               side: EPhysicsBodyClothAnchorSide);
    pub fn ephysics_body_cloth_anchor_add(body1: *mut EPhysicsBody,
                                          body2: *mut EPhysicsBody,
                                          node: c_int);
    pub fn ephysics_body_cloth_anchor_del(body: *mut EPhysicsBody);
    pub fn ephysics_body_shape_add(world: *mut EPhysicsWorld,
                                   shape: *mut EPhysicsShape)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_top_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_bottom_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_left_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_right_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_front_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_back_boundary_add(world: *mut EPhysicsWorld)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_del(body: *mut EPhysicsBody);
    pub fn ephysics_body_world_get(body: *const EPhysicsBody)
     -> *mut EPhysicsWorld;
    pub fn ephysics_body_evas_object_set(body: *mut EPhysicsBody,
                                         evas_obj: *mut EvasObject,
                                         use_obj_pos: EinaBool);
    pub fn ephysics_body_evas_object_unset(body: *mut EPhysicsBody)
     -> *mut EvasObject;
    pub fn ephysics_body_evas_object_get(body: *const EPhysicsBody)
     -> *mut EvasObject;
    pub fn ephysics_body_face_evas_object_set(body: *mut EPhysicsBody,
                                              face: EPhysicsBodyFace,
                                              evas_obj: *mut EvasObject,
                                              use_obj_pos: EinaBool);
    pub fn ephysics_body_face_evas_object_get(body: *const EPhysicsBody,
                                              face: EPhysicsBodyFace)
     -> *mut EvasObject;
    pub fn ephysics_body_face_evas_object_unset(body: *mut EPhysicsBody,
                                                face: EPhysicsBodyFace)
     -> *mut EvasObject;
    pub fn ephysics_body_resize(body: *mut EPhysicsBody, w: EvasCoord,
                                h: EvasCoord, d: EvasCoord);
    pub fn ephysics_body_move(body: *mut EPhysicsBody, x: EvasCoord,
                              y: EvasCoord, z: EvasCoord);
    pub fn ephysics_body_geometry_set(body: *mut EPhysicsBody, x: EvasCoord,
                                      y: EvasCoord, z: EvasCoord,
                                      w: EvasCoord, h: EvasCoord,
                                      d: EvasCoord);
    pub fn ephysics_body_geometry_get(body: *const EPhysicsBody,
                                      x: *mut EvasCoord, y: *mut EvasCoord,
                                      z: *mut EvasCoord, w: *mut EvasCoord,
                                      h: *mut EvasCoord, d: *mut EvasCoord);
    pub fn ephysics_body_mass_set(body: *mut EPhysicsBody,
                                  mass: c_double);
    pub fn ephysics_body_mass_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_linear_velocity_set(body: *mut EPhysicsBody,
                                             x: c_double,
                                             y: c_double,
                                             z: c_double);
    pub fn ephysics_body_linear_velocity_get(body: *const EPhysicsBody,
                                             x: *mut c_double,
                                             y: *mut c_double,
                                             z: *mut c_double);
    pub fn ephysics_body_angular_velocity_set(body: *mut EPhysicsBody,
                                              x: c_double,
                                              y: c_double,
                                              z: c_double);
    pub fn ephysics_body_angular_velocity_get(body: *const EPhysicsBody,
                                              x: *mut c_double,
                                              y: *mut c_double,
                                              z: *mut c_double);
    pub fn ephysics_body_sleeping_threshold_set(body: *mut EPhysicsBody,
                                                linear_threshold: c_double,
                                                angular_threshold: c_double);
    pub fn ephysics_body_sleeping_threshold_get(body: *const EPhysicsBody,
                                                linear_threshold: *mut c_double,
                                                angular_threshold: *mut c_double);
    pub fn ephysics_body_stop(body: *mut EPhysicsBody);
    pub fn ephysics_body_damping_set(body: *mut EPhysicsBody,
                                     linear_damping: c_double,
                                     angular_damping: c_double);
    pub fn ephysics_body_damping_get(body: *const EPhysicsBody,
                                     linear_damping: *mut c_double,
                                     angular_damping: *mut c_double);
    pub fn ephysics_body_collision_group_add(body: *mut EPhysicsBody,
                                             group: *const c_char)
     -> EinaBool;
    pub fn ephysics_body_collision_group_del(body: *mut EPhysicsBody,
                                             group: *const c_char)
     -> EinaBool;
    pub fn ephysics_body_collision_group_list_get(body: *const EPhysicsBody)
     -> *const EinaList;
    pub fn ephysics_body_evas_object_update(body: *mut EPhysicsBody);
    pub fn ephysics_body_event_callback_add(body: *mut EPhysicsBody,
                                            _type: EPhysicsCallbackBodyType,
                                            func: EPhysicsBodyEventCb,
                                            data: *const c_void);
    pub fn ephysics_body_event_callback_del(body: *mut EPhysicsBody,
                                            _type: EPhysicsCallbackBodyType,
                                            func: EPhysicsBodyEventCb)
     -> *mut c_void;
    pub fn ephysics_body_event_callback_del_full(body: *mut EPhysicsBody,
                                                 _type: EPhysicsCallbackBodyType,
                                                 func: EPhysicsBodyEventCb,
                                                 data: *mut c_void)
     -> *mut c_void;
    pub fn ephysics_body_collision_position_get(collision: *const EPhysicsBodyCollision,
                                                x: *mut EvasCoord,
                                                y: *mut EvasCoord,
                                                z: *mut EvasCoord);
    pub fn ephysics_body_collision_contact_body_get(collision: *const EPhysicsBodyCollision)
     -> *mut EPhysicsBody;
    pub fn ephysics_body_restitution_set(body: *mut EPhysicsBody,
                                         restitution: c_double);
    pub fn ephysics_body_restitution_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_friction_set(body: *mut EPhysicsBody,
                                      friction: c_double);
    pub fn ephysics_body_friction_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_central_impulse_apply(body: *mut EPhysicsBody,
                                               x: c_double,
                                               y: c_double,
                                               z: c_double);
    pub fn ephysics_body_torque_impulse_apply(body: *mut EPhysicsBody,
                                              pitch: c_double,
                                              yaw: c_double,
                                              roll: c_double);
    pub fn ephysics_body_impulse_apply(body: *mut EPhysicsBody,
                                       x: c_double,
                                       y: c_double,
                                       z: c_double,
                                       pos_x: EvasCoord, pos_y: EvasCoord,
                                       pos_z: EvasCoord);
    pub fn ephysics_body_angular_movement_enable_set(body: *mut EPhysicsBody,
                                                     enable_x: EinaBool,
                                                     enable_y: EinaBool,
                                                     enable_z: EinaBool);
    pub fn ephysics_body_angular_movement_enable_get(body: *const EPhysicsBody,
                                                     enable_x: *mut EinaBool,
                                                     enable_y: *mut EinaBool,
                                                     enable_z: *mut EinaBool);
    pub fn ephysics_body_linear_movement_enable_set(body: *mut EPhysicsBody,
                                                    enable_x: EinaBool,
                                                    enable_y: EinaBool,
                                                    enable_z: EinaBool);
    pub fn ephysics_body_linear_movement_enable_get(body: *const EPhysicsBody,
                                                    enable_x: *mut EinaBool,
                                                    enable_y: *mut EinaBool,
                                                    enable_z: *mut EinaBool);
    pub fn ephysics_body_rotation_get(body: *const EPhysicsBody,
                                      rotation: *mut EPhysicsQuaternion)
     -> *mut EPhysicsQuaternion;
    pub fn ephysics_body_rotation_set(body: *mut EPhysicsBody,
                                      quat: *mut EPhysicsQuaternion);
    pub fn ephysics_body_data_set(body: *mut EPhysicsBody,
                                  data: *mut c_void);
    pub fn ephysics_body_data_get(body: *const EPhysicsBody)
     -> *mut c_void;
    pub fn ephysics_body_central_force_apply(body: *mut EPhysicsBody,
                                             x: c_double,
                                             y: c_double,
                                             z: c_double);
    pub fn ephysics_body_torque_apply(body: *mut EPhysicsBody,
                                      torque_x: c_double,
                                      torque_y: c_double,
                                      torque_z: c_double);
    pub fn ephysics_body_force_apply(body: *mut EPhysicsBody,
                                     x: c_double,
                                     y: c_double,
                                     z: c_double,
                                     pos_x: EvasCoord, pos_y: EvasCoord,
                                     pos_z: EvasCoord);
    pub fn ephysics_body_forces_get(body: *const EPhysicsBody,
                                    x: *mut c_double,
                                    y: *mut c_double,
                                    z: *mut c_double);
    pub fn ephysics_body_torques_get(body: *const EPhysicsBody,
                                     x: *mut c_double,
                                     y: *mut c_double,
                                     z: *mut c_double);
    pub fn ephysics_body_forces_clear(body: *mut EPhysicsBody);
    pub fn ephysics_body_center_mass_get(body: *const EPhysicsBody,
                                         x: *mut c_double,
                                         y: *mut c_double,
                                         z: *mut c_double);
    pub fn ephysics_body_density_set(body: *mut EPhysicsBody,
                                     density: c_double);
    pub fn ephysics_body_density_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_volume_get(body: *const EPhysicsBody)
     -> c_double;
    pub fn ephysics_body_material_set(body: *mut EPhysicsBody,
                                      material: EPhysicsBodyMaterial);
    pub fn ephysics_body_material_get(body: *const EPhysicsBody)
     -> EPhysicsBodyMaterial;
    pub fn ephysics_body_light_set(body: *mut EPhysicsBody,
                                   enable: EinaBool);
    pub fn ephysics_body_light_get(body: *const EPhysicsBody) -> EinaBool;
    pub fn ephysics_body_back_face_culling_set(body: *mut EPhysicsBody,
                                               enable: EinaBool);
    pub fn ephysics_body_back_face_culling_get(body: *const EPhysicsBody)
     -> EinaBool;
    pub fn ephysics_body_clockwise_get(body: *const EPhysicsBody)
     -> EinaBool;
    pub fn ephysics_constraint_linked_add(body1: *mut EPhysicsBody,
                                          body2: *mut EPhysicsBody)
     -> *mut EPhysicsConstraint;
    pub fn ephysics_constraint_anchor_set(constraint: *mut EPhysicsConstraint,
                                          anchor_b1_x: EvasCoord,
                                          anchor_b1_y: EvasCoord,
                                          anchor_b1_z: EvasCoord,
                                          anchor_b2_x: EvasCoord,
                                          anchor_b2_y: EvasCoord,
                                          anchor_b2_z: EvasCoord);
    pub fn ephysics_constraint_anchor_get(constraint: *const EPhysicsConstraint,
                                          anchor_b1_x: *mut EvasCoord,
                                          anchor_b1_y: *mut EvasCoord,
                                          anchor_b1_z: *mut EvasCoord,
                                          anchor_b2_x: *mut EvasCoord,
                                          anchor_b2_y: *mut EvasCoord,
                                          anchor_b2_z: *mut EvasCoord);
    pub fn ephysics_constraint_add(body: *mut EPhysicsBody)
     -> *mut EPhysicsConstraint;
    pub fn ephysics_constraint_linear_limit_set(constraint: *mut EPhysicsConstraint,
                                                lower_x: EvasCoord,
                                                upper_x: EvasCoord,
                                                lower_y: EvasCoord,
                                                upper_y: EvasCoord,
                                                lower_z: EvasCoord,
                                                upper_z: EvasCoord);
    pub fn ephysics_constraint_linear_limit_get(constraint: *const EPhysicsConstraint,
                                                lower_x: *mut EvasCoord,
                                                upper_x: *mut EvasCoord,
                                                lower_y: *mut EvasCoord,
                                                upper_y: *mut EvasCoord,
                                                lower_z: *mut EvasCoord,
                                                upper_z: *mut EvasCoord);
    pub fn ephysics_constraint_angular_limit_set(constraint: *mut EPhysicsConstraint,
                                                 counter_clock_x: c_double,
                                                 clock_wise_x: c_double,
                                                 counter_clock_y: c_double,
                                                 clock_wise_y: c_double,
                                                 counter_clock_z: c_double,
                                                 clock_wise_z: c_double);
    pub fn ephysics_constraint_angular_limit_get(constraint: *const EPhysicsConstraint,
                                                 counter_clock_x: *mut c_double,
                                                 clock_wise_x: *mut c_double,
                                                 counter_clock_y: *mut c_double,
                                                 clock_wise_y: *mut c_double,
                                                 counter_clock_z: *mut c_double,
                                                 clock_wise_z: *mut c_double);
    pub fn ephysics_constraint_del(constraint: *mut EPhysicsConstraint);
}