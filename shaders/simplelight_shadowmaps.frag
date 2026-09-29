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

layout(binding = 3)  uniform texture3D      texTexture;
layout(binding = 4)  uniform texture2DArray defferedTexture;
layout(binding = 5)  uniform texture2DArray defferedDepthTexture;
layout(binding = 6)  uniform texture2DArray shadowTexture;
layout(binding = 7)  uniform sampler        imageSampler;
layout(binding = 8)  uniform sampler        attachmentSampler;
layout(binding = 9)  uniform texture2D      noiseTexture;
layout(binding = 10) uniform sampler        noiseSampler;

const float PI           = 3.14159265359;
const float GOLDEN_RATIO = 0.61803398875;
const int   MAX_STEPS    = 128;
const float EPS          = 1e-5;
const float SS_THICKNESS = 0.3;
const vec3  SKY_COLOR    = vec3(0.02, 0.02, 0.05);
//const vec3  SKY_COLOR    = vec3(0.67, 0.84, 0.89);
const float FIREFLY_CAP  = 10.0;                  

vec3 SampleAlbedo(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
}
vec3 SampleRMA(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
}
vec3 SampleNormal(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
}
float SampleDepth(vec2 uv) {
    return texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
}

vec3 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat) {
    vec4 clip  = vec4(uv * 2.0 - 1.0, depth, 1.0);
    vec4 world = inversemat * clip;
    return world.xyz / world.w;
}

vec3 WorldPosToUVDepth(vec3 worldPos, mat4 mvp) {
    vec4 clip = mvp * vec4(worldPos, 1.0);
    vec3 ndc  = clip.xyz / clip.w;
    return vec3(ndc.xy * 0.5 + 0.5, ndc.z);
}

vec3 get_raydir(vec2 uv, float fov, inout vec3 camForward, inout vec3 camRight, inout vec3 camUp) {
    float pitch = dmi.deffrot[0].x;
    float yaw   = dmi.deffrot[0].y;

    camForward = normalize(vec3(
        cos(pitch) * sin(yaw),
       -sin(pitch),
       -cos(pitch) * cos(yaw)
    ));

    vec3 worldUp = vec3(0.0, 1.0, 0.0);
    camRight = normalize(cross(camForward, worldUp));
    camUp = normalize(cross(camRight, camForward));

    vec2  screen = uv * 2.0 - 1.0;
    float aspect = mi.resolutions.x / mi.resolutions.y;
    float tanHalfFov = tan(radians(fov) * 0.5);

    return normalize(camForward + camRight * screen.x * aspect * tanHalfFov + camUp * screen.y * tanHalfFov);
}

vec2 RayDirToUV(vec3 rayDir, vec3 camForward, vec3 camRight, vec3 camUp, float aspect, float fovDeg) {
    float tanHalfFov = tan(radians(fovDeg) * 0.5);
    float d = dot(rayDir, camForward);
    float x = dot(rayDir, camRight) / (d * aspect   * tanHalfFov);
    float y = dot(rayDir, camUp)    / (d             * tanHalfFov);
    return vec2(x, y) * 0.5 + 0.5;
}

// ── Shadowmaps ────────────────────────────────────────────────────────────────

const float SHADOW_DEPTH_BIAS    = 0.0005; // constant bias in shadowmap depth space (tune)
const float SHADOW_NORMAL_TEXELS = 1.5;    // normal offset, in shadowmap texels at the receiver's depth
const float SHADOW_EDGE_FADE     = 0.04;   // spot "cone" softness: fade over this fraction of shadowmap UV
const int   SHADOW_PCF_RADIUS    = 1;      // 1 = 3x3 (9 taps), 0 = single tap
const float LIGHT_CONTRIB_EPS    = 0.01;   // skip a light's shadow lookup + BRDF below this max contribution

