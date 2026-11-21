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

// 6 directional Gerstner Wavelets ranging from 35m swells down to 1.2m capillary ripples
// Authentic CryEngine 2/3 tropical ocean simulation
vec3 CalculateWaveNormal(vec2 p, float t)
{
	vec2 d1 = vec2(0.89, 0.45);
	vec2 d2 = vec2(-0.45, 0.89);
	vec2 d3 = vec2(0.60, -0.80);
	vec2 d4 = vec2(-0.75, -0.66);
	vec2 d5 = vec2(0.30, 0.95);
	vec2 d6 = vec2(-0.92, 0.38);

	// Swells
	float ph1 = dot(p, d1) * 0.15 + t * 0.7;
	float ph2 = dot(p, d2) * 0.28 + t * 1.0;
	// Medium chop
	float ph3 = dot(p, d3) * 0.65 + t * 1.5;
	float ph4 = dot(p, d4) * 1.20 + t * 2.1;
	// Micro ripples (sparkling highlights)
	float ph5 = dot(p, d5) * 2.50 + t * 3.0;
	float ph6 = dot(p, d6) * 5.00 + t * 4.2;

	vec2 grad = vec2(0.0);
	grad += d1 * cos(ph1) * 0.35;
	grad += d2 * cos(ph2) * 0.28;
	grad += d3 * cos(ph3) * 0.22;
	grad += d4 * cos(ph4) * 0.16;
	grad += d5 * cos(ph5) * 0.10;
	grad += d6 * cos(ph6) * 0.06;

	return normalize(vec3(-grad.x, 1.0, -grad.y));
}

// Schlick's approximation for Fresnel reflectance
float SchlickFresnel(float cosTheta, float F0)
{
	return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

// ACES Filmic tonemapping curve
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
	float t = time * 1.2;

	// Multi-octave wave normal perturbation
	vec3 waveNorm = CalculateWaveNormal(v_FragPos.xz, t);
	vec3 norm = normalize(mix(normalize(v_Normal), waveNorm, 0.92));

	vec3 viewDir = normalize(viewPos - v_FragPos);
	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Schlick Fresnel factor (R0 = 0.02 for clean ocean water)
	float fresnel = SchlickFresnel(cosTheta, 0.02);

	// Dynamic Atmospheric Sky Dome Reflection (CryEngine)
	vec3 reflectDir = reflect(-viewDir, norm);
	vec3 skyHorizon = vec3(0.72, 0.84, 0.96);
	vec3 skyZenith = vec3(0.12, 0.38, 0.75);
	vec3 skyReflectColor = mix(skyHorizon, skyZenith, clamp(reflectDir.y, 0.0, 1.0));

	// Tropical CryEngine water body colors:
	// Shallows: vibrant Caribbean turquoise/cyan
	// Deep: deep ocean navy blue
	vec3 shallowColor = vec3(0.05, 0.62, 0.72);
	vec3 deepColor = vec3(0.01, 0.06, 0.16);

	vec3 waterBase = mix(deepColor, shallowColor, clamp(cosTheta * 0.6 + 0.4, 0.0, 1.0));
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.85);

	// Wave crest foam
	float waveSteepness = length(waveNorm.xz);
	float foamFactor = smoothstep(0.40, 0.65, waveSteepness);
	vec3 foamColor = vec3(0.95, 0.98, 1.0);
	surfaceColor = mix(surfaceColor, foamColor, foamFactor * 0.60);

	// Directional sun specular glitter & subsurface scattering
	vec3 sunGlitter = vec3(0.0);
	vec3 sssColor = vec3(0.0);
	for (int i = 0; i < lightIndices.usedDirectionalLightCount; i++)
	{
		DirectionalLight sun = directionalLights[lightIndices.directionalLightsIndices[i].x];
		vec3 lightDir = normalize(-sun.direction);
		vec3 halfDir = normalize(lightDir + viewDir);

		float NdotH = max(dot(norm, halfDir), 0.0);
		// Crisp micro-facet sparkles + broad golden path
		float sharpGlint = pow(NdotH, 256.0) * 2.8;
		float broadGlint = pow(NdotH, 32.0) * 0.50;
		sunGlitter += (sharpGlint + broadGlint) * sun.color * 1.8;

		// Subsurface scattering through wave peaks
		float sss = pow(max(dot(viewDir, -lightDir), 0.0), 3.0) * max(norm.y, 0.0);
		sssColor += vec3(0.08, 0.65, 0.60) * sss * 0.5 * sun.color;

		// Sun disc reflection on the sky dome
		float sunReflectDot = max(dot(reflectDir, lightDir), 0.0);
		surfaceColor += vec3(1.0, 0.92, 0.75) * pow(sunReflectDot, 128.0) * 1.2 * fresnel;
	}

	vec3 finalColor = surfaceColor + sunGlitter + sssColor;

	// Atmospheric exponential height fog
	if (fogParams.x > 0.5)
	{
		vec3 viewVec = v_FragPos - viewPos;
		float dist = length(viewVec);
		float fogDensity = fogParams.y > 0.0 ? fogParams.y : 0.0004;
		float fogAmount = clamp(1.0 - exp(-dist * fogDensity), 0.0, 0.90);
		finalColor = mix(finalColor, fogColor, fogAmount);
	}

	finalColor = ACESFilm(finalColor);
	// Crystal-clear tropical water transparency: see-through in shallows, reflective at distance
	float alpha = clamp(0.28 + fresnel * 0.60, 0.0, 0.92);
	gl_FragColor = vec4(finalColor, alpha);
}
