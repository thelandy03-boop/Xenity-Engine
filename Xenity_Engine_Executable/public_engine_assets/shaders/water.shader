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
out float v_WaveHeight;

uniform mat4 model;
uniform mat4 MVP;
uniform mat3 normalMatrix;
uniform float time;

// Physical 3D Gerstner Wave displacement (CryEngine 2/3)
vec3 GerstnerDisplacement(vec2 worldXZ, float t)
{
	vec3 disp = vec3(0.0);

	// Wave 1: Primary oceanic swell (L ~ 50m, Amp = 0.55m)
	vec2 d1 = vec2(0.85, 0.52);
	float w1 = 0.125;
	float phi1 = dot(worldXZ, d1) * w1 + t * 0.90;
	disp.x += d1.x * (0.40 * 0.55) * cos(phi1);
	disp.z += d1.y * (0.40 * 0.55) * cos(phi1);
	disp.y += 0.55 * sin(phi1);

	// Wave 2: Secondary swell (L ~ 30m, Amp = 0.35m)
	vec2 d2 = vec2(-0.50, 0.86);
	float w2 = 0.210;
	float phi2 = dot(worldXZ, d2) * w2 + t * 1.25;
	disp.x += d2.x * (0.45 * 0.35) * cos(phi2);
	disp.z += d2.y * (0.45 * 0.35) * cos(phi2);
	disp.y += 0.35 * sin(phi2);

	// Wave 3: Cross chop (L ~ 16m, Amp = 0.22m)
	vec2 d3 = vec2(0.68, -0.73);
	float w3 = 0.400;
	float phi3 = dot(worldXZ, d3) * w3 + t * 1.80;
	disp.x += d3.x * (0.50 * 0.22) * cos(phi3);
	disp.z += d3.y * (0.50 * 0.22) * cos(phi3);
	disp.y += 0.22 * sin(phi3);

	// Wave 4: Surface wavelets (L ~ 8m, Amp = 0.12m)
	vec2 d4 = vec2(-0.75, -0.66);
	float w4 = 0.800;
	float phi4 = dot(worldXZ, d4) * w4 + t * 2.50;
	disp.x += d4.x * (0.50 * 0.12) * cos(phi4);
	disp.z += d4.y * (0.50 * 0.12) * cos(phi4);
	disp.y += 0.12 * sin(phi4);

	return disp;
}

void main()
{
	float t = time * 1.2;
	vec4 worldBase = model * vec4(a_Position, 1.0);
	vec3 disp = GerstnerDisplacement(worldBase.xz, t);

	// Displace vertex physically in 3D space
	vec3 displacedPos = a_Position;
	displacedPos.y += disp.y;
	// Lateral Gerstner displacement scaled back to local mesh scale
	displacedPos.x += disp.x * 0.00067;
	displacedPos.z += disp.z * 0.00067;

	gl_Position = MVP * vec4(displacedPos, 1.0);
	v_FragPos = vec3(model * vec4(displacedPos, 1.0));
	v_TexCoord = a_TexCoord;
	v_Normal = normalMatrix * a_Normal;
	v_Color = a_Color;
	v_WaveHeight = disp.y;
}

//-------------- {fragment}

#version 330

in vec3 v_Normal;
in vec3 v_FragPos;
in vec4 v_Color;
in vec2 v_TexCoord;
in float v_WaveHeight;

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

