#version 330 core

layout (location = 0) in vec2 Position;

uniform vec2 u_resolution;

void main()
{
    vec2 uv = Position;

    uv.x *= u_resolution.y / u_resolution.x;
    uv.x *= u_resolution.x / u_resolution.y;


    gl_Position = vec4(uv, 0.0, 1.0);
}
