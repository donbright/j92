#version 130
attribute vec3 position;
attribute vec3 normal;
uniform mat4 view;
uniform mat4 proj;
uniform mat4 transform;
uniform mat3 scale;
varying vec3 ls_normal;

void main() {
    ls_normal   = normal;
    gl_Position = proj * view * transform * mat4(scale) * vec4(position, 1.0);
}