// World-space size of one shadowmap texel at clip-space depth clipW.
// For M = P * V (V a pure rotation+translation), |row0(M).xyz| == P[0][0].
float ShadowTexelWorldSize(mat4 M, float clipW) {
    float xScale = length(vec3(M[0][0], M[1][0], M[2][0]));
    return (2.0 / mi.resolutions.z) * clipW / max(xScale, 1e-6);
}

// Shared projection + PCF core, reused by the surface path (with normal-offset
// bias) and the volumetric march (raw position, no surface to offset along).
// pcfRadius is dynamic (not SHADOW_PCF_RADIUS) so callers can trade quality for
// bandwidth: 1 (3x3) for primary hits, 0 (single tap) for bounce and volumetrics.
float ShadowProject(vec3 worldPos, int si, float depthBias, int pcfRadius) {
    mat4 M   = smi.shadowViews[si];
    vec4 smv = M * vec4(worldPos, 1.0);
    if (smv.w <= 0.0) return 0.0;                    // behind the light
    vec3 ndc = smv.xyz / smv.w;
    if (ndc.z < 0.0 || ndc.z > 1.0) return 0.0;      // outside light depth range

    // same sampling coordinates as the old shcalcpl (u = x*0.5+0.5, v = y*0.5+0.5)
    vec2 uv = ndc.xy * 0.5 + 0.5;
    vec2 edgeDist = min(uv, 1.0 - uv);
    float fade = smoothstep(0.0, SHADOW_EDGE_FADE, min(edgeDist.x, edgeDist.y));
    if (fade <= 0.0) return 0.0;                     // outside the spot frustum

    float texel = 1.0 / mi.resolutions.z;
    float lit   = 0.0;
    int   taps  = 0;
    for (int y = -pcfRadius; y <= pcfRadius; y++) {
        for (int x = -pcfRadius; x <= pcfRadius; x++) {
            vec2  tap = uv + vec2(float(x), float(y)) * texel;
            float sd  = texture(sampler2DArray(shadowTexture, attachmentSampler), vec3(tap, float(si))).r;
            lit  += (ndc.z - depthBias < sd) ? 1.0 : 0.0;
            taps += 1;
        }
    }
    return (lit / float(taps)) * fade;
}

// Returns light visibility in [0,1] for shadowmap layer `si`, for a shaded surface point.
//   NdotL       - dot(N, L) of the receiver, used to scale the normal offset
//   extraOffset - additional world-space offset along N (e.g. voxelSize for voxel bounce hits)
float ShadowVisibility(vec3 worldPos, vec3 N, float NdotL, int si, float extraOffset, int pcfRadius) {
    mat4 M = smi.shadowViews[si];

    // clip.w at the un-offset position gives the texel size in world units
    float w0 = dot(vec4(M[0][3], M[1][3], M[2][3], M[3][3]), vec4(worldPos, 1.0));
    if (w0 <= 0.0) return 0.0;                       // behind the light

    // normal-offset bias: grows at grazing angles, never fully zero
    float sinTheta = sqrt(max(1.0 - NdotL * NdotL, 0.0));
    float offset   = ShadowTexelWorldSize(M, w0) * SHADOW_NORMAL_TEXELS * mix(0.5, 1.0, sinTheta)
                   + extraOffset;
    vec3  p        = worldPos + N * offset;

    return ShadowProject(p, si, SHADOW_DEPTH_BIAS, pcfRadius);
}

// ── PBR ───────────────────────────────────────────────────────────────────────

vec3 FresnelSchlick(float cosTheta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float DistributionGGX(vec3 N, vec3 H, float roughness) {
    float a     = roughness * roughness;
    float a2    = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float denom = NdotH * NdotH * (a2 - 1.0) + 1.0;
    return a2 / (PI * denom * denom + 1e-6);
}

float GeometrySchlickGGX(float NdotV, float roughness) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;
    return NdotV / (NdotV * (1.0 - k) + k);
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
    return GeometrySchlickGGX(max(dot(N, V), 0.0), roughness)
         * GeometrySchlickGGX(max(dot(N, L), 0.0), roughness);
}

