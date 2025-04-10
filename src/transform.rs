pub struct Transform
{
    pub translation: glam::Vec3,
    pub scaling: glam::Vec3,
    pub rotation: glam::Vec3,
}

impl Transform
{
    pub fn get_matrix(&self) -> glam::Mat4
    {
        glam::Mat4::from_scale_rotation_translation(
            self.scaling,
            glam::Quat::from_euler(
                glam::EulerRot::XYZ, 
                self.rotation.x.to_radians(),
                self.rotation.y.to_radians(),
                self.rotation.z.to_radians()
            ),
            self.translation
        )
    } 
}