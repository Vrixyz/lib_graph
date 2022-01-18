#version 450
layout(location=0)out vec4 o_Target;
layout(location=2)in vec2 v_Uv;
layout(set=2,binding=0)uniform ColorMaterial_color{
    vec4 color;
};

float draw_circle(vec2 coord,float radius){
    float pct=length(coord-vec2(.5));
    pct=1.-pct;
    pct=smoothstep(.5,.6,pct);
    return pct;
}
float draw_circle_hard(vec2 coord,float radius){
    return step(length(coord),radius);
}
void main(){
    float circle=draw_circle_hard(v_Uv-vec2(.5),.5);
    
    if(circle<.01){
        discard;
    }
    vec3 c=vec3(circle);
    o_Target=vec4(c,1.)*color;
}