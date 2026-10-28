#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 uv;

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

layout(binding = 3) uniform texture2DArray texTexture;
layout(binding = 4) uniform texture2DArray lightingTexture;
layout(binding = 5) uniform texture2DArray lightingDepthTexture;
layout(binding = 6) uniform texture2DArray defferedTexture;
layout(binding = 7) uniform texture2DArray defferedDepthTexture;
layout(binding = 8) uniform texture2DArray shadowTexture;
layout(binding = 9) uniform sampler imageSampler;
layout(binding = 10) uniform sampler attachmentSampler;

float field(vec2 p, float t, float k) {
    p += k * vec2(sin(p.y * 1.3 + t * 0.40),
                  cos(p.x * 1.1 - t * 0.30));
    p += k * 0.5 * vec2(sin(p.y * 2.1 - t * 0.60),
                        cos(p.x * 2.3 + t * 0.50));
    return sin(p.x * 1.5 + t * 0.20)
         + sin(p.y * 1.7 - t * 0.25)
         + sin((p.x + p.y) * 1.1 + t * 0.30);
}

// Anti-aliased hard edge (flat comic look).
float edge(float v, float th) {
    float w = max(fwidth(v), 1e-4);
    return smoothstep(th - w, th + w, v);
}

void main() {
    vec3 base = pow(texture(sampler2DArray(texTexture, imageSampler),
                            vec3(uv.x, 1.0 - uv.y, 0)).rgb, vec3(2.2)) / 16.0;

    float t    = mi.rtinfo.x;
    float cps  = clamp(mi.rtinfo.y / 2000.0, 0.0, 1.0);
    float fps  = clamp(mi.rtinfo.z / 2000.0, 0.0, 1.0);

    float aspect = mi.resolutions.x / mi.resolutions.y;
    vec2 p = (uv - 0.5) * vec2(aspect, 1.0) * 3.0;

    p *= mix(0.8, 1.6, cps);
    float k = mix(0.4, 1.2, fps);

    float v = field(p, t, k);

    float thOuter = 0.6;
    float thInner = 1.7;
    float m1 = edge(v, thOuter);
    float m2 = edge(v, thInner);

    vec3 col = base;
    col = mix(col, base * 0.70, m1);
    col = mix(col, base * 0.45, m2);

    float w = max(fwidth(v), 1e-4);
    float ink = 1.0 - smoothstep(0.0, w * 2.0, abs(v - thOuter));
    col = mix(col, base * 0.15, ink * 0.8);

    outColor = vec4(col, 1.0);
}