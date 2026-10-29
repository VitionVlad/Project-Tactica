#version 450

layout(location = 0) out vec4 outColor;
layout(location = 0) in  vec2 fuv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 t;
    mat4 r;
    mat4 s;
    vec4 addinfo;
    vec4 rtinfo;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedMVP[10];
    mat4 defferedMVPInverse[10];
    vec4 deffpos[10];
    vec4 deffrot[10];
} dmi;

layout(binding = 3)  uniform texture2D      texTexture;
layout(binding = 4)  uniform texture2DArray defferedTexture;
layout(binding = 5)  uniform texture2DArray defferedDepthTexture;
layout(binding = 6)  uniform texture2DArray shadowTexture;
layout(binding = 7)  uniform sampler        imageSampler;
layout(binding = 8)  uniform sampler        attachmentSampler;

const float PI = 3.14159265359;

const float EXPOSURE        = 1.0;
const float LIGHT_INTENSITY = 3.0;
const vec3  AMBIENT_SKY     = vec3(0.0004);
const vec3  AMBIENT_GROUND  = vec3(0.00015);
const float ATTEN_MIN_DIST2 = 0.1;

#define SHADOW_TAPS 8
const float SHADOW_SOFTNESS     = 2.0;     
const float SHADOW_BIAS         = 0.0;
const float SHADOW_PLANE_BIAS_MAX = 0.0015;
const float SPOT_EDGE_FADE      = 0.04;

#define VOLUMETRICS 1
#define VOL_STEPS   5
const float VOL_DENSITY  = 0.05;
const float VOL_MAX_DIST = 40.0;
const float VOL_ANISO    = 0.35;
const float VOL_CULL     = 0.0008;

const float GA_COS = -0.7373688780783197;
const float GA_SIN =  0.6754902942615238;

