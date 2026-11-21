//-------------- {pc}
//-------------- {vertex}

#version 330

layout(location = 0) in vec2 a_TexCoord;
layout(location = 1) in vec3 a_Normal;
layout(location = 2) in vec3 a_Position;
layout(location = 3) in vec4 a_Color;

out vec2 v_TexCoord;
out vec3 v_Normal;
out vec3 v_FragPos;
out vec4 v_Color;

uniform mat4 model;
uniform mat4 MVP;
uniform mat3 normalMatrix;

void main()
{
	gl_Position = MVP * vec4(a_Position, 1.0);
	v_FragPos = vec3(model * vec4(a_Position, 1.0));
	v_TexCoord = a_TexCoord;
	v_Normal = normalMatrix * a_Normal;
	v_Color = a_Color;
}

//-------------- {fragment}

#version 330

in vec3 v_Normal;
in vec3 v_FragPos;
in vec4 v_Color;
in vec2 v_TexCoord;

uniform vec4 color;
uniform vec2 tiling;
uniform vec2 offset;
uniform vec3 ambientLight;
uniform vec3 viewPos;
uniform vec4 fogParams;
uniform vec3 fogColor;
uniform float time;

struct DirectionalLight 
{
	vec3 direction;
	vec3 color;
};

#define NR_DIRECTIONAL_LIGHTS 10
uniform DirectionalLight directionalLights[NR_DIRECTIONAL_LIGHTS];

layout(std140) uniform LightIndices
{
	int usedPointLightCount;
	int usedSpotLightCount;
	int usedDirectionalLightCount;
	int padding0;

	ivec4 pointLightsIndices[10];
	ivec4 spotLightsIndices[10];
	ivec4 directionalLightsIndices[10];
} lightIndices;

// CryEngine 2/3 Multi-Harmonic Ocean Wave Simulation with Distance LOD Filtering
vec3 CalculateWaveNormal(vec2 p, float t, float dist)
{
	// Distance LOD: Attenuate high frequency ripples with distance to eliminate moiré aliasing
	float lodMedium = clamp(1.0 - dist * 0.003, 0.0, 1.0);
	float lodFine = clamp(1.0 - dist * 0.015, 0.0, 1.0);

	// Octave 1: Primary oceanic swell (L ~ 50m)
	vec2 d1 = vec2(0.85, 0.52);
	float ph1 = dot(p, d1) * 0.06 + t * 0.70;

	// Octave 2: Secondary cross chop (L ~ 25m)
	vec2 d2 = vec2(-0.55, 0.83);
	float ph2 = dot(p, d2) * 0.14 + t * 1.10;

	// Octave 3: Surface ripples (L ~ 10m, fades smoothly by 300m)
	vec2 d3 = vec2(0.70, -0.71);
	float ph3 = dot(p, d3) * 0.35 + t * 1.80;

	// Octave 4: Capillary sparkle (L ~ 3m, fades smoothly by 70m)
	vec2 d4 = vec2(-0.90, 0.43);
	float ph4 = dot(p, d4) * 0.90 + t * 2.60;

	vec2 grad = vec2(0.0);
	grad += d1 * cos(ph1) * 0.16;
	grad += d2 * cos(ph2) * (0.12 * lodMedium);
	grad += d3 * cos(ph3) * (0.08 * lodMedium);
	grad += d4 * cos(ph4) * (0.04 * lodFine);

	return normalize(vec3(-grad.x, 1.0, -grad.y));
}