vec3 CosineHemisphere(vec2 xi) {
    float phi      = 2.0 * PI * xi.x;
    float cosTheta = sqrt(1.0 - xi.y);
    float sinTheta = sqrt(xi.y);
    return vec3(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);
}

mat3 CreateTBN(vec3 N) {
    vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
    vec3 T  = normalize(cross(up, N));
    vec3 B  = cross(N, T);
    return mat3(T, B, N);
}

struct BRDFSample {
    vec3  direction;
    vec3  weight;
    float pdf;
};

BRDFSample SampleGGX(vec3 V, vec3 N, vec3 F0, float roughness, vec2 Xi) {
    BRDFSample result;
    mat3  TBN    = CreateTBN(N);
    vec3  Vlocal = transpose(TBN) * V;
    float alpha  = max(0.001, roughness * roughness);

    vec3 Vh   = normalize(vec3(alpha * Vlocal.x, alpha * Vlocal.y, Vlocal.z));
    float lensq = Vh.x * Vh.x + Vh.y * Vh.y;
    vec3 T1   = lensq > 0.0 ? vec3(-Vh.y, Vh.x, 0.0) * inversesqrt(lensq) : vec3(1.0, 0.0, 0.0);
    vec3 T2   = cross(Vh, T1);

    float r   = sqrt(Xi.x);
    float phi = 2.0 * PI * Xi.y;
    float t1  = r * cos(phi);
    float t2  = r * sin(phi);
    float s   = 0.5 * (1.0 + Vh.z);
    t2 = mix(sqrt(max(0.0, 1.0 - t1*t1)), t2, s);

    vec3 Nh = t1*T1 + t2*T2 + sqrt(max(0.0, 1.0 - t1*t1 - t2*t2)) * Vh;
    vec3 Hlocal = normalize(vec3(alpha * Nh.x, alpha * Nh.y, max(0.0, Nh.z)));
    vec3 H = normalize(TBN * Hlocal);

    result.direction = reflect(-V, H);

    float NdotV = max(dot(N, V),  0.0001);
    float NdotL = max(dot(N, result.direction), 0.0001);
    float NdotH = max(dot(N, H), 0.0001);
    float VdotH = max(dot(V, H), 0.0001);

    vec3  F = FresnelSchlick(VdotH, F0);
    float D = DistributionGGX(N, H, roughness);
    float G = GeometrySmith(N, V, result.direction, roughness);

    result.pdf    = max(D * NdotH / (4.0 * VdotH), 1e-5);
    result.weight = F * G * VdotH / max(NdotV * NdotH, 1e-5);
    return result;
}

BRDFSample SampleDiffuse(vec3 albedo, vec3 N, vec2 Xi) {
    BRDFSample result;
    mat3 TBN       = CreateTBN(N);
    vec3 local     = CosineHemisphere(Xi);
    result.direction = normalize(TBN * local);
    float NdotL    = max(dot(N, result.direction), 0.0);
    result.pdf     = NdotL / PI;
    result.weight  = albedo;
    return result;
}

BRDFSample finalSample(vec3 V, vec3 N, vec3 albedo,
                        float roughness, float metallic, vec3 rnd) {
    vec3 F0   = mix(vec3(0.04), albedo, metallic);
    vec3 F    = FresnelSchlick(max(dot(N, V), 0.0), F0);
    float specP = clamp(dot(F, vec3(0.2126, 0.7152, 0.0722)), 0.05, 0.95);

    BRDFSample brdf;
    if (rnd.x < specP) {
        brdf = SampleGGX(V, N, F0, roughness, rnd.yz);
        brdf.weight /= specP;
    } else {
        brdf = SampleDiffuse(albedo * (1.0 - metallic), N, rnd.yz);
        brdf.weight /= (1.0 - specP);
    }
    return brdf;
}