float DistributionGGX(vec3 N, vec3 H, float roughness){
  float a      = roughness*roughness;
  float a2     = a*a;
  float NdotH  = max(dot(N, H), 0.0);
  float NdotH2 = NdotH*NdotH;
  float num   = a2;
  float denom = (NdotH2 * (a2 - 1.0) + 1.0);
  denom = PI * denom * denom;
  return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness){
  float r = (roughness + 1.0);
  float k = (r*r) / 8.0;
  float num   = NdotV;
  float denom = NdotV * (1.0 - k) + k;
  return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness){
  float NdotV = max(dot(N, V), 0.0);
  float NdotL = max(dot(N, L), 0.0);
  float ggx2  = GeometrySchlickGGX(NdotV, roughness);
  float ggx1  = GeometrySchlickGGX(NdotL, roughness);
  return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0){
  return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float interleavedGradientNoise(vec2 p){
  return fract(52.9829189 * fract(dot(p, vec2(0.06711056, 0.00583715))));
}

vec3 ACESFilm(vec3 x){
  const float a = 2.51;
  const float b = 0.03;
  const float c = 2.43;
  const float d = 0.59;
  const float e = 0.14;
  return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

vec4 shadowCoord(int idx, vec3 P){
  vec4 s = smi.shadowViews[idx] * vec4(P, 1.0);
  float iw = 1.0 / s.w;
  return vec4(s.xy * iw * 0.5 + 0.5, s.z * iw, s.w);
}

float shadowFrustumMask(vec4 sc, bool positional){
  if (!positional) return 1.0;
  if (sc.w <= 0.0 || sc.z > 1.0 || sc.z < -1.0) return 0.0;
  vec2 e = min(sc.xy, 1.0 - sc.xy);
  return smoothstep(0.0, SPOT_EDGE_FADE, min(e.x, e.y));
}

float shadowSoft(vec3 P, vec3 dPdx, vec3 dPdy, int idx, bool positional, float bias){
  vec4 sc = shadowCoord(idx, P);
  float mask = shadowFrustumMask(sc, positional);
  if (mask <= 0.0) return 0.0;

  vec3 gx = shadowCoord(idx, P + dPdx).xyz - sc.xyz;
  vec3 gy = shadowCoord(idx, P + dPdy).xyz - sc.xyz;
  float det = gx.x * gy.y - gx.y * gy.x;
  vec2 dzduv = vec2(0.0);
  if (abs(det) > 1e-12) {
    dzduv = vec2(gy.y * gx.z - gx.y * gy.z,
                 gx.x * gy.z - gy.x * gx.z) / det;
  }

  float radius = SHADOW_SOFTNESS / mi.resolutions.z;
  float rot = interleavedGradientNoise(gl_FragCoord.xy) * 6.2831853;
  vec2 dir = vec2(cos(rot), sin(rot));

  float lit = 0.0;
  for (int k = 0; k < SHADOW_TAPS; k++) {
    float r = sqrt((float(k) + 0.5) / float(SHADOW_TAPS));
    vec2 o = dir * (r * radius);
    float planeZ = clamp(dot(dzduv, o), -SHADOW_PLANE_BIAS_MAX, SHADOW_PLANE_BIAS_MAX);
    float sd = textureLod(sampler2DArray(shadowTexture, attachmentSampler), vec3(sc.xy + o, float(idx)), 0.0).r;
    lit += (sc.z + planeZ - bias < sd) ? 1.0 : 0.0;
    dir = vec2(dir.x * GA_COS - dir.y * GA_SIN, dir.x * GA_SIN + dir.y * GA_COS);
  }
  return (lit / float(SHADOW_TAPS)) * mask;
}

float shadowTap(vec3 P, int idx, bool positional){
  vec4 sc = shadowCoord(idx, P);
  float mask = shadowFrustumMask(sc, positional);
  if (mask <= 0.0) return 0.0;
  float sd = textureLod(sampler2DArray(shadowTexture, attachmentSampler), vec3(sc.xy, float(idx)), 0.0).r;
  return (sc.z - SHADOW_BIAS < sd) ? mask : 0.0;
}

float phaseHG(float cosTheta){
  float g2 = VOL_ANISO * VOL_ANISO;
  return (1.0 - g2) / pow(1.0 + g2 - 2.0 * VOL_ANISO * cosTheta, 1.5);
}

vec3 volumetricScatter(vec3 camPos, vec3 P, int shIdx, bool positional, vec3 lightPos, vec3 lightRadiance, float jitter){
  vec3 ray = P - camPos;
  float len = length(ray);
  if (len < 1e-4) return vec3(0.0);
  vec3 rd = ray / len;
  len = min(len, VOL_MAX_DIST);

  // cull lights that are too far from the view ray to matter
  if (positional) {
    float tc = clamp(dot(lightPos - camPos, rd), 0.0, len);
    vec3 dc = lightPos - (camPos + rd * tc);
    float att = 1.0 / (dot(dc, dc) + 1.0);
    if (max(lightRadiance.r, max(lightRadiance.g, lightRadiance.b)) * att * VOL_DENSITY * len < VOL_CULL) {
      return vec3(0.0);
    }
  }

  float stepLen = len / float(VOL_STEPS);
  float t = stepLen * jitter;
  float acc = 0.0;
  for (int s = 0; s < VOL_STEPS; s++) {
    vec3 p = camPos + rd * t;
    vec3 L;
    float att = 1.0;
    if (positional) {
      vec3 toL = lightPos - p;
      float d2 = dot(toL, toL);
      L = toL * inversesqrt(max(d2, 1e-6));
      att = 1.0 / (d2 + 1.0);
    } else {
      L = normalize(-lightPos);
    }
    acc += shadowTap(p, shIdx, positional) * att * phaseHG(dot(rd, L));
    t += stepLen;
  }
  return lightRadiance * (acc * stepLen * VOL_DENSITY);
}

vec3 PBR(vec3 norm, vec3 albedo, float metallic, float roughness, float ao, vec3 WorldPos){
  vec3 N = normalize(norm);
  vec3 camPos = dmi.deffpos[0].xyz;
  vec3 V = normalize(camPos - WorldPos);
  roughness = clamp(roughness, 0.04, 1.0);

  vec3 F0 = mix(vec3(0.04), albedo, metallic);
  vec3 Lo = vec3(0.0);
  vec3 fog = vec3(0.0);

  vec3 dPdx = dFdx(WorldPos);
  vec3 dPdy = dFdy(WorldPos);
  float jitter = interleavedGradientNoise(gl_FragCoord.xy + vec2(17.0, 31.0));

  int lightCount = int(mi.lightinfo.w);
  int scs = 0;
  for (int i = 0; i < lightCount; i++) {
    bool positional = smi.lightpos[i].w == 1.0;

    int shIdx = -1;
    if (smi.lightcol[i].w != 0.0) {
      shIdx = scs;
      scs++;
    }

    vec3 L;
    float dist2 = 1.0;
    if (positional) {
      vec3 toL = smi.lightpos[i].xyz - WorldPos;
      dist2 = max(dot(toL, toL), ATTEN_MIN_DIST2);
      L = normalize(toL);
    } else {
      L = normalize(-smi.lightpos[i].xyz);
    }

    vec3 radiance = smi.lightcol[i].xyz * (LIGHT_INTENSITY / dist2);

    float NdotL = max(dot(N, L), 0.0);
    if (NdotL > 0.0) {
      vec3 H = normalize(V + L);
      float NDF = DistributionGGX(N, H, roughness);
      float G   = GeometrySmith(N, V, L, roughness);
      vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);
      vec3 kD   = (vec3(1.0) - F) * (1.0 - metallic);
      vec3 specular = (NDF * G * F) / (4.0 * max(dot(N, V), 0.0) * NdotL + 0.0001);

      float shadow = 1.0;
      if (shIdx >= 0) {
        shadow = shadowSoft(WorldPos, dPdx, dPdy, shIdx, positional, SHADOW_BIAS);
      }
      Lo += (kD * albedo / PI + specular) * radiance * NdotL * shadow;
    }

#if VOLUMETRICS
    if (shIdx >= 0) {
      fog += volumetricScatter(camPos, WorldPos, shIdx, positional, smi.lightpos[i].xyz,
                               smi.lightcol[i].xyz * LIGHT_INTENSITY/10.0, jitter);
    }
#endif
  }

  vec3 ambient = mix(AMBIENT_GROUND, AMBIENT_SKY, N.y * 0.5 + 0.5) * albedo * ao;

  vec3 color = ambient + Lo + fog;
  color = ACESFilm(color * EXPOSURE);
  color = pow(color, vec3(1.0 / 2.2));

  color += (interleavedGradientNoise(gl_FragCoord.xy + vec2(5.0, 11.0)) - 0.5) / 255.0;
  return color;
}

vec3 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat){
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth, 1.0);
  vec4 viewSpacePosition = inversemat * clipSpacePosition;
  viewSpacePosition /= viewSpacePosition.w;
  return viewSpacePosition.xyz;
}

