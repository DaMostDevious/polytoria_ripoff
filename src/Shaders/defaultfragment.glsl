in vec3 pos;
in vec3 nor;

uniform vec3 lightDirection;
uniform vec3 lightPos;
uniform vec3 baseColor;

out vec4 outColor;

void main() {
    vec3 normal = normalize(nor);

    float diff = max(dot(normal, lightDirection), 0.0);
    vec3 diffuse = diff * vec3(1.0,1.0,1.0);

    float ambient = 0.6;
    vec3 result = (ambient + diffuse) * baseColor;

    outColor = vec4(mix(baseColor,result, length(pos - lightPos)), 1.0);
}
