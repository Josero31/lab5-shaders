use nalgebra_glm::{Vec3, Vec4, Mat4};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    let model_mat3 = Mat4::identity();
    let normal_matrix = model_mat3;

    let normal_vector = Vec4::new(
        vertex.normal.x,
        vertex.normal.y,
        vertex.normal.z,
        0.0
    );

    let transformed_normal = normal_matrix * normal_vector;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: Vec3::new(transformed_normal.x, transformed_normal.y, transformed_normal.z).normalize(),
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: u8) -> Color {
    match shader_type {
        1 => sun_shader(fragment, uniforms),
        2 => rocky_planet_shader(fragment, uniforms),
        3 => gas_giant_shader(fragment, uniforms),
        4 => ringed_planet_shader(fragment, uniforms),
        5 => planet_with_moon_shader(fragment, uniforms),
        6 => moon_shader(fragment, uniforms),
        _ => Color::new(255, 255, 255),
    }
}

// ===== SHADER 1: SOL (ESTRELLA) =====
// Capas: 1) Plasma base, 2) Corona animada, 3) Manchas solares, 4) Brillo radiante
fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;
    
    // Capa 1: Plasma base con movimiento
    let plasma_freq = 3.0;
    let plasma1 = ((pos.x * plasma_freq + time).sin() * (pos.y * plasma_freq + time * 1.3).cos()).abs();
    let plasma2 = ((pos.y * plasma_freq - time * 0.8).sin() * (pos.z * plasma_freq + time).cos()).abs();
    let plasma = (plasma1 + plasma2) * 0.5;
    
    // Capa 2: Corona animada (más brillante en los bordes)
    let distance_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    let corona_effect = (1.0 - distance_from_center).max(0.0);
    let corona_pulse = (time * 2.0).sin() * 0.5 + 0.5;
    let corona = corona_effect * corona_pulse * 0.3;
    
    // Capa 3: Manchas solares oscuras
    let spot_freq = 8.0;
    let spot = ((pos.x * spot_freq).sin() * (pos.y * spot_freq).cos() * (pos.z * spot_freq + time).sin()).abs();
    let dark_spots = if spot > 0.85 { 0.6 } else { 1.0 };
    
    // Capa 4: Brillo radiante desde el centro
    let radial_glow = 1.0 - distance_from_center.min(1.0);
    let glow_intensity = radial_glow * radial_glow * 0.5;
    
    // Combinar todas las capas
    let base_color = Color::new(255, 100, 0); // Naranja brillante
    let bright_color = Color::new(255, 255, 100); // Amarillo brillante
    let corona_color = Color::new(255, 200, 50); // Amarillo corona
    
    let mixed = base_color.lerp(&bright_color, plasma);
    let with_corona = mixed.lerp(&corona_color, corona);
    let with_spots = with_corona.mul(dark_spots);
    let final_color = with_spots.mul(1.0 + glow_intensity);
    
    final_color
}

// ===== SHADER 2: PLANETA ROCOSO (estilo Tierra/Marte) =====
// Capas: 1) Continentes, 2) Océanos, 3) Nubes, 4) Atmósfera
fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.005;
    
    // Capa 1: Continentes (noise complejo)
    let land_freq = 5.0;
    let land_noise1 = ((pos.x * land_freq).sin() * (pos.y * land_freq).cos()).abs();
    let land_noise2 = ((pos.y * land_freq * 1.5).sin() * (pos.z * land_freq * 1.5).cos()).abs();
    let land = (land_noise1 + land_noise2) * 0.5;
    
    // Capa 2: Océanos vs tierra
    let is_land = land > 0.5;
    let base_color = if is_land {
        // Variación de continentes (marrón/verde)
        let variation = ((pos.x * 10.0).sin() * (pos.z * 10.0).cos()).abs();
        if variation > 0.6 {
            Color::new(34, 139, 34) // Verde (vegetación)
        } else {
            Color::new(139, 90, 43) // Marrón (tierra)
        }
    } else {
        Color::new(30, 60, 140) // Azul océano
    };
    
    // Capa 3: Nubes animadas
    let cloud_freq = 8.0;
    let cloud_offset = time * 0.5;
    let clouds = ((pos.x * cloud_freq + cloud_offset).sin() * 
                  (pos.y * cloud_freq).cos() * 
                  (pos.z * cloud_freq - cloud_offset * 0.7).sin()).abs();
    let cloud_threshold = 0.7;
    
    // Capa 4: Atmósfera (brillo en los bordes)
    let distance_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    let atmosphere = (1.0 - distance_from_center).max(0.0).powf(3.0) * 0.2;
    let atmo_color = Color::new(100, 150, 255);
    
    let mut final_color = base_color;
    
    // Aplicar nubes
    if clouds > cloud_threshold {
        let cloud_color = Color::new(255, 255, 255);
        final_color = final_color.lerp(&cloud_color, (clouds - cloud_threshold) * 2.0);
    }
    
    // Aplicar atmósfera
    final_color = final_color.lerp(&atmo_color, atmosphere);
    
    // Iluminación básica
    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_dir).max(0.2);
    
    final_color.mul(diffuse)
}

