in vec3 position;
in vec3 normal;

uniform mat4 modelMatrix;
uniform mat4 viewProjection;

out vec3 nor;
out vec3 pos;

void main() {
    vec4 world_pos = modelMatrix * vec4(position, 1.0);
    pos = world_pos.xyz;

    mat3 normalMatrix = transpose(inverse(mat3(modelMatrix)));
    nor = normalize(normalMatrix * normal);

    gl_Position = viewProjection * world_pos;
}
