use cgmath::*;
use glium::glutin;

pub struct CameraState {
    aspect_ratio: f32,
    position: Point3<f32>,
    direction: Point3<f32>,
    perspective: Matrix4<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        let aspect = 1024.0 / 768.0;
        CameraState {
            aspect_ratio: aspect,
            position: Point3::new(0.1, 0.1, 1.0),
            direction: Point3::new(0.0, 0.0, -1.0),
            perspective: perspective(Deg(60.0), aspect, 0.1, 100.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        self.perspective.into()
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        // Why does this crap ass cgmath library not have any addition operator for Points??? WTF?
        let target = Point3::new(self.position.x + self.direction.x, self.position.y + self.direction.y,
                                 self.position.z + self.direction.z);
        Matrix4::look_at(self.position, target, Vector3::new(0.0, 1.0, 0.0)).into()
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position.into()
    }

    pub fn get_direction(&self) -> [f32; 3] {
        self.direction.into()
    }

    pub fn update(&mut self) {
        self.direction.z += 0.001;
        self.direction.x += 0.001;

        if self.moving_up {
            self.position.y += 0.01;
        }

        if self.moving_left {
            self.position.x -= 0.01;
        }

        if self.moving_down {
            self.position.y -= 0.01;
        }

        if self.moving_right {
            self.position.x += 0.01;
        }

        if self.moving_forward {
            self.position.x += self.direction.x * 0.01;
            self.position.y += self.direction.y * 0.01;
            self.position.z += self.direction.z * 0.01;
        }

        if self.moving_backward {
            self.position.x -= self.direction.x * 0.01;
            self.position.y -= self.direction.y * 0.01;
            self.position.z -= self.direction.z * 0.01;
        }
    }

    pub fn process_input(&mut self, event: &glutin::WindowEvent) {
        let input = match *event {
            glutin::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }

    pub fn update_direction(&mut self, diff_x: f64, diff_y: f64) {
//        println!("Modifiying direction: {} {}", diff_x, diff_y);
//        let modified = self.direction.z + (diff_x as f32);
//        if modified < -1.0 {
//            self.direction.z = 1.0;
//        }
//        else if modified > 1.0 {
//            self.direction.z = -1.0;
//        }
//        else {
//            self.direction.z = modified;
//        }
    }
}

