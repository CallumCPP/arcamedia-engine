#version 300 es
layout(location = 0) in vec2 aPosition;

uniform struct Transform {
    vec2 position;
    vec2 scale;
    float rotation;
} transform;

uniform struct Camera {
    vec2 position;
    float zoom;
} camera;

void main() {
    vec2 rotatedPosition = vec2(
        aPosition[0] * cos(transform.rotation) - aPosition[1] * sin(transform.rotation),
        aPosition[1] * cos(transform.rotation) + aPosition[0] * sin(transform.rotation)
    );

    vec2 screenScaledPosition = vec2(rotatedPosition[0], rotatedPosition[1]*16.0/9.0);

    gl_Position = vec4((screenScaledPosition * transform.scale + transform.position) * camera.zoom - camera.position, 0.0, 1.0);
}