vec4 voxelRaycast(inout vec3 rayOrigin, vec3 rayDir, out vec3 outn) {
    float voxelSize = mi.rtinfo.w;
    vec3  gridMin   = mi.rtinfo.xyz;
    vec3  gridSize  = mi.addinfo.yzw;
    vec3  gridMax   = gridMin + gridSize * voxelSize;

    outn = vec3(0.0, 1.0, 0.0);

    vec3  invDir = 1.0 / (rayDir + vec3(EPS));
    vec3  t0     = (gridMin - rayOrigin) * invDir;
    vec3  t1     = (gridMax - rayOrigin) * invDir;
    vec3  tMinV  = min(t0, t1);
    vec3  tMaxV2 = max(t0, t1);

    float tEnter = max(max(tMinV.x, tMinV.y), tMinV.z);
    float tExit  = min(min(tMaxV2.x, tMaxV2.y), tMaxV2.z);
    if (tEnter > tExit || tExit < 0.0) return vec4(0.0); // miss

    float t        = max(tEnter, 0.0);
    vec3  entryPos = rayOrigin + rayDir * (t + EPS);
    vec3  localPos = (entryPos - gridMin) / voxelSize;

    ivec3 voxel  = clamp(ivec3(floor(localPos)), ivec3(0), ivec3(gridSize) - 1);
    ivec3 vstep  = ivec3(sign(rayDir));
    vec3  tDelta = abs(voxelSize / rayDir);
    vec3  frac   = localPos - floor(localPos);

    vec3 tMaxV;
    tMaxV.x = (rayDir.x > 0.0 ? 1.0 - frac.x : frac.x) * tDelta.x;
    tMaxV.y = (rayDir.y > 0.0 ? 1.0 - frac.y : frac.y) * tDelta.y;
    tMaxV.z = (rayDir.z > 0.0 ? 1.0 - frac.z : frac.z) * tDelta.z;

    int  lastAxis = 0;
    bool sawEmpty = false;
    float tps     = 0.0;

    for (int i = 0; i < MAX_STEPS; i++) {
        if (any(lessThan(voxel, ivec3(0))) ||
            any(greaterThanEqual(voxel, ivec3(gridSize)))) {
            return vec4(0.0);
        }
        vec3 uvw = (vec3(voxel) + 0.5) / gridSize;
        vec4 cl  = texture(sampler3D(texTexture, attachmentSampler), uvw);
        if (cl.a > 0.5) {
            if (sawEmpty) {
                vec3 n;
                if (lastAxis == 0) n = vec3(-float(vstep.x), 0.0, 0.0);
                else if (lastAxis == 1) n = vec3(0.0, -float(vstep.y), 0.0);
                else n = vec3(0.0, 0.0, -float(vstep.z));
                outn      = n;
                rayOrigin = rayOrigin + rayDir * tps;
                return vec4(cl.rgb, 1.0);
            }
        } else {
            sawEmpty = true;
        }
        if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
            tps      = tMaxV.x; tMaxV.x += tDelta.x;
            voxel.x += vstep.x; lastAxis  = 0;
        } else if (tMaxV.y < tMaxV.z) {
            tps      = tMaxV.y; tMaxV.y += tDelta.y;
            voxel.y += vstep.y; lastAxis  = 1;
        } else {
            tps      = tMaxV.z; tMaxV.z += tDelta.z;
            voxel.z += vstep.z; lastAxis  = 2;
        }
    }

    return vec4(0.0);
}

