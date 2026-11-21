# Xenity Engine - Knowledge Base & AI Research Vault
> **Repositorio de Documentación Técnica, Investigación y Planes de Arquitectura**
> Este directorio está estructurado para que tanto desarrolladores humanos como asistentes de Inteligencia Artificial (Antigravity, Claude, ChatGPT, Gemini, etc.) puedan consultar, modificar, expandir y añadir nuevos documentos `.md` de forma estandarizada y modular.

---

## 📂 Estructura de Carpetas

```text
knowledge_base/
├── README.md                                  # Índice general y guía de contribución (este archivo)
├── graphics/                                  # Arquitectura gráfica, shaders, pipelines y post-procesado
│   ├── 01_cryengine_master_plan.md           # Plan maestro de destilación CryEngine 2/3
│   └── 02_glsl_reference_shaders.md           # Colección de shaders GLSL listos para usar
├── platforms/                                 # Especificaciones de hardware, consolas y APIs
│   └── 01_console_hardware_constraints.md    # Restricciones técnicas de PS3, 360, Vita y PSP
└── templates/                                 # Plantillas estándar para nuevos documentos
    └── ai_document_template.md                # Plantilla base para que cualquier IA añada contenido
```

---

## 🤖 Guía para Asistentes de IA (AI Guidelines)

Cuando un asistente de IA genere o proponga nueva documentación técnica para Xenity Engine, debe seguir estas reglas:

1. **Ubicación modular**:
   - Temas de renderizado, shaders, luz y materiales van en `graphics/`.
   - Temas de arquitectura multiplataforma, memoria y consolas van en `platforms/`.
   - Físicas, audio, networking o lógica de gameplay deben crear su respectiva carpeta (ej: `physics/`, `network/`).
2. **Formato estándar**:
   - Usar la plantilla ubicada en [`templates/ai_document_template.md`](templates/ai_document_template.md).
   - Incluir título claro, objetivo técnico, algoritmos/fórmulas matemáticas en LaTeX (`$$...$$`), snippets de código compilables y enlaces de referencia.
3. **Control de Versiones**:
   - Siempre confirmar los cambios con `git add` y `git commit` describiendo el aporte de la IA en el mensaje (ej: `docs(ai): add PBR BRDF formulation reference`).

---

## 📑 Índice de Documentos Actuales

### 🎨 Gráficos & Shaders
* [**01. Plan Maestro CryEngine 2/3**](graphics/01_cryengine_master_plan.md): Adaptación ligera de SSAO, rayos crepusculares, niebla atmosférica y agua tropical sin colapsar el rendimiento.
* [**02. Shaders GLSL de Referencia**](graphics/02_glsl_reference_shaders.md): Implementaciones completas y funcionales en GLSL (God rays 2D, Niebla exponencial, Fresnel Schlick, Tonemapping ACES).

### 🎮 Plataformas & Consolas
* [**01. Restricciones de Hardware en Consolas**](platforms/01_console_hardware_constraints.md): Cuadro comparativo de memoria, GPUs, limitaciones de shaders y estrategias de optimización para PS3, Xbox 360, PS Vita, PSP y PC.
