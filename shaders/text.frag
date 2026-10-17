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

vec3 SampleRT(vec2 uv) {
    return texture(sampler2DArray(lightingTexture, attachmentSampler), vec3(uv, mi.resolutions.a)).rgb;
}

vec4 SampleMaterial(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1));
}

void main() {
    if(mi.addinfo.z > 0.0 && mi.addinfo.z < 10.0){
        outColor = vec4(pow(1.0 - texture(sampler2DArray(texTexture, imageSampler), vec3(uv.x/mi.addinfo.x + (1.0/mi.addinfo.x)*mi.addinfo.y, -uv.y, 0)).rgb, vec3(2.2)), 1.0);
    }else if(mi.addinfo.z > 10.0){
        outColor = vec4(pow(texture(sampler2DArray(texTexture, imageSampler), vec3(uv.x/mi.addinfo.x + (1.0/mi.addinfo.x)*mi.addinfo.y, -uv.y, 0)).rgb, vec3(2.2)) / 16.0, 1.0);
    }else{
        outColor = vec4(pow(texture(sampler2DArray(texTexture, imageSampler), vec3(uv.x/mi.addinfo.x + (1.0/mi.addinfo.x)*mi.addinfo.y, -uv.y, 0)).rgb, vec3(2.2)), 1.0);
    }
}