// 6 directional Gerstner Wavelets ranging from 50m swells down to capillary ripples
// Authentic CryEngine 2/3 tropical ocean surface normal simulation
vec3 CalculateWaveNormal(vec2 p, float t)
{
	vec2 d1 = vec2(0.85, 0.52);
	vec2 d2 = vec2(-0.50, 0.86);
	vec2 d3 = vec2(0.68, -0.73);
	vec2 d4 = vec2(-0.75, -0.66);
	vec2 d5 = vec2(0.35, 0.94);
	vec2 d6 = vec2(-0.92, 0.38);

	// Swells matching vertex displacement
	float ph1 = dot(p, d1) * 0.125 + t * 0.90;
	float ph2 = dot(p, d2) * 0.210 + t * 1.25;
	float ph3 = dot(p, d3) * 0.400 + t * 1.80;
	float ph4 = dot(p, d4) * 0.800 + t * 2.50;
	// Micro capillary ripples (sparkling highlights)
	float ph5 = dot(p, d5) * 2.200 + t * 3.60;
	float ph6 = dot(p, d6) * 4.800 + t * 5.10;

	vec2 grad = vec2(0.0);
	grad += d1 * cos(ph1) * 0.26;
	grad += d2 * cos(ph2) * 0.22;
	grad += d3 * cos(ph3) * 0.18;
	grad += d4 * cos(ph4) * 0.14;
	grad += d5 * cos(ph5) * 0.08;
	grad += d6 * cos(ph6) * 0.05;

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
	vec3 norm = normalize(mix(normalize(v_Normal), waveNorm, 0.88));

	vec3 viewDir = normalize(viewPos - v_FragPos);
	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Schlick Fresnel factor (R0 = 0.02 for clean ocean water)
	float fresnel = SchlickFresnel(cosTheta, 0.02);

	// Dynamic Atmospheric Sky Dome Reflection (CryEngine 2/3)
	vec3 reflectDir = reflect(-viewDir, norm);
	vec3 skyHorizon = vec3(0.68, 0.82, 0.96);
	vec3 skyZenith = vec3(0.10, 0.35, 0.72);
	vec3 skyReflectColor = mix(skyHorizon, skyZenith, clamp(reflectDir.y, 0.0, 1.0));

	// Tropical CryEngine water body colors:
	// Shallows: vibrant Caribbean turquoise/cyan (rich, saturated, crystal-clear)
	// Mid-depth: emerald cyan
	// Deep ocean: rich cobalt navy blue
	vec3 shallowColor = vec3(0.04, 0.72, 0.78);
	vec3 midColor = vec3(0.02, 0.38, 0.54);
	vec3 deepColor = vec3(0.01, 0.07, 0.22);

	// Depth gradient based on view angle and wave elevation
	float depthMix = clamp(cosTheta * 0.5 + 0.3 + v_WaveHeight * 0.25, 0.0, 1.0);
	vec3 waterBase = mix(deepColor, mix(midColor, shallowColor, depthMix), depthMix);
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.85);

	// Authentic CryEngine Wave Crest Foam (Whitecaps):
	// ONLY triggers at steep wave peaks (steepness > 0.70)
	// Zero foam on the rest of the ocean, eliminating milky wash!
	float waveSteepness = length(waveNorm.xz);
	float crestThreshold = 0.70 - v_WaveHeight * 0.10;
	float foamFactor = smoothstep(crestThreshold, crestThreshold + 0.22, waveSteepness);
	vec3 foamColor = vec3(0.96, 0.98, 1.0);
	surfaceColor = mix(surfaceColor, foamColor, foamFactor * 0.85);

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
		float sharpGlint = pow(NdotH, 512.0) * 3.5;
		float broadGlint = pow(NdotH, 48.0) * 0.65;
		sunGlitter += (sharpGlint + broadGlint) * sun.color * 2.0;

		// CryEngine Subsurface Scattering through wave crests (emerald glow when backlit)
		float sss = pow(max(dot(viewDir, -lightDir), 0.0), 3.0) * max(norm.y, 0.0);
		sssColor += vec3(0.05, 0.75, 0.65) * sss * 0.75 * sun.color;

		// Sun disc reflection on the sky dome
		float sunReflectDot = max(dot(reflectDir, lightDir), 0.0);
		surfaceColor += vec3(1.0, 0.92, 0.75) * pow(sunReflectDot, 128.0) * 1.5 * fresnel;
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
	// Dense, physical tropical ocean: solid presence with beautiful Fresnel reflectance
	float alpha = clamp(0.72 + fresnel * 0.24, 0.0, 0.96);
	gl_FragColor = vec4(finalColor, alpha);
}
