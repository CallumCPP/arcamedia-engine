#version 300 es

precision mediump float;

uniform vec4 fragColor;
uniform sampler2D image;

in vec2 vTexCoord;
out vec4 outColor;

void main() {
    outColor = texture(image, vTexCoord) * fragColor;
}