// shadowOffset: extra world-space normal offset for the shadowmap lookup
//               (0 for G-buffer surfaces, ~voxelSize for voxel raycast hits)
// pcfRadius:    shadow filter radius (1 = 3x3, 0 = single tap). Use 0 for the
//               low-frequency bounce term; the denoiser hides the extra noise
//               there for a fraction of the shadow-fetch cost.
vec3 EvaluateDirectLighting(vec3 pos, vec3 N, vec3 V, vec3 albedo, float roughness, float metallic,
                            float shadowOffset, int pcfRadius) {
    int   lightCount = clamp(int(mi.lightinfo.w), 0, 100);
    vec3  F0         = mix(vec3(0.04), albedo, metallic);
    vec3  total      = vec3(0.0);
    int   shadowIdx  = 0;   // next free shadowmap layer

    for (int li = 0; li < lightCount; li++) {
        // Claim the shadowmap layer BEFORE any early-out, otherwise skipped
        // lights would shift the layer index of every later shadowed light.
        int si = -1;
        if (smi.lightcol[li].w > 0.5) {
            si = shadowIdx;
            shadowIdx++;
        }

        vec3  lpos      = smi.lightpos[li].xyz;
        vec3  lcol      = smi.lightcol[li].xyz;
        float lintens   = 2.0;
        vec3  toLight   = lpos - pos;
        float lightDist2 = dot(toLight, toLight);
        float atten      = lintens / (1.0 + lightDist2 * 0.05);

        // Cheap reject BEFORE the shadow lookup: a light whose maximum possible
        // contribution (ignoring NdotL/visibility) is negligible isn't worth a
        // texture-array fetch. This is the single most common skip for lights
        // far from `pos`.
        float maxContrib = atten * max(lcol.r, max(lcol.g, lcol.b));
        if (maxContrib < LIGHT_CONTRIB_EPS) continue;

        float lightDist = sqrt(lightDist2);
        vec3  L         = toLight / lightDist;
        float NdotL     = max(dot(N, L), 0.0);
        if (NdotL <= 0.0) continue;

        float visibility = 1.0;
        if (si >= 0) {
            visibility = ShadowVisibility(pos, N, NdotL, si, shadowOffset, pcfRadius);
            if (visibility <= 0.0) continue;
        }

        vec3  H     = normalize(V + L);
        float NdotV = max(dot(N, V),    0.0001);
        float NdotH = max(dot(N, H),    0.0001);
        float VdotH = max(dot(V, H),    0.0001);
        vec3  F = FresnelSchlick(VdotH, F0);
        float D = DistributionGGX(N, H, roughness);
        float G = GeometrySmith(N, V, L, roughness);
        vec3  specular = (D * G * F) / max(4.0 * NdotV * NdotL, 1e-5);
        vec3  kD = (1.0 - F) * (1.0 - metallic);
        vec3  diffuse = kD * albedo / PI;
        total += (diffuse + specular) * lcol * atten * NdotL * visibility;
    }

    return total;
}

// ── Volumetric light scatter ────────────────────────────────────────────────
// In-scattered light along the view ray, shaped by each light's shadowmap.
// Only shadowed lights (lightcol.w > 0.5) participate — occlusion is what
// turns this into beams (window slits, doorways) rather than a uniform haze.
// All constants are scene-scale dependent; tune against your world units.
const int   VOL_STEPS      = 12;     // view-ray march steps
const float VOL_DENSITY    = 0.05;   // extinction coefficient (higher = hazier, shorter throw)
const float VOL_ANISOTROPY = 0.6;    // Henyey-Greenstein g; >0 favors forward scattering toward the eye
const float VOL_INTENSITY  = 0.5;    // overall strength multiplier on the accumulated scatter
const float VOL_SHADOW_BIAS = 0.0015;// depth bias for unoffset march samples (not a surface, so small is fine)
const float VOL_SKY_DIST   = 40.0;   // march distance used when the primary ray hits no surface

float HGPhase(float cosTheta, float g) {
    float g2    = g * g;
    float denom = max(1.0 + g2 + 2.0 * g * cosTheta, 1e-4);
    return (1.0 - g2) / (4.0 * PI * denom * sqrt(denom));
}

