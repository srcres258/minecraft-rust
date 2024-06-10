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

out vec4 outColour;
in  vec3 passTextureCoord;

uniform samplerCube texSampler;

vec3 brightnessContrast(vec3 value, float brightness, float contrast)
{
    return (value - 0.5) * contrast + 0.5 + (brightness-1);
}

vec4 color;

vec3 gamma(vec3 value, float param)
{
    return vec3(pow(abs(value.r), param),pow(abs(value.g), param),pow(abs(value.b), param));
}


void main()
{
    color = texture(texSampler, passTextureCoord);
    color = vec4(brightnessContrast(color.xyz, 1.15f, 1.15f), color.w);
    color = vec4(gamma(color.xyz, 4.8f),color.w);

    outColour = color;
    if (outColour.a == 0) discard;
}
