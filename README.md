# Lab 5 - Sistema Solar Procedural 🌍🪐

Sistema solar renderizado en GPU con **shaders procedurales 100%** (sin texturas).

## 👨‍💻 Autor

**José Sánchez**

## � Descripción del Programa

Este programa renderiza un sistema solar completo con 6 cuerpos celestes usando shaders procedurales escritos en WGSL (WebGPU Shading Language). Cada planeta tiene su propio shader con efectos visuales únicos generados matemáticamente en tiempo real.

### Cuerpos Celestes Implementados

1. ☀️ **Sol** - Estrella con efectos de plasma, corona animada y manchas solares
2. 🌍 **Planeta Rocoso** - Continentes, océanos, nubes y atmósfera
3. 🌋 **Planeta Volcánico** - Lava animada, corteza oscura y grietas brillantes
4. 🪐 **Gigante Gaseoso** - Bandas atmosféricas, turbulencias y tormenta característica
5. � **Planeta con Anillos** - Anillos procedurales y atmósfera gaseosa
6. 🌑 **Luna/Planeta Helado** - Superficie rocosa con cráteres

### Técnicas Utilizadas

- **Renderizado GPU**: Uso de wgpu (WebGPU API para Rust)
- **Shaders Procedurales**: 6 fragment shaders únicos con efectos complejos
- **Noise Procedural**: Generación de patrones orgánicos con funciones matemáticas
- **Animación Temporal**: Efectos dinámicos basados en tiempo
- **Iluminación**: Sistema de luz direccional con cálculos diffuse
- **Geometría**: Esfera de alta resolución con normales para iluminación realista

## 🚀 Instalación y Ejecución

### Prerequisitos
- Rust y Cargo instalados ([rustup.rs](https://rustup.rs/))
- GPU compatible con WebGPU/Vulkan/DirectX 12

### Ejecutar
```bash
git clone https://github.com/Josero31/lab5-shaders.git
cd lab5-shaders
cargo run --release
```

## 🛠️ Tecnología

- **Lenguaje**: Rust 🦀
- **Renderizado**: wgpu 0.19 (WebGPU API)
- **Ventana**: winit 0.29
- **Shaders**: WGSL (WebGPU Shading Language)

## 📁 Estructura

```
lab5-shaders/
├── src/
│   ├── main.rs           # Loop principal y configuración wgpu
│   ├── shader.wgsl       # 6 shaders procedurales únicos
│   └── shaders.rs        # (archivo auxiliar)
├── Cargo.toml
└── README.md
```

---

**Lab 5 - Gráficas por Computadora**
