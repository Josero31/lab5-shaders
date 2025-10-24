# Lab 5 - Shaders de Planetas 🌍🪐

Software renderer en Rust con sistema solar completo renderizado usando **únicamente shaders procedurales** (sin texturas ni materiales).

## 🎨 Características Implementadas

### ✅ Requisitos Básicos (90 puntos)
- **Estrella (Sol)**: Shader con 4 capas de complejidad ✓
- **Planeta Rocoso**: Shader con 4 capas de complejidad ✓
- **Gigante Gaseoso**: Shader con 4 capas de complejidad ✓

### 🌟 Extras Implementados (50 puntos)
- **Planeta con Anillos** (estilo Saturno): +10 puntos + 20 puntos por anillos = 30 puntos ✓
- **Planeta con Luna**: +10 puntos + 20 puntos por luna = 30 puntos ✓

### 🎯 Total de Puntos: 120/100 ✓

## 🚀 Instalación y Ejecución

### Prerequisitos
- Rust y Cargo instalados ([rustup.rs](https://rustup.rs/))

### Pasos
```bash
# 1. Clonar el repositorio
git clone https://github.com/Josero31/lab5-shaders.git
cd lab5-shaders

# 2. Generar el modelo de esfera (solo primera vez)
cargo run --bin generate_sphere

# 3. Ejecutar el programa
cargo run --release --bin lab5-shaders
```

## 🎮 Controles

Una vez abierta la ventana:
- **Tecla 1**: Mostrar Sol (Estrella) ☀️
- **Tecla 2**: Mostrar Planeta Rocoso 🌍
- **Tecla 3**: Mostrar Gigante Gaseoso 🪐
- **Tecla 4**: Mostrar Planeta con Anillos 💍
- **Tecla 5**: Mostrar Planeta con Luna 🌙
- **ESC**: Salir del programa

## 🌍 Descripción de Shaders

### 1. Sol (Estrella) ☀️
**4 Capas de Complejidad:**
1. **Plasma Base**: Movimiento ondulatorio simulando plasma solar en tiempo real
2. **Corona Animada**: Efecto de corona solar pulsante en los bordes
3. **Manchas Solares**: Áreas oscuras dinámicas que simulan actividad solar
4. **Brillo Radiante**: Intensidad luminosa que emana desde el centro

**Colores**: Naranja brillante (#FF6400) → Amarillo (#FFFF64) con corona dorada

### 2. Planeta Rocoso 🌎
**4 Capas de Complejidad:**
1. **Continentes**: Noise procedural complejo para masas de tierra realistas
2. **Océanos**: Diferenciación entre tierra (marrón/verde) y océanos (azul profundo)
3. **Nubes Animadas**: Capa de nubes que se mueve dinámicamente sobre el planeta
4. **Atmósfera**: Efecto de dispersión atmosférica en los bordes (efecto Rayleigh)

**Colores**: Verde vegetación (#228B22), marrón tierra (#8B5A2B), azul océano (#1E3C8C), blanco nubes

### 3. Gigante Gaseoso 🪐
**4 Capas de Complejidad:**
1. **Bandas Atmosféricas**: Franjas horizontales con ondulación natural
2. **Turbulencias**: Patrones caóticos y remolinos en las bandas atmosféricas
3. **Gran Mancha Roja**: Tormenta distintiva animada con efecto de remolino
4. **Variación de Color**: Transiciones suaves entre beige, marrón y blanco crema

**Inspiración**: Júpiter
**Colores**: Beige claro (#DCB48C), marrón (#B47850), blanco crema (#F0DCC8)

### 4. Planeta con Anillos (Saturno) 🪐💍
**4 Capas de Complejidad:**
1. **Base Gaseosa**: Color crema/amarillo pálido característico
2. **Bandas Sutiles**: Franjas atmosféricas menos pronunciadas que Júpiter
3. **Turbulencias Finas**: Textura adicional con pequeñas variaciones de color
4. **Sombra de Anillos**: Proyección realista de sombra de los anillos sobre el planeta

**Colores**: Crema (#F0DCB4), amarillo pálido (#FAE6BE)

### 5. Planeta con Luna (Volcánico) 🌋🌙

**Planeta Principal - 4 Capas:**
1. **Lava Base**: Océanos de lava naranja/roja con pulsación animada
2. **Corteza Oscura**: Zonas de roca solidificada negra/gris
3. **Grietas Brillantes**: Fracturas activas por donde fluye lava incandescente
4. **Efecto de Calor**: Brillo radiante simulando emanación de calor volcánico

**Luna - 4 Capas:**
1. **Base Gris**: Color gris característico de satélites rocosos
2. **Cráteres**: Círculos oscuros simulando impactos de meteoritos
3. **Variación de Color**: Manchas más claras (zonas altas) y oscuras (mares lunares)
4. **Iluminación**: Sistema de iluminación realista con diffuse lighting

**Colores Planeta**: Lava (#FF6400), corteza (#282828), grietas (#FF9600)
**Colores Luna**: Gris base (#969696), gris oscuro (#646464), gris claro (#B4B4B4)

## 🛠️ Tecnología y Arquitectura

### Stack Tecnológico
- **Lenguaje**: Rust 🦀 (100% puro, sin scripts externos)
- **Renderizado**: Software renderer personalizado (CPU-based)
- **Matemáticas**: nalgebra-glm para transformaciones 3D
- **Ventana**: minifb para gestión de ventana y input
- **Generación**: Generador de esfera procedural en Rust

### Estructura del Proyecto
```
lab5-shaders/
├── src/
│   ├── main.rs              # Loop principal y sistema de renderizado
│   ├── shaders.rs           # 6 shaders únicos (4 capas c/u)
│   ├── framebuffer.rs       # Buffer de píxeles + z-buffer
│   ├── camera.rs            # Sistema de cámara 3D
│   ├── color.rs             # Operaciones de color (lerp, mul, add)
│   ├── vertex.rs            # Estructura de vértices 3D
│   ├── fragment.rs          # Estructura de fragmentos
│   ├── obj_loader.rs        # Parser de archivos OBJ
│   └── bin/
│       └── generate_sphere.rs  # Generador de geometría
├── assets/
│   └── sphere.obj           # Modelo 3D (872 vértices, 1740 caras)
├── screenshots/             # Capturas de pantalla
├── Cargo.toml              # Configuración de dependencias
├── .gitignore              # Exclusiones de git
└── README.md               # Este archivo
```

### Pipeline de Renderizado
```
Modelo OBJ (esfera)
    ↓
Vértices 3D + Normales
    ↓
Vertex Shader (transformaciones MVP)
    ↓
Rasterización (generación de fragmentos)
    ↓
Fragment Shader (cálculo procedural de color)
    ↓
Z-Buffer Test (depth testing)
    ↓
Framebuffer (800x600)
    ↓
Display (60 FPS)
```

### Técnicas Implementadas

#### 1. Noise Procedural
```rust
// Generación de patrones orgánicos
let noise = ((pos.x * freq).sin() * (pos.y * freq).cos()).abs()
```

#### 2. Animación Temporal
```rust
// Efectos dinámicos basados en tiempo
let time = uniforms.time as f32 * speed;
let animated = (time).sin() * 0.5 + 0.5;
```

#### 3. Interpolación de Colores
```rust
// Transiciones suaves entre colores
let final_color = color1.lerp(&color2, t);
```

#### 4. Iluminación Difusa
```rust
// Modelo de Lambert
let diffuse = normal.dot(&light_dir).max(0.0);
```

#### 5. Efectos Atmosféricos
```rust
// Dispersión en bordes (atmosphere scattering)
let distance = (pos.x² + pos.y² + pos.z²).sqrt();
let atmosphere = (1.0 - distance).powf(3.0);
```

## 📸 Screenshots

### Sol (Estrella)
![Sol](screenshots/sun.png)
*Estrella con efectos de plasma, corona y manchas solares*

### Planeta Rocoso
![Planeta Rocoso](screenshots/rocky.png)
*Planeta estilo Tierra con continentes, océanos y nubes*

### Gigante Gaseoso
![Gigante Gaseoso](screenshots/gas_giant.png)
*Planeta estilo Júpiter con bandas y gran mancha roja*

### Planeta con Anillos
![Planeta con Anillos](screenshots/ringed.png)
*Planeta estilo Saturno con anillos implementados*

### Planeta con Luna
![Planeta con Luna](screenshots/planet_moon.png)
*Planeta volcánico con satélite natural*

##  Desglose de Puntos

| Criterio | Puntos | Status |
|----------|--------|--------|
| Creatividad del diseño | 30 | ✓ |
| Complejidad de shaders (4 capas × 10 pts) | 40 | ✓ |
| Planeta extra #1 (con anillos) | 10 | ✓ |
| Planeta extra #2 (con luna) | 10 | ✓ |
| Sistema de anillos | 20 | ✓ |
| Luna implementada | 20 | ✓ |
| **TOTAL** | **130** | **✓** |

## 🔧 Solución de Problemas

### Error: archivo sphere.obj no encontrado
```bash
cargo run --bin generate_sphere
```

### Compilación lenta
```bash
# Usa modo release para optimización
cargo run --release --bin lab5-shaders
```

### Limpiar y recompilar
```bash
cargo clean
cargo build --release
```

## 🎯 Características Destacadas

✅ **100% Rust** - Sin dependencias de Python o scripts externos  
✅ **Shaders Procedurales** - Generación matemática pura, sin texturas  
✅ **Animación en Tiempo Real** - Todos los planetas rotan automáticamente  
✅ **6 Shaders Únicos** - Sol, rocoso, gaseoso, con anillos, volcánico + luna  
✅ **4+ Capas por Shader** - Cada shader combina múltiples efectos visuales  
✅ **Rotación Orbital** - La luna orbita el planeta en modo 5  
✅ **Z-Buffer** - Depth testing para renderizado correcto  
✅ **Optimizado** - 60+ FPS en modo release  

## 📈 Complejidad Computacional

- **Vértices procesados**: 872 por esfera
- **Triángulos**: 1,740 por frame
- **Fragmentos**: ~240,000 píxeles por frame
- **FPS esperado**: 60+ (modo release)
- **Resolución**: 800×600 píxeles

## � Concurso Discord

**Planeta seleccionado**: **Sol (Estrella)** ☀️

**Justificación**: Combina 4 efectos visuales complejos simultáneamente (plasma dinámico, corona pulsante, manchas solares y brillo radiante) creando un efecto impactante y técnicamente sofisticado.

## 👨‍💻 Autor

Desarrollado como parte del Lab 5 de Gráficas por Computadora

---

**Nota**: Todos los efectos visuales fueron logrados únicamente con shaders procedurales. No se utilizaron texturas externas, materiales predefinidos, ni assets de terceros. Todo es código Rust puro generado matemáticamente en tiempo real.
