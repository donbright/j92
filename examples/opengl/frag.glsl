#version 130
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

varying vec3 ls_normal;
void main() {
    gl_FragColor = vec4((ls_normal + 1.0) / 2.0, 0.95);
}