// Schlick's approximation for Fresnel reflectance
float SchlickFresnel(float cosTheta, float F0)
{
	return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

// ACES Filmic tonemapping curve (CryEngine 3)
vec3 ACESFilm(vec3 x)
{
	float a = 2.51;
	float b = 0.03;
	float c = 2.43;
	float d = 0.59;
	float e = 0.14;
	return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

void main()
{
	float t = time * 1.1;
	vec3 viewVec = viewPos - v_FragPos;
	float dist = length(viewVec);
	vec3 viewDir = normalize(viewVec);

	// Distance-filtered wave normal (eliminates moiré horizontal banding completely)
	vec3 waveNorm = CalculateWaveNormal(v_FragPos.xz, t, dist);
	vec3 norm = normalize(mix(normalize(v_Normal), waveNorm, 0.85));

	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Schlick Fresnel factor (R0 = 0.02 for clean ocean water)
	float fresnel = SchlickFresnel(cosTheta, 0.02);

	// Atmospheric Sky Dome Reflection (CryEngine 2/3)
	vec3 reflectDir = reflect(-viewDir, norm);
	vec3 skyHorizon = vec3(0.68, 0.82, 0.96);
	vec3 skyZenith = vec3(0.12, 0.38, 0.75);
	vec3 skyReflectColor = mix(skyHorizon, skyZenith, clamp(reflectDir.y, 0.0, 1.0));

	// Tropical CryEngine water body colors:
	// Shallows: vibrant Caribbean turquoise/cyan (crystal clear)
	// Mid-depth: emerald cyan
	// Deep ocean: rich cobalt navy blue
	vec3 shallowColor = vec3(0.04, 0.72, 0.78);
	vec3 midColor = vec3(0.02, 0.38, 0.54);
	vec3 deepColor = vec3(0.01, 0.07, 0.22);

	float depthMix = clamp(cosTheta * 0.5 + 0.3, 0.0, 1.0);
	vec3 waterBase = mix(deepColor, mix(midColor, shallowColor, depthMix), depthMix);
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.82);

	// Crest Foam (Whitecaps) with distance fade (only visible near camera, no horizon streaks)
	float waveSteepness = length(waveNorm.xz);
	float foamDistFade = clamp(1.0 - dist * 0.015, 0.0, 1.0);
	float foamFactor = smoothstep(0.16, 0.26, waveSteepness) * foamDistFade;
	vec3 foamColor = vec3(0.96, 0.98, 1.0);
	surfaceColor = mix(surfaceColor, foamColor, foamFactor * 0.60);

	// Directional sun specular glitter & subsurface scattering
	vec3 sunGlitter = vec3(0.0);
	vec3 sssColor = vec3(0.0);
	if (lightIndices.usedDirectionalLightCount > 0)
	{
		DirectionalLight sun = directionalLights[lightIndices.directionalLightsIndices[0].x];
		vec3 lightDir = normalize(-sun.direction);
		vec3 halfDir = normalize(lightDir + viewDir);

		float NdotH = max(dot(norm, halfDir), 0.0);
		// Crisp micro-facet sparkles + broad golden path
		float sharpGlint = pow(NdotH, 256.0) * 2.2;
		float broadGlint = pow(NdotH, 48.0) * 0.60;
		sunGlitter = (sharpGlint + broadGlint) * sun.color * 1.6;

		// CryEngine Subsurface Scattering through wave crests (emerald glow when backlit)
		float sss = pow(max(dot(viewDir, -lightDir), 0.0), 3.0) * max(norm.y, 0.0);
		sssColor = vec3(0.04, 0.65, 0.55) * sss * 0.60 * sun.color;

		// Sun disc reflection on the sky dome
		float sunReflectDot = max(dot(reflectDir, lightDir), 0.0);
		surfaceColor += vec3(1.0, 0.95, 0.80) * pow(sunReflectDot, 64.0) * 1.2 * fresnel;
	}

	vec3 finalColor = surfaceColor + sunGlitter + sssColor;

	// Atmospheric exponential height fog
	if (fogParams.x > 0.5)
	{
		float fogDensity = fogParams.y > 0.0 ? fogParams.y : 0.0004;
		float fogAmount = clamp(1.0 - exp(-dist * fogDensity), 0.0, 0.90);
		finalColor = mix(finalColor, fogColor, fogAmount);
	}

	finalColor = ACESFilm(finalColor);
	float alpha = clamp(0.72 + fresnel * 0.24, 0.0, 0.96);
	gl_FragColor = vec4(finalColor, alpha);
}