// ===== SHADER 3: GIGANTE GASEOSO (estilo Júpiter) =====
// Capas: 1) Bandas atmosféricas, 2) Turbulencias, 3) Gran mancha roja, 4) Variación de color
fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.008;
    
    // Capa 1: Bandas atmosféricas horizontales
    let band_freq = 15.0;
    let band_y = pos.y + (pos.x * 2.0).sin() * 0.1; // Ondulación
    let bands = ((band_y * band_freq).sin() + 1.0) * 0.5;
    
    // Capa 2: Turbulencias en las bandas
    let turb_freq = 20.0;
    let turbulence = ((pos.x * turb_freq + time).sin() * 
                      (pos.y * turb_freq * 0.5).cos() * 
                      (pos.z * turb_freq - time * 0.8).sin()).abs();
    let turb_offset = turbulence * 0.2;
    
    // Capa 3: Gran mancha roja (tormenta)
    let spot_center = Vec3::new(0.3, -0.2, 0.0);
    let dist_to_spot = ((pos.x - spot_center.x).powi(2) + 
                        (pos.y - spot_center.y).powi(2) + 
                        (pos.z - spot_center.z).powi(2)).sqrt();
    let red_spot = (1.0 - (dist_to_spot / 0.3).min(1.0)).max(0.0);
    let spot_swirl = ((pos.x * 30.0 + time * 2.0).sin() * (pos.y * 30.0).cos()).abs();
    let spot_intensity = red_spot * spot_swirl;
    
    // Capa 4: Colores de las bandas (beige/marrón/blanco)
    let band_value = bands + turb_offset;
    let color1 = Color::new(220, 180, 140); // Beige claro
    let color2 = Color::new(180, 120, 80);  // Marrón
    let color3 = Color::new(240, 220, 200); // Casi blanco
    
    let mut final_color = if band_value < 0.33 {
        color1.lerp(&color2, band_value * 3.0)
    } else if band_value < 0.66 {
        color2.lerp(&color3, (band_value - 0.33) * 3.0)
    } else {
        color3.lerp(&color1, (band_value - 0.66) * 3.0)
    };
    
    // Aplicar la mancha roja
    if spot_intensity > 0.3 {
        let red_color = Color::new(200, 80, 60);
        final_color = final_color.lerp(&red_color, spot_intensity);
    }
    
    // Iluminación básica
    let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_dir).max(0.3);
    
    final_color.mul(diffuse)
}

