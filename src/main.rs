mod framebuffer;
mod color;
mod vertex;
mod fragment;
mod shaders;
mod obj_loader;
mod camera;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::time::Duration;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj_loader::Obj;
use camera::Camera;
use shaders::{vertex_shader, fragment_shader};
use fragment::Fragment;
use color::Color;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise_offset: Vec3,
}

fn create_noise() -> Vec3 {
    Vec3::new(
        rand::random::<f32>() * 100.0,
        rand::random::<f32>() * 100.0,
        rand::random::<f32>() * 100.0,
    )
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,   1.0, 0.0,
        0.0,    0.0,   0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], shader_type: u8) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let min_x = a.x.min(b.x).min(c.x).floor() as i32;
    let min_y = a.y.min(b.y).min(c.y).floor() as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil() as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil() as i32;

    let triangle_area = edge_function(&a, &b, &c);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            let w1 = edge_function(&b, &c, &point);
            let w2 = edge_function(&c, &a, &point);
            let w3 = edge_function(&a, &b, &point);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let w1 = w1 / triangle_area;
                let w2 = w2 / triangle_area;
                let w3 = w3 / triangle_area;

                let depth = a.z * w1 + b.z * w2 + c.z * w3;
                
                let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
                let normal = normal.normalize();

                let position = v1.position * w1 + v2.position * w2 + v3.position * w3;

                fragments.push(Fragment::new(
                    Vec3::new(x as f32, y as f32, depth),
                    normal,
                    depth,
                    position,
                    0.0,
                ));
            }
        }
    }

    fragments
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Lab 5 - Shader Planets",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    // Cargar el modelo de esfera
    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut time = 0;
    let mut current_shader: u8 = 1; // 1=Sol, 2=Rocoso, 3=Gaseoso, 4=Con anillos, 5=Con luna
    let mut rotation_y = 0.0f32;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Cambiar shader con teclas numéricas
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            current_shader = 1;
            println!("Shader: Sol (Estrella)");
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            current_shader = 2;
            println!("Shader: Planeta Rocoso");
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            current_shader = 3;
            println!("Shader: Gigante Gaseoso");
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            current_shader = 4;
            println!("Shader: Planeta con Anillos");
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            current_shader = 5;
            println!("Shader: Planeta con Luna");
        }

        time += 1;
        rotation_y += 0.01;

        framebuffer.clear();

        // Renderizar el planeta actual
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let rotation = Vec3::new(0.0, rotation_y, 0.0);
        let scale = 1.0f32;

        let uniforms = Uniforms {
            model_matrix: create_model_matrix(translation, scale, rotation),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
            time,
            noise_offset: create_noise(),
        };

        render(&mut framebuffer, &uniforms, &vertex_arrays, current_shader);

        // Renderizar luna si está en modo planeta con luna
        if current_shader == 5 {
            let moon_translation = Vec3::new(2.0, 0.0, 0.0);
            let moon_rotation = Vec3::new(0.0, rotation_y * 0.5, 0.0);
            let moon_scale = 0.3f32;

            let moon_uniforms = Uniforms {
                model_matrix: create_model_matrix(moon_translation, moon_scale, moon_rotation),
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
                viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
                time,
                noise_offset: create_noise(),
            };

            render(&mut framebuffer, &moon_uniforms, &vertex_arrays, 6); // Shader 6 para la luna
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}
