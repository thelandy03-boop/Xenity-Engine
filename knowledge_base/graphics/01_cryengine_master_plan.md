# Plan Maestro: Adaptación Gráfica de CryEngine 2/3 para Xenity Engine
> **Módulo**: Graphics & Rendering Pipeline
> **Autor / Generado por**: Antigravity AI
> **Fecha**: 2026-11-20
> **Estado**: Aprobado / En Planificación

---

## 1. Filosofía de Diseño: "La Destilación CryEngine"

*Crysis* (CryEngine 2) y *Crysis 2/3* (CryEngine 3) marcaron la cúspide visual de la séptima generación. Sin embargo, su código fuente original era un monolito de millones de líneas diseñado para hardware de PC extremo o adaptado a duras penas en consolas.

**Nuestra estrategia**: Extraer las fórmulas matemáticas, los algoritmos de post-procesado y los trucos ópticos que crearon ese estilo visual fotorrealista (junglas exuberantes, océanos paradisíacos, rayos solares volumétricos y luz atmosférica), implementándolos en **versiones Forward/Hybrid ultra-ligeras** escritas en C++ y GLSL estándar.

---

## 2. Repositorios de Referencia y Fuentes Oficiales

| Proyecto / Fuente | Enlace Oficial | Qué aprender / extraer para Xenity |
| :--- | :--- | :--- |
| **CryEngine Fuente Oficial** | [github.com/CRYTEK/CRYENGINE](https://github.com/CRYTEK/CRYENGINE) | Shaders en `Engine/Shaders/HWScripts/CryFX/`. Fórmulas de niebla y agua. |
| **OpenLara (XProger)** | [github.com/XProger/OpenLara](https://github.com/XProger/OpenLara) | Arquitectura de render multiplataforma en `src/render/` y `src/platform/` (PSP, Vita, PS3, 360, Linux, Win). |
| **Urho3D** | [github.com/urho3d/Urho3D](https://github.com/urho3d/Urho3D) | Shaders GLSL limpios en `bin/CoreData/Shaders/GLSL/` (`LitSolid.glsl`, `PostProcess.glsl`). |
| **Dhewm3 (Doom 3 BFG)** | [github.com/dhewm/dhewm3](https://github.com/dhewm/dhewm3) | Pipeline de 60 FPS en PS3/360: Difuso + Normal + Specular optimizado. |
| **BGFX Graphics Abstraction** | [github.com/bkaradzic/bgfx](https://github.com/bkaradzic/bgfx) | Abstracción de APIs de GPU y ejemplos en `examples/` (sombras CSM, PBR, bloom). |
| **Paper Original de SSAO (Crytek)** | *Screen-Space Ambient Occlusion* (Vladimir Kajalin, 2007) | El paper original publicado por Crytek en ShaderX7 explicando el algoritmo. |
| **Paper de Shading de Epic Games** | *Real Shading in Unreal Engine 4* (Brian Karis, Siggraph) | Aproximación matemática para materiales dieléctricos y metálicos. |

---

## 3. Las 5 Técnicas Icónicas Adaptadas

### A. Rayos Solares Crepusculares (Screen-Space God Rays)
En lugar de un costoso raymarching volumétrico 3D, se utiliza un **Radial Blur en 2D** sobre el Framebuffer a partir de la posición del sol en pantalla proyectada.

### B. Niebla Atmosférica por Altura (Exponential Height Fog)
Integra analíticamente la densidad lumínica en función de la altura sobre el nivel del mar y la distancia de la cámara, coloreando el horizonte según la dirección de la luz solar (*in-scattering*).

### C. Océano Tropical con Fresnel de Schlick y Oleaje
Combina dos capas de mapas de normales animadas en direcciones opuestas con la aproximación de Fresnel de Schlick:
$$R(\theta) = R_0 + (1 - R_0)(1 - \cos\theta)^5$$

### D. SSAO Inteligente (Screen-Space Ambient Occlusion)
* **Tier 2 (PC)**: Muestreo de 8 puntos en hemisferio sobre el mapa de profundidad en resolución $1/2$ con desenfoque bilateral posterior.
* **Tier 1 (PS3 / 360 / Vita)**: Baked AO en texturas o en colores de vértices generado en Blender/Assimp (ahorra 4 ms de GPU por cuadro).
* **Tier 0 (PSP)**: Desactivado.

### E. Tonemapping ACES y Color Grading (LUT 3D)
Comprime el rango dinámico de color en coma flotante (donde el sol excede $1.0$) mediante la curva cinematográfica ACES, evitando que las altas luces se saturen en blanco plano:
$$\text{Color}_{\text{final}} = \frac{x(2.51x + 0.03)}{x(2.43x + 0.59) + 0.14}$$

---

## 4. Matriz de Compatibilidad Multiplataforma

| Característica | **Tier 0: PSP** | **Tier 1: PS3 / Xbox 360 / Vita** | **Tier 2: PC (Linux / Windows)** |
| :--- | :--- | :--- | :--- |
| **Pipeline** | Fixed Function / Vertex Light | Forward (1 Dir Light + Probes) | Forward+ / Hybrid Deferred |
| **Resolución** | 480 × 272 | 720p / 960 × 544 (Vita) | 1080p / 1440p / 4K |
| **Iluminación** | Vertex Color + 1 Luz | Lightmaps UV2 + Blinn-Phong Specular | PBR Cook-Torrance + Lightmaps |
| **Sombras** | Ninguna o proyector plano | Cascaded Shadow Maps 1024×1024 (PCF) | Cascaded Shadow Maps 2048×2048 (PCF 16x) |
| **God Rays** | Desactivado | Screen-Space Radial Blur (6 muestras) | Screen-Space Radial Blur (16 muestras) |
| **Niebla** | Niebla por vértice (OpenGL 1.x) | Niebla exponencial por fragmento | Niebla exponencial + Scattering solar |
| **Agua** | Textura transparente simple | 2 Normal maps + Fresnel + Shore foam | Normal maps + Reflejos Screen-Space |
| **AO** | Desactivado | Baked AO en Textura / Vértices | Real-Time SSAO 1/2 Res + Blur |
| **Post-Proceso** | Ninguno | ACES Tonemapping + Bloom + LUT | ACES Tonemapping + Bloom + LUT + FXAA |

---

## 5. Hoja de Ruta de Implementación en Xenity

### Fase 1: Base de Post-Procesado HDR
- [ ] Modificar `Camera::ChangeFrameBufferSize` para inicializar el texture buffer en `GL_RGBA16F`.
- [ ] Implementar `bloom_tonemap.frag` con extracción de luces altas y función `ACESFilm()`.
- [ ] Implementar el shader de `Radial Blur` para God Rays del sol.

### Fase 2: Shaders de Superficie y Mallas
- [ ] Activar `aiProcess_CalcTangentSpace` en `assimp_mesh_loader.cpp` para generar tangentes y bitangentes.
- [ ] Actualizar `standard_shader.frag` con soporte para Normal Mapping en espacio tangente.
- [ ] Añadir `ApplyHeightFog()` en el fragment shader principal.

### Fase 3: Segundo Canal UV y Lightmaps
- [ ] Permitir en `mesh_data.h` la carga de `uv2` (segundo canal de coordenadas de textura).
- [ ] Añadir slot `lightmapTexture` en la clase `Material`.
- [ ] Multiplicar $(\text{Albedo} \times \text{Lightmap})$ en el cálculo difuso de áreas estáticas.

### Fase 4: Abstracción de Consolas inspirada en OpenLara
- [ ] Separar las llamadas dependientes de OpenGL de escritorio en una capa de backend para permitir vincular fácilmente *vitaGL* (PS Vita) o *GCM* (PS3).
