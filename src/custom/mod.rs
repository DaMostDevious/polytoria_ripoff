use three_d::*;


pub struct custom_controller{
    pub current_position: Vec3,
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub last_pos: Vec2,
}

impl Default for custom_controller{
    fn default() -> Self {
        Self { current_position: vec3(0.0,0.0,0.0), target: vec3(0.0,0.0,0.0), distance: 5.0, last_pos: vec2(0.0,0.0), yaw: 0.0, pitch: 0.0 }
    }
}

impl custom_controller{
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut Vec<Event>){
        for event in events.iter(){
            match event{
                Event::MouseMotion { button, delta, position, modifiers, handled } => {
                    match button{
                        Some(mousebutton) => {
                            match mousebutton{
                                MouseButton::Right => {
                                    self.pitch += delta.1 * 0.01;
                                    self.yaw += delta.0 * 0.01;

                                    let limit = std::f32::consts::FRAC_PI_2 - 0.01;
                                    self.pitch = self.pitch.clamp(-limit, limit);

                                    let direction = vec3(
                                        self.yaw.cos() * self.pitch.cos(),
                                        self.pitch.sin(),
                                        self.yaw.sin() * self.pitch.cos()
                                    );

                                    let position = self.target + direction * self.distance;

                                    camera.set_view(position, self.target, vec3(0.0,1.0,0.0));
                                }
                                _ => {}
                            }
                        },
                        None => {}
                    }
                }
                Event::MouseWheel { delta, position, modifiers, handled } => {
                    self.distance -= delta.1 / 25.0;
                    self.distance = self.distance.clamp(0.1,100.0);
                    self.recalculatepos(camera);
                }
                _ => {}
            }
        }
    }

    pub fn refocus(&mut self, camera: &Camera, new_target: Vec3) {
        self.target = new_target;

        let dir = (camera.position() - self.target).normalize();

        self.pitch = dir.y.asin();
        self.yaw = dir.z.atan2(dir.x);
    }

    pub fn recalculatepos(&mut self, camera: &mut Camera){
        let direction = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        );

        let position = self.target + direction * self.distance;

        camera.set_view(position, self.target, vec3(0.0,1.0,0.0))
    }
}