float near = 0.1; 
float far  = 100.0; 

float LinearizeDepth(float d) {         
    return near * far / (far + d * (far - near));
}

void main() {
  vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  //d = LinearizeDepth(d);

  vec3 albedo = pow(texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb, vec3(2.2));

  vec4 rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgba;
  vec3 normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
  vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

  vec4 op = vec4(PBR(normal, albedo, rma.x, rma.y, 1.0, wrldpos), 1.0);

  bool hlighted = rma.a > 0.1;
  float fac = hlighted ? 0.85 : 1.0;
  albedo = rma.a > 1.0 ? vec3(1.0) : albedo;

  op = mix(vec4(albedo, 1.0), op, fac);

  //float dst = smoothstep(0.0, 30.0, distance(mi.addinfo.yz, wrldpos.xz));

  //op = mix(vec4(smi.lightcol[0].xyz, 1.0), op, 1.0-max(min(dst, 1.0), 0.0));

  //if(mi.addinfo.w < 1.0){
  //  op.rg *= max(min(mi.addinfo.w, 1.0), 0.25);
  //}else if(mi.addinfo.w >= 5.0){
  //  op.rgb = mix(op.rgb, vec3(0.0), max(min(mi.addinfo.w-10.0, 1.0), 0.0));
  //}else{
  //  op.rgb = mix(op.rgb, vec3(1.0), max(min(mi.addinfo.w-1.0, 1.0), 0.0));
  //}

  outColor = op;
}
