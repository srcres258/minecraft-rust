// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 src_resources
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#version 330

layout(location = 0) in vec3  inVertexPosition;
layout(location = 1) in vec2  inTextureCoord;
layout(location = 2) in float inCardinalLight;

out vec2 passTextureCoord;
out float passCardinalLight;

uniform mat4 projViewMatrix;
uniform float globalTime;


vec4 getWorldPos()
{
    vec3 inVert = inVertexPosition.xyz;
    inVert.x += sin((globalTime + inVert.z + inVert.y) * 1.8f) / 15.0f;
    inVert.z -= cos((globalTime + inVert.x + inVert.y) * 1.8f) / 15.0f;
    return vec4(inVert, 1);
}

void main()
{
    gl_Position = projViewMatrix * getWorldPos();

    passTextureCoord    = inTextureCoord;
    passCardinalLight   = inCardinalLight;
}
