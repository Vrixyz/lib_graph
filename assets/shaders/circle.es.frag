#version 300 es
precision lowp float;

out vec4 o_Target;
in vec2 v_Uv;

layout(std140)uniform ColorMaterial_color{// set = 2, binding = 0
    vec4 color;
};

float draw_circle(vec2 coord,float radius){
    float pct=length(coord-vec2(.5));
    pct=1.-pct;
    pct=smoothstep(.5,.6,pct);
    return pct;
}
float draw_circle_hard(vec2 coord,float radius){
    return step(length(coord-vec2(.5)),radius);
}

void main()
{
    float l=draw_circle_hard(v_Uv,.5);
    if(l<.1){
        discard;
    }
    vec3 c=vec3(l);
    o_Target=vec4(c,1.)*color;
}