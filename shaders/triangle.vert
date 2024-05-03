// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#version 450

vec2 verticies[3] = vec2[](
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
);

void main() {
    vec2 vtx = verticies[gl_VertexIndex];

    gl_Position = vec4(vtx, 0.0, 1.0);
}