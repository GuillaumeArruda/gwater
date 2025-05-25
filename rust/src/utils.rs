use godot::prelude::*;

pub fn get_parent_of_type<T:GodotClass + Inherits<Node>>(node: Gd<Node>) -> Option<Gd<T>> {
    let mut parent = node.get_parent()?;
    loop{
        let as_t= parent.try_cast::<T>();
        match as_t {
            Ok(x)=> return Some(x),
            Err(x)=> parent= x.get_parent()?,
        }
    }
} 
