#version 300 es

precision mediump float;
uniform vec4 fragColor;
out vec4 outColor;

void main() {
    outColor = fragColor;
}