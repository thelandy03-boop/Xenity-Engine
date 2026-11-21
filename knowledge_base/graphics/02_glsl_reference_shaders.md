# Shaders GLSL de Referencia: Efectos CryEngine para Xenity Engine
> **Módulo**: Graphics / Shaders
> **Autor / Generado por**: Antigravity AI
> **Fecha**: 2026-11-20
> **Estado**: Aprobado / Listo para Integración

---

## 1. Shader de Rayos Crepusculares (Screen-Space God Rays)

```glsl
// god_rays.frag
#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform sampler2D u_OcclusionTexture; // Textura del sol y siluetas opacas
uniform vec2 u_SunPos;                // Coordenadas normalizadas [0,1] del sol en pantalla
uniform float u_Density = 0.84;
uniform float u_Weight = 0.58;
uniform float u_Decay = 0.96;
uniform float u_Exposure = 0.25;

const int NUM_SAMPLES = 10; // 6 para PS3/Vita/360, 12 para PC

void main() {
    vec2 deltaTexCoord = (TexCoords - u_SunPos) * (1.0 / float(NUM_SAMPLES)) * u_Density;
    vec2 currentCoord = TexCoords;
    vec3 color = vec3(0.0);
    float illuminationDecay = 1.0;

    for (int i = 0; i < NUM_SAMPLES; i++) {
        currentCoord -= deltaTexCoord;
        vec3 sampleColor = texture(u_OcclusionTexture, currentCoord).rgb;
        sampleColor *= illuminationDecay * u_Weight;
        color += sampleColor;
        illuminationDecay *= u_Decay;
    }

    FragColor = vec4(color * u_Exposure, 1.0);
}
```

---

## 2. Función de Niebla Atmosférica por Altura (Exponential Height Fog)

```glsl
// height_fog.glsl
vec3 ApplyHeightFog(vec3 surfaceColor, vec3 worldPos, vec3 cameraPos, vec3 sunDir, vec3 fogColor, vec3 sunFogColor) {
    vec3 viewDir = worldPos - cameraPos;
    float dist = length(viewDir);
    viewDir = normalize(viewDir);

    float fogDensity = 0.003;
    float fogHeightFalloff = 0.08;
    float fogHeight = 0.0;

    float h = worldPos.y - fogHeight;
    float fogAmount = (fogDensity / fogHeightFalloff) * exp(-cameraPos.y * fogHeightFalloff) * (1.0 - exp(-dist * viewDir.y * fogHeightFalloff)) / max(viewDir.y, 0.0001);
    fogAmount = clamp(1.0 - exp(-fogAmount), 0.0, 1.0);

    float sunScattering = max(dot(viewDir, sunDir), 0.0);
    vec3 finalFogColor = mix(fogColor, sunFogColor, pow(sunScattering, 4.0));

    return mix(surfaceColor, finalFogColor, fogAmount);
}
```

---

## 3. Shader de Agua con Fresnel de Schlick (Tropical Ocean)

```glsl
// ocean_water.frag
#version 330 core
in vec3 v_WorldPos;
in vec2 v_TexCoords;
out vec4 FragColor;

uniform sampler2D u_NormalMap1;
uniform sampler2D u_NormalMap2;
uniform samplerCube u_Skybox;
uniform vec3 u_CameraPos;
uniform float u_Time;

void main() {
    vec3 V = normalize(u_CameraPos - v_WorldPos);

    vec2 uv1 = v_TexCoords * 4.0 + vec2(u_Time * 0.03, u_Time * 0.02);
    vec2 uv2 = v_TexCoords * 8.0 - vec2(u_Time * 0.02, u_Time * 0.04);

    vec3 n1 = texture(u_NormalMap1, uv1).xyz * 2.0 - 1.0;
    vec3 n2 = texture(u_NormalMap2, uv2).xyz * 2.0 - 1.0;
    vec3 N = normalize(vec3(n1.xy + n2.xy, n1.z));

    float R0 = 0.02;
    float fresnel = R0 + (1.0 - R0) * pow(clamp(1.0 - dot(V, N), 0.0, 1.0), 5.0);

    vec3 deepWaterColor = vec3(0.01, 0.18, 0.28);
    vec3 skyReflection = texture(u_Skybox, reflect(-V, N)).rgb;

    vec3 waterColor = mix(deepWaterColor, skyReflection, fresnel);
    FragColor = vec4(waterColor, clamp(fresnel + 0.35, 0.0, 1.0));
}
```

---

## 4. Tonemapping ACES (Naughty Dog & Crytek)

```glsl
// tonemap_aces.glsl
vec3 ACESFilm(vec3 x) {
    float a = 2.51;
    float b = 0.03;
    float c = 2.43;
    float d = 0.59;
    float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}
```