// origin/rayDir - camera position and primary ray direction (world space)
// maxDist       - distance to march: to the surface hit, or VOL_SKY_DIST for sky
// dither        - jitter in 0..1 for the first sample, hides step banding
vec3 VolumetricLightScatter(vec3 origin, vec3 rayDir, float maxDist, float dither) {
    int   lightCount     = clamp(int(mi.lightinfo.w), 0, 100);
    float stepSize       = maxDist / float(VOL_STEPS);
    float t              = stepSize * dither;
    float transmittance  = 1.0;
    vec3  scatter        = vec3(0.0);

    for (int s = 0; s < VOL_STEPS; s++) {
        vec3 p = origin + rayDir * t;
        int  shadowIdx = 0;

        for (int li = 0; li < lightCount; li++) {
            int si = -1;
            if (smi.lightcol[li].w > 0.5) {
                si = shadowIdx;
                shadowIdx++;
            }
            if (si < 0) continue;   // unshadowed lights can't be occluded into a beam shape

            vec3  toLight = smi.lightpos[li].xyz - p;
            float dist2   = dot(toLight, toLight);
            vec3  L       = toLight * inversesqrt(max(dist2, 1e-6));

            float vis = ShadowProject(p, si, VOL_SHADOW_BIAS, 0);
            if (vis <= 0.0) continue;

            float atten = 2.0 / (1.0 + dist2 * 0.05);   // same falloff as EvaluateDirectLighting
            float phase = HGPhase(dot(L, -rayDir), VOL_ANISOTROPY);
            scatter += smi.lightcol[li].xyz * atten * vis * phase * transmittance;
        }

        transmittance *= exp(-VOL_DENSITY * stepSize);
        t += stepSize;
    }

    return scatter * VOL_INTENSITY * stepSize;
}

