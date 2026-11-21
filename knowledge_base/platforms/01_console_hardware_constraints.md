# Restricciones de Hardware en Consolas: Guía de Optimización para Xenity
> **Módulo**: Platforms / Architecture
> **Autor / Generado por**: Antigravity AI
> **Fecha**: 2026-11-20
> **Estado**: Aprobado / Referencia Permanente

---

## 1. Tabla de Restricciones Críticas de Hardware

| Plataforma | CPU | RAM Total | VRAM (Video) | API Gráfica | Shaders Programables | Límite Textura |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **PSP** | MIPS R4000 @ 333 MHz | 32 / 64 MB | Compartida (2 MB eDRAM) | Sony GU (Hardware nativo) | ❌ No (Función Fija) | 256 × 256 |
| **PS Vita** | 4-Core ARM Cortex-A9 | 512 MB | 128 MB dedicada | GXM / vitaGL (GLES 2.0) | ✅ Sí (GLES 2.0 / CG) | 1024 × 1024 |
| **PS3** | Cell (1 PPE + 6-7 SPEs) | 256 MB XDR | 256 MB GDDR3 | GCM / PSL1GHT (Tiny3D/GL) | ✅ Sí (Shader Model 3.0) | 2048 × 2048 |
| **Xbox 360** | Xenon (3-Core PowerPC) | 512 MB GDDR3 | 512 MB Unificada (+10 MB eDRAM) | Direct3D 9.0c+ / XNA | ✅ Sí (Shader Model 3.0+) | 2048 × 2048 |
| **PC (Linux / Win)** | x86_64 multinúcleo | 4 GB - 64 GB | 2 GB - 24 GB | OpenGL 3.3 / Vulkan | ✅ Sí (GLSL 330+ / SPIR-V) | 8192 × 8192 |

---

## 2. Puntos Críticos y Trampas Comunes en Consolas

### ⚠️ PlayStation 3 (El cuello de botella de la memoria dividida)
* **Memoria Dividida**: La PS3 tiene **256 MB de RAM de sistema** y **256 MB de VRAM**. Si un buffer excede los 256 MB de video, el juego crasheará por *Out Of Memory*.
* **Tasa de Relleno (*Fillrate*)**: El chip RSX tiene menos ancho de banda que la Xbox 360. Evitar shaders con decenas de instrucciones matemáticas por píxel o transparencias superpuestas (*overdraw*).
* **Solución**: Forward rendering con 1 sola luz dinámica con sombra; todas las demás luces deben hornearse en **Lightmaps**.

### ⚠️ PlayStation Portable (PSP - Sin shaders programables)
* **Función fija (*Fixed-Function Pipeline*)**: La PSP **no ejecuta fragment ni vertex shaders**.
* La iluminación debe calcularse por vértice en el procesador gráfico de la PSP (`sceGuAmbientColor`, `sceGuLight`).
* El motor debe tener una ruta de renderizado sin shaders cuando compile para PSP (como hace **OpenLara** en su backend de PSP).

### ⚠️ PS Vita (PowerVR SGX543MP4+)
* **Arquitectura TBDR (Tile-Based Deferred Renderer)**: El GPU de Vita divide la pantalla en cuadrículas (*tiles*). Es increíblemente eficiente con geometría opaca, pero sufre con cambios de estado y texturas gigantes.
* **Formato de textura**: Usar formatos comprimidos compatibles con hardware (GXT / PVRTC).

---

## 3. Estrategia de Formatos de Textura y Assets

1. **Modelos 3D**:
   - Mantener las mallas de personajes entre **3,000 y 8,000 triángulos** para consolas.
   - En PC se pueden usar niveles de detalle (LOD 0) de hasta 40,000 triángulos.
2. **Texturas**:
   - En PC: Texturas RGBA8 / BC7 de hasta 2048×2048.
   - En PS3 / 360: DXT1 (sin canal alfa) y DXT5 (con canal alfa), máximo 1024×1024.
   - En Vita: Texturas cuadradas en potencias de 2 ($2^n$).
   - En PSP: Texturas indexadas de 8 bits (256 colores) o 16 bits (RGB565).
