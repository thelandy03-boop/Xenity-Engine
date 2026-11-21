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
uniform sampler2D diffuse;

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
	// Dual scrolling normal map animation
	float t = time * 0.035;
	
	// Layer 1: broad oceanic swell moving northeast
	vec2 uv1 = (v_FragPos.xz * 0.030) + vec2(t * 0.60, t * 0.40);
	// Layer 2: finer cross waves moving southeast
	vec2 uv2 = (v_FragPos.xz * 0.065) + vec2(-t * 0.40, t * 0.55);

	// Sample hardware filtered normal map (MIP-maps automatically eliminate moiré and distant noise)
	vec3 n1 = texture2D(diffuse, uv1).rgb * 2.0 - 1.0;
	vec3 n2 = texture2D(diffuse, uv2).rgb * 2.0 - 1.0;

	// Blend normals in tangent space (where Z is up towards sky)
	vec3 waveTangent = normalize(vec3(n1.xy + n2.xy, n1.z * n2.z * 1.5));

	// Convert tangent normal to world normal (Y is up in world space)
	// Normal strength factor: 0.35 gives gentle, realistic tropical ripples
	vec3 norm = normalize(vec3(waveTangent.x * 0.35, waveTangent.z, waveTangent.y * 0.35));

	vec3 viewVec = viewPos - v_FragPos;
	float dist = length(viewVec);
	vec3 viewDir = normalize(viewVec);

	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Schlick Fresnel approximation: shallow glance has high sky reflection, top-down view sees through
	float fresnel = 0.02 + 0.98 * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 4.5);

	// Atmospheric Sky Dome Reflection (Caribbean clear sky)
	vec3 reflectDir = reflect(-viewDir, norm);
	vec3 skyHorizon = vec3(0.68, 0.82, 0.96);
	vec3 skyZenith = vec3(0.12, 0.38, 0.75);
	vec3 skyReflectColor = mix(skyHorizon, skyZenith, clamp(reflectDir.y, 0.0, 1.0));

	// Vibrant Tropical Ocean colors:
	// Shallow: glowing Caribbean cyan
	// Mid: tropical turquoise
	// Deep: azure blue
	vec3 shallowColor = vec3(0.04, 0.78, 0.85);
	vec3 midColor = vec3(0.02, 0.45, 0.62);
	vec3 deepColor = vec3(0.01, 0.12, 0.32);

	float depthMix = clamp(cosTheta * 0.65 + 0.20, 0.0, 1.0);
	vec3 waterBase = mix(deepColor, mix(midColor, shallowColor, depthMix), depthMix);
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.80);

	// Sun directional specular glitter & reflection
	if (lightIndices.usedDirectionalLightCount > 0)
	{
		DirectionalLight sun = directionalLights[lightIndices.directionalLightsIndices[0].x];
		vec3 lightDir = normalize(-sun.direction);
		vec3 halfDir = normalize(lightDir + viewDir);

		float NdotH = max(dot(norm, halfDir), 0.0);
		// Crisp micro-facet sparkles + golden reflection path
		float sharpGlint = pow(NdotH, 192.0) * 2.0;
		float broadGlint = pow(NdotH, 32.0) * 0.45;
		surfaceColor += (sharpGlint + broadGlint) * sun.color * 1.5;

		// Sun disc reflection on the sky dome
		float sunReflectDot = max(dot(reflectDir, lightDir), 0.0);
		surfaceColor += vec3(1.0, 0.95, 0.82) * pow(sunReflectDot, 64.0) * 1.0 * fresnel;
	}

	vec3 finalColor = surfaceColor;

	// Atmospheric height fog
	if (fogParams.x > 0.5)
	{
		float fogDensity = fogParams.y > 0.0 ? fogParams.y : 0.0004;
		float fogAmount = clamp(1.0 - exp(-dist * fogDensity), 0.0, 0.90);
		finalColor = mix(finalColor, fogColor, fogAmount);
	}

	finalColor = ACESFilm(finalColor);
	float alpha = clamp(0.74 + fresnel * 0.22, 0.0, 0.96);
	gl_FragColor = vec4(finalColor, alpha);
}