vec3 voxelplusssrt() {
    vec2 uv = vec2(fuv.x, 1.0 - fuv.y);

    vec3  camForward, camRight, camUp;
    float fovDeg = dmi.deffrot[0].w;
    float aspect = mi.resolutions.x / mi.resolutions.y;
    vec3  primaryRay = get_raydir(uv, fovDeg, camForward, camRight, camUp);
    vec3  camOrigin  = dmi.deffpos[0].xyz;   // ASSUMPTION: deffpos[0].xyz is the camera's world position
    vec3  V = -primaryRay;

    // Dedicated dither sample for the volumetric march, decorrelated from the
    // BRDF's blue-noise sample below so their noise patterns don't stack.
    vec2 volBnUV = mod(uv * 2.0 + (mi.addinfo.x * GOLDEN_RATIO + 0.37), 1.0);
    float volDither = texture(sampler2D(noiseTexture, attachmentSampler), volBnUV).w;

    float d = SampleDepth(uv);
    bool  hitSurface = d < 0.9999;
    vec3  wrldpos = hitSurface ? WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]) : vec3(0.0);
    float volDist = hitSurface ? distance(camOrigin, wrldpos) : VOL_SKY_DIST;
    vec3  volume  = VolumetricLightScatter(camOrigin, primaryRay, volDist, volDither);

    if (!hitSurface) return SKY_COLOR + volume;

    vec3 albedo = SampleAlbedo(uv);
    vec3 rma    = SampleRMA(uv);
    vec3 normal = SampleNormal(uv);
    float roughness = rma.r;
    float metallic  = rma.g;

    int spp = 1;

    vec3 result = vec3(0.0);

    for (int j = 0; j < spp; j++) {
        vec2 bnUV = mod(uv * 2.0 + float(mi.addinfo.x + j) * GOLDEN_RATIO, 1.0);
        vec4 bn   = texture(sampler2D(noiseTexture, attachmentSampler), bnUV);
        vec3 throughput = albedo;
        BRDFSample brdf = finalSample(V, normal, albedo, roughness, metallic, bn.xyz);
        vec3 rayDir     = brdf.direction;
        throughput     *= brdf.weight;
        vec3 direct = EvaluateDirectLighting(
            wrldpos + normal * 0.05, normal, V,
            albedo, roughness, metallic, 0.0, SHADOW_PCF_RADIUS
        );
        result += direct;
        vec3  bouncePos    = wrldpos;
        vec3  bounceNormal = normal;
        vec3  bounceAlbedo = vec3(0.0);
        float bounceRough  = 0.85;
        float bounceMetal  = 0.0;
        bool  bounceHit    = false;
        float bounceShadowOff = 0.0;   // >0 only for voxel hits (blocky surface)
        vec2 nextUV = RayDirToUV(rayDir, camForward, camRight, camUp, aspect, fovDeg);
        bool onScreen = nextUV.x >= 0.0 && nextUV.x <= 1.0 && nextUV.y >= 0.0 && nextUV.y <= 1.0;
        bool awayFromCam = dot(rayDir, normal) > 0.0;
        if (onScreen && awayFromCam) {
            vec3 marchPos = wrldpos + normal * 0.05;
            bool ssHit    = false;
            for (int s = 0; s < 64; s++) {
                marchPos    += rayDir * 0.1;
                vec3 uvd     = WorldPosToUVDepth(marchPos, dmi.defferedMVP[0]);
                if (uvd.x < 0.0 || uvd.x > 1.0 || uvd.y < 0.0 || uvd.y > 1.0) break;
                float sd    = SampleDepth(uvd.xy);
                float delta = uvd.z - sd;
                if (delta > 0.0 && delta < SS_THICKNESS) {
                    bouncePos    = WorldPosFromDepth(sd, uvd.xy, dmi.defferedMVPInverse[0]);
                    bounceNormal = SampleNormal(uvd.xy);
                    bounceAlbedo = SampleAlbedo(uvd.xy);
                    vec3 brma    = SampleRMA(uvd.xy);
                    bounceRough  = brma.r;
                    bounceMetal  = brma.g;
                    bounceHit    = true;
                    ssHit        = true;
                    break;
                }
            }
            if (!ssHit) {
                vec3 voxNormal;
                vec3 voxOrigin = wrldpos + normal * mi.rtinfo.w * 1.5;
                vec4 voxHit    = voxelRaycast(voxOrigin, rayDir, voxNormal);
                if (voxHit.a > 0.5) {
                    bouncePos    = voxOrigin;
                    bounceNormal = voxNormal;
                    bounceAlbedo = voxHit.rgb;
                    bounceShadowOff = mi.rtinfo.w;
                    bounceHit    = true;
                }
            }
        } else {
            vec3 voxNormal;
            vec3 voxOrigin = wrldpos + normal * mi.rtinfo.w * 1.5;
            vec4 voxHit    = voxelRaycast(voxOrigin, rayDir, voxNormal);
            if (voxHit.a > 0.5) {
                bouncePos    = voxOrigin;
                bounceNormal = voxNormal;
                bounceAlbedo = voxHit.rgb;
                bounceShadowOff = mi.rtinfo.w;
                bounceHit    = true;
            }
        }
        if (!bounceHit) {
            result += throughput * SKY_COLOR;
            continue;
        }
        vec3 bounceV      = -rayDir;
        vec3 bounceDirect = EvaluateDirectLighting(
            bouncePos + bounceNormal * 0.05, bounceNormal, bounceV,
            bounceAlbedo, bounceRough, bounceMetal, bounceShadowOff, 0
        );
        result += throughput * bounceDirect;
    }
    result /= float(spp);
    result += volume;
    result = min(result, vec3(FIREFLY_CAP));
    return result;
}

void main() {
    //vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
    //outColor = vec4(0.0, 0.0, 0.0, 1.0);
    //float frn = mi.resolutions.a;
    //int fri = int(gl_FragCoord.y+gl_FragCoord.x);
    //if((frn == 0.0 && fri % 2 == 0) || (frn == 1.0 && fri % 2 == 1)){
    //    outColor = vec4(voxelplusssrt(), 1.0);
    //}

    outColor = vec4(voxelplusssrt(), 1.0);
    //outColor = vec4(SampleAlbedo(uv), 1.0);
}