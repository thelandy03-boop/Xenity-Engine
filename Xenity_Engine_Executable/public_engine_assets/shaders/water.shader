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

// ACES Filmic tonemapping
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
	float t = time * 0.040;

	// Authentic multi-scale wave coordinates (realistic scale: ~8m, ~3m, ~1m)
	vec2 uv1 = (v_FragPos.xz * 0.12) + vec2(t * 0.50, t * 0.35);
	vec2 uv2 = (v_FragPos.xz * 0.35) + vec2(-t * 0.38, t * 0.48);
	vec2 uv3 = (v_FragPos.xz * 0.85) + vec2(t * 0.22, -t * 0.60);

	// Hardware trilinear sampled normal maps (MIP-maps automatically eliminate moiré)
	vec3 n1 = texture2D(diffuse, uv1).rgb * 2.0 - 1.0;
	vec3 n2 = texture2D(diffuse, uv2).rgb * 2.0 - 1.0;
	vec3 n3 = texture2D(diffuse, uv3).rgb * 2.0 - 1.0;

	// Blend multi-frequency normal vectors
	vec2 blendedTangent = n1.xy * 0.50 + n2.xy * 0.35 + n3.xy * 0.15;
	float tangentZ = max(0.1, sqrt(max(0.0, 1.0 - dot(blendedTangent, blendedTangent))));
	vec3 waveTangent = vec3(blendedTangent, tangentZ);

	// Convert tangent space to world normal (world Y is up)
	// Normal tilt strength: 0.45 gives crisp, organic water surface definition
	vec3 norm = normalize(vec3(waveTangent.x * 0.45, waveTangent.z, waveTangent.y * 0.45));

	vec3 viewVec = viewPos - v_FragPos;
	float dist = length(viewVec);
	vec3 viewDir = normalize(viewVec);

	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Fresnel factor (Schlick): glancing angles reflect sky, top-down view is transparent
	float fresnel = 0.04 + 0.96 * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 4.0);

	// Caribbean Sky Dome Reflection
	vec3 reflectDir = reflect(-viewDir, norm);
	vec3 skyHorizon = vec3(0.70, 0.84, 0.96);
	vec3 skyZenith = vec3(0.15, 0.42, 0.78);
	vec3 skyReflectColor = mix(skyHorizon, skyZenith, clamp(reflectDir.y, 0.0, 1.0));

	// Vibrant Tropical Ocean colors:
	// Shallow: glowing Caribbean cyan
	// Mid: tropical emerald turquoise
	// Deep: rich azure cobalt
	vec3 shallowColor = vec3(0.06, 0.82, 0.88);
	vec3 midColor = vec3(0.02, 0.48, 0.65);
	vec3 deepColor = vec3(0.01, 0.12, 0.30);

	float depthMix = clamp(cosTheta * 0.70 + 0.15, 0.0, 1.0);
	vec3 waterBase = mix(deepColor, mix(midColor, shallowColor, depthMix), depthMix);

	// Shimmering organic sunlight caustics on wave surfaces
	float caustic1 = pow(clamp(n1.z * 0.5 + 0.5, 0.0, 1.0), 4.0);
	float caustic2 = pow(clamp(n2.z * 0.5 + 0.5, 0.0, 1.0), 3.0);
	float causticPattern = (caustic1 * 0.6 + caustic2 * 0.4);
	waterBase += vec3(0.12, 0.45, 0.50) * causticPattern * 0.40;

	// Blend water body with sky reflection via Fresnel
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.75);

	// Sun directional specular glitter & reflection path
	if (lightIndices.usedDirectionalLightCount > 0)
	{
		DirectionalLight sun = directionalLights[lightIndices.directionalLightsIndices[0].x];
		vec3 lightDir = normalize(-sun.direction);
		vec3 halfDir = normalize(lightDir + viewDir);

		float NdotH = max(dot(norm, halfDir), 0.0);
		// Micro-sparkles + broad golden sun path
		float sharpGlint = pow(NdotH, 256.0) * 2.8;
		float broadGlint = pow(NdotH, 40.0) * 0.50;
		surfaceColor += (sharpGlint + broadGlint) * sun.color * 1.6;

		// Sun disc reflection on the sky dome
		float sunReflectDot = max(dot(reflectDir, lightDir), 0.0);
		surfaceColor += vec3(1.0, 0.95, 0.82) * pow(sunReflectDot, 64.0) * 1.2 * fresnel;
	}

	vec3 finalColor = surfaceColor;

	// Atmospheric distance fog
	if (fogParams.x > 0.5)
	{
		float fogDensity = fogParams.y > 0.0 ? fogParams.y : 0.0004;
		float fogAmount = clamp(1.0 - exp(-dist * fogDensity), 0.0, 0.90);
		finalColor = mix(finalColor, fogColor, fogAmount);
	}

	finalColor = ACESFilm(finalColor);

	// Translucent water: 65% in shallows to see baseplate through water, 90% at glancing angles
	float alpha = clamp(0.68 + fresnel * 0.24, 0.0, 0.92);
	gl_FragColor = vec4(finalColor, alpha);
}
