#version 300 es
layout(location = 0) in vec2 aPosition;
layout(location = 1) in vec2 aTexCoord;

uniform struct Transform {
    vec2 position;
    vec2 size;
    float rotation;
} transform;

uniform struct Camera {
    vec2 position;
    float zoom;
} camera;

out vec2 vTexCoord;

void main() {
    vec2 finalPosition = aPosition;

    vec2 normalizedSize = transform.size/1920.0;                                                // Size by pixel
    finalPosition *= normalizedSize;                                                            // Apply Transform size

    finalPosition = vec2(
        finalPosition.x * cos(transform.rotation) - finalPosition.y * sin(transform.rotation),  // Rotation
        finalPosition.y * cos(transform.rotation) + finalPosition.x * sin(transform.rotation)
    );

    finalPosition = vec2(finalPosition.x, finalPosition.y*16.0/9.0);                            // Screen scaling

    finalPosition = (finalPosition + transform.position - camera.position) * camera.zoom;

    vTexCoord = aTexCoord;

    gl_Position = vec4(finalPosition, 0.0, 1.0);
}