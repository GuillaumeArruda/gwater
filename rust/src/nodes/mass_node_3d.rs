use godot::prelude::*;
use godot::classes::Node3D;
use godot::classes::INode3D;
use godot::classes::RigidBody3D;
use godot::classes::rigid_body_3d;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct MassNode3D {
    base : Base<Node3D>,
    #[export(range = (0.0,1000.0,or_greater))]
    mass : f32,
}

#[godot_api]
impl INode3D for MassNode3D {
    fn init(base: Base<Node3D>) -> Self{
        Self {
            base,
            mass : 500.0,
        }
    }

    fn enter_tree(&mut self) {
        self.update_mass(self.mass);
    }

    fn exit_tree(&mut self){
        self.update_mass(-self.mass)
    }
}

impl MassNode3D {
    fn update_mass(&mut self, mass : f32){
        let parent: Option<Gd<RigidBody3D>> = crate::utils::get_parent_of_type::<RigidBody3D>(self.to_gd().upcast::<Node>());
        if let Some(mut rigid_body) = parent {
            let center_of_mass = rigid_body.get_center_of_mass();
            if rigid_body.get_center_of_mass_mode() == rigid_body_3d::CenterOfMassMode::AUTO {
                rigid_body.set_center_of_mass_mode(rigid_body_3d::CenterOfMassMode::CUSTOM);
            }

            let current_mass = rigid_body.get_mass();
            let new_mass = current_mass + mass;
            let new_center_of_mass = (center_of_mass * current_mass + ((self.base().get_global_position() - rigid_body.get_global_position()) * mass)) / new_mass;
            rigid_body.set_mass(new_mass); 
            rigid_body.set_center_of_mass(new_center_of_mass);
        }
        else {
            godot_error!("MassNode3D require a rigidbody3d somewhere in its parent hierarchy")
        }
    }
}