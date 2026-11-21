# Procedencia de Código, Algoritmos y Fuentes Oficiales (Source Provenance)
> **Propósito**: Documentar con precisión matemática y enlaces canónicos cada técnica gráfica adaptada en Xenity Engine para garantizar total trazabilidad histórica, académica y técnica (cero suposiciones o código inventado).

---

## 1. Repositorios Oficiales de Código Fuente de Referencia

| Motor / Proyecto | Repositorio Oficial | Módulos y Archivos de Interés para Xenity |
| :--- | :--- | :--- |
| **Crytek CryEngine** | [github.com/CRYTEK/CRYENGINE](https://github.com/CRYTEK/CRYENGINE) | • `Engine/Shaders/HWScripts/CryFX/Common.cfx`<br>• `Engine/Shaders/HWScripts/CryFX/Fog.cfi` (Fórmulas originales de niebla y dispersión solar)<br>• `Engine/Shaders/HWScripts/CryFX/Water.cfx` (Mapeo dual de normales y oleaje)<br>• `Engine/Shaders/HWScripts/CryFX/PostEffects.cfi` (Radial Blur God Rays) |
| **OpenLara (XProger)** | [github.com/XProger/OpenLara](https://github.com/XProger/OpenLara) | • `src/platform/psp/` (Uso óptimo de GU y formatos de vértices compactados)<br>• `src/platform/vita/` (vitaGL y shaders GLES 2.0)<br>• `src/platform/ps3/` (PSL1GHT y RSX GCM directo)<br>• `src/render/` (Pipeline agnóstico a la API gráfica) |
| **Urho3D** | [github.com/urho3d/Urho3D](https://github.com/urho3d/Urho3D) | • `bin/CoreData/Shaders/GLSL/LitSolid.glsl` (Pipeline forward estándar con cálculo de luces por iteración)<br>• `bin/CoreData/Shaders/GLSL/PostProcess.glsl` (Tonemapping, Bloom y corrección gamma) |
| **Dhewm3 (Doom 3 BFG)** | [github.com/dhewm/dhewm3](https://github.com/dhewm/dhewm3) | • `neo/renderer/` (Implementación clásica 60 FPS Diffuse + Specular + Normal map en OpenGL 1.4 / 2.0 para consolas de 7ma gen) |
| **BGFX Abstraction** | [github.com/bkaradzic/bgfx](https://github.com/bkaradzic/bgfx) | • `examples/` (06-meshes con TBN, 13-shadows con CSM, 21-deferred con SSAO) |

---

## 2. Fuentes Académicas y Matemáticas de Cada Técnica

### A. Niebla Atmosférica por Altura Exponencial (Exponential Height Fog)
* **Origen y Publicación Original**:
  - *Finding Next Gen: CryEngine 2* (Martin Mittring, Crytek, Siggraph 2007).
  - *Analytical Atmosphere Fog* (Inigo Quilez, 2009). Derivación matemática de la integral de densidad de Rayleigh/Mie a lo largo del rayo de vista.
* **Fórmula Matemática Canónica**:
  La integral de la densidad $\rho(h) = \rho_0 e^{-\lambda y}$ a lo largo de un rayo $\mathbf{p}(t) = \mathbf{c} + t \mathbf{v}$ viene dada analíticamente por:
  $$\text{Extinction}(d) = \frac{\rho_0}{\lambda \cdot v_y} e^{-\lambda c_y} \left(1 - e^{-\lambda d \cdot v_y}\right)$$
  Donde:
  - $\rho_0$ (`fogDensity`): Densidad al nivel del mar.
  - $\lambda$ (`fogHeightFalloff`): Factor exponencial de reducción por altitud.
  - $c_y$: Altura de la cámara (`viewPos.y`).
  - $v_y$: Componente vertical del vector normalizado de la visual (`viewDir.y`).
  - $d$: Distancia al fragmento (`dist`).
* **Implementación en Xenity**: [`Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader`](file:///home/landy/Proyectos/Xenity-Engine/Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader#L194-L210).

---

### B. Mapeo de Normales sin Tangentes por Vértice (Screen-Space Cotangent Frame)
* **Origen y Publicación Original**:
  - *Normal Mapping without Precomputed Tangents* (Christian Schüler, publicado en el libro *ShaderX5: Advanced Rendering Techniques*, 2006).
  - Utilizado ampliamente por la industria cuando el ancho de banda del bus de memoria hacia la GPU (como los 22.4 GB/s de la RSX en PS3) no puede desperdiciarse transportando vectores `vec4 Tangent` por cada vértice.
* **Fundamento Matemático**:
  Utiliza las derivadas parciales de pantalla (`dFdx` y `dFdy` en GLSL) para construir la base ortonormal $\mathbf{T}, \mathbf{B}, \mathbf{N}$ resolviendo el sistema lineal de coordenadas de textura respecto a la posición 3D:
  $$\mathbf{T} = (\mathbf{dp}_2^\perp \cdot duv_{1.x}) + (\mathbf{dp}_1^\perp \cdot duv_{2.x})$$
  $$\mathbf{B} = (\mathbf{dp}_2^\perp \cdot duv_{1.y}) + (\mathbf{dp}_1^\perp \cdot duv_{2.y})$$
  Donde $\mathbf{dp}_i^\perp = \mathbf{dp}_i \times \mathbf{N}$.
* **Implementación en Xenity**: [`Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader`](file:///home/landy/Proyectos/Xenity-Engine/Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader#L164-L188).

---

### C. Tonemapping Fílmico ACES (Academy Color Encoding System)
* **Origen y Publicación Original**:
  - Ajuste analítico de curva racional de Stephen Hill y Krzysztof Narkowicz (2015), derivado del estándar ACES RRT/ODT utilizado en CryEngine 3, Unreal Engine 4 y el pipeline de Naughty Dog (*The Last of Us*).
* **Fórmula Matemática Canónica**:
  $$f(x) = \text{saturate}\left( \frac{x (2.51 x + 0.03)}{x (2.43 x + 0.59) + 0.14} \right)$$
  Garantiza hombros suaves para altas luces especulares (sin quemado blanco plano) y preservación del contraste en sombras profundas.
* **Implementación en Xenity**: [`Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader`](file:///home/landy/Proyectos/Xenity-Engine/Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader#L152-L162).

---

### D. Modelo Especular Blinn-Phong (Aesthetic de 7ma Generación)
* **Origen y Publicación Original**:
  - Jim Blinn, *Models of light reflection for computer synthesized pictures* (SIGGRAPH 1977).
  - Adoptado como el estándar universal en motores como CryEngine 2, id Tech 4 (Doom 3 BFG) y Unreal Engine 3 debido a que sustituye el costoso vector de reflexión $\mathbf{R} = 2(\mathbf{N}\cdot\mathbf{L})\mathbf{N} - \mathbf{L}$ por el vector medio $\mathbf{H} = \frac{\mathbf{L} + \mathbf{V}}{\|\mathbf{L} + \mathbf{V}\|}$.
* **Implementación en Xenity**: [`Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader`](file:///home/landy/Proyectos/Xenity-Engine/Xenity_Engine_Executable/public_engine_assets/shaders/standard.shader#L96-L149).

---

### E. Rayos Solares Crepusculares 2D (Screen-Space God Rays)
* **Origen y Publicación Original**:
  - Kenny Mitchell, *Volumetric Light Scattering as a Post-Process* (publicado en *GPU Gems 3*, Capítulo 13, Addison-Wesley 2007).
  - Adoptado en *Crysis* (2007) para simular luz solar filtrada a través de árboles en lugar de recurrir al carísimo Raymarching 3D volumétrico en consolas de 256 MB.
* **Documentación y Shader GLSL Listo**: [`knowledge_base/graphics/02_glsl_reference_shaders.md`](file:///home/landy/Proyectos/Xenity-Engine/knowledge_base/graphics/02_glsl_reference_shaders.md#L11-L60).

---

### F. Oclusión Ambiental en Espacio de Pantalla (SSAO)
* **Origen y Publicación Original**:
  - Vladimir Kajalin (Investigador principal de Crytek), *Screen-Space Ambient Occlusion* (publicado en *ShaderX7: Advanced Rendering Techniques*, Charles River Media 2007).
  - Este algoritmo nació específicamente dentro de Crytek para *Crysis* en 2007. Muestrea una esfera o hemisferio en el Z-buffer y compara la profundidad para oscurecer esquinas y hendiduras.
* **Documentación y Guía de Implementación**: [`knowledge_base/graphics/01_cryengine_master_plan.md`](file:///home/landy/Proyectos/Xenity-Engine/knowledge_base/graphics/01_cryengine_master_plan.md#L146-L151).
