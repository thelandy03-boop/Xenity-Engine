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

// Multi-octave wave normal calculation with crisp CryEngine surface ripples
vec3 CalculateWaveNormal(vec2 p, float t)
{
	vec2 p1 = p * 0.4 + vec2(t * 0.3, t * 0.15);
	vec2 p2 = p * 0.9 - vec2(t * 0.25, t * 0.4);
	vec2 p3 = p * 2.2 + vec2(t * 0.5, -t * 0.3);
	vec2 p4 = p * 4.5 - vec2(t * 0.7, t * 0.6);

	float dx = (cos(p1.x + p1.y * 1.2) * 0.40
	          + cos(p2.x * 1.3 - p2.y) * 0.35
	          + cos(p3.x + p3.y * 1.5) * 0.30
	          + cos(p4.x * 1.4 - p4.y) * 0.25);
	float dy = (cos(p1.x + p1.y * 1.2) * 0.45
	          - cos(p2.x * 1.3 - p2.y) * 0.35
	          + cos(p3.x + p3.y * 1.5) * 0.45
	          - cos(p4.x * 1.4 - p4.y) * 0.30);

	return normalize(vec3(-dx * 0.35, 1.0, -dy * 0.35));
}

// Schlick's approximation for Fresnel reflectance
float SchlickFresnel(float cosTheta, float F0)
{
	return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

void main()
{
	vec2 uv = (v_TexCoord * tiling) + offset;
	float t = time * 1.2;

	// Multi-octave wave normal perturbation
	vec3 waveNorm = CalculateWaveNormal(v_FragPos.xz, t);
	vec3 norm = normalize(mix(normalize(v_Normal), waveNorm, 0.90));

	vec3 viewDir = normalize(viewPos - v_FragPos);
	float cosTheta = max(dot(norm, viewDir), 0.0);

	// Schlick Fresnel factor (R0 = 0.02 for clean ocean water)
	float fresnel = SchlickFresnel(cosTheta, 0.02);

	// Tropical CryEngine color gradient:
	// Shallows: vibrant Caribbean turquoise/cyan
	// Deep: deep ocean navy blue
	vec3 shallowColor = vec3(0.06, 0.55, 0.68);
	vec3 deepColor = vec3(0.01, 0.10, 0.25);
	vec3 skyReflectColor = vec3(0.65, 0.78, 0.95);

	vec3 waterBase = mix(deepColor, shallowColor, clamp(cosTheta * 0.7 + 0.3, 0.0, 1.0));
	vec3 surfaceColor = mix(waterBase, skyReflectColor, fresnel * 0.75);

	// Sun glitter (high-gloss Blinn-Phong specular on wave facets)
	vec3 sunGlitter = vec3(0.0);
	for (int i = 0; i < lightIndices.usedDirectionalLightCount; i++)
	{
		DirectionalLight sun = directionalLights[lightIndices.directionalLightsIndices[i].x];
		vec3 lightDir = normalize(-sun.direction);
		vec3 halfDir = normalize(lightDir + viewDir);
		float specAngle = max(dot(norm, halfDir), 0.0);
		float spec = pow(specAngle, 64.0); // sharp sparkling reflection
		sunGlitter += spec * sun.color * 1.6;
	}

	vec3 finalColor = surfaceColor + sunGlitter;

	// Subtle atmospheric height fog
	if (fogParams.x > 0.5)
	{
		vec3 viewVec = v_FragPos - viewPos;
		float dist = length(viewVec);
		float fogDensity = fogParams.y > 0.0 ? fogParams.y : 0.0004;
		float fogAmount = clamp(1.0 - exp(-dist * fogDensity), 0.0, 0.90);
		finalColor = mix(finalColor, fogColor, fogAmount);
	}

	float alpha = clamp(0.50 + fresnel * 0.45, 0.0, 0.92);
	gl_FragColor = vec4(finalColor, alpha);
}