// ===== SHADER 4: PLANETA CON ANILLOS (estilo Saturno) =====
// Capas: 1) Base gaseosa, 2) Bandas sutiles, 3) Efecto de anillo (en el planeta), 4) Sombra de anillos
fn ringed_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.005;
    
    // Capa 2: Bandas atmosféricas sutiles
    let band_freq = 12.0;
    let bands = ((pos.y * band_freq + (pos.x * 3.0).sin() * 0.15).sin() + 1.0) * 0.5;
    let band_color1 = Color::new(230, 210, 170);
    let band_color2 = Color::new(250, 230, 190);
    let banded = band_color1.lerp(&band_color2, bands);
    
    // Capa 3: Textura adicional (pequeñas turbulencias)
    let turb = ((pos.x * 25.0 + time).sin() * (pos.z * 25.0).cos()).abs();
    let turb_factor = 0.9 + turb * 0.1;
    
    // Capa 4: Sombra de los anillos proyectada en el planeta
    let ring_shadow_y = pos.y;
    let ring_shadow = if ring_shadow_y.abs() < 0.15 {
        let shadow_pattern = ((pos.x * 50.0).sin() * (pos.z * 50.0).cos()).abs();
        0.5 + shadow_pattern * 0.2
    } else {
        1.0
    };
    
    let final_color = banded.mul(turb_factor * ring_shadow);
    
    // Iluminación
    let light_dir = Vec3::new(1.0, 0.8, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_dir).max(0.25);
    
    final_color.mul(diffuse)
}

// ===== SHADER 5: PLANETA CON LUNA (rocoso con océanos de lava) =====
// Capas: 1) Lava base, 2) Corteza oscura, 3) Grietas brillantes, 4) Efecto de calor
fn planet_with_moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;
    
    // Capa 1: Lava base (naranja/rojo)
    let lava_pulse = (time * 3.0).sin() * 0.5 + 0.5;
    let lava_color = Color::new(255, 100, 0);
    
    // Capa 2: Corteza oscura (negro/gris)
    let crust_freq = 8.0;
    let crust_noise = ((pos.x * crust_freq).sin() * 
                       (pos.y * crust_freq).cos() * 
                       (pos.z * crust_freq).sin()).abs();
    let is_crust = crust_noise > 0.4;
    
    // Capa 3: Grietas brillantes de lava
    let crack_freq = 20.0;
    let cracks = ((pos.x * crack_freq + time).sin() * 
                  (pos.y * crack_freq - time * 0.7).cos()).abs();
    let has_crack = cracks > 0.85;
    
    // Capa 4: Efecto de calor radiante
    let heat_glow = ((pos.x * 5.0 + time * 2.0).sin() * 
                     (pos.z * 5.0 - time).cos()).abs() * 0.3;
    
    let crust_color = Color::new(40, 40, 40);
    let crack_color = Color::new(255, 150, 0);
    
    let mut final_color = if is_crust {
        crust_color
    } else {
        lava_color.mul(0.7 + lava_pulse * 0.3)
    };
    
    // Aplicar grietas brillantes
    if has_crack {
        final_color = crack_color;
    }
    
    // Añadir brillo de calor
    let glow_add = Color::new(
        (heat_glow * 100.0) as u8,
        (heat_glow * 50.0) as u8,
        0
    );
    final_color = final_color.add(&glow_add);
    
    // Iluminación
    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_dir).max(0.3);
    
    final_color.mul(diffuse)
}

// ===== SHADER 6: LUNA (satélite gris con cráteres) =====
// Capas: 1) Base gris, 2) Cráteres oscuros, 3) Variación de color, 4) Iluminación
fn moon_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    
    // Capa 1: Base gris
    let base_color = Color::new(150, 150, 150);
    
    // Capa 2: Cráteres (círculos oscuros)
    let crater_freq = 15.0;
    let crater1 = ((pos.x * crater_freq).sin() * (pos.y * crater_freq).cos()).abs();
    let crater2 = ((pos.y * crater_freq * 1.3).sin() * (pos.z * crater_freq * 1.3).cos()).abs();
    let has_crater = crater1 > 0.8 || crater2 > 0.8;
    
    // Capa 3: Variación de color (manchas más claras/oscuras)
    let variation_freq = 8.0;
    let variation = ((pos.x * variation_freq).sin() * 
                     (pos.z * variation_freq).cos()).abs();
    
    let dark_gray = Color::new(100, 100, 100);
    let light_gray = Color::new(180, 180, 180);
    
    let final_color = if has_crater {
        dark_gray
    } else {
        base_color.lerp(&light_gray, variation)
    };
    
    // Capa 4: Iluminación fuerte
    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_dir).max(0.15);
    
    final_color.mul(diffuse)
}
