#version 330 core

uniform float u_time;
uniform vec2 u_resolution;
vec2 fragCoord = gl_FragCoord.xy;
out vec4 theColor;

vec3 palette(float d){
	return mix(vec3(0.3451, 0.051, 0.8941),vec3(0.8902, 0.9529, 0.0157),d);
}

vec2 rotate(vec2 p,float a){
	float c = cos(a);
    float s = sin(a);
    return p*mat2(c,s,-s,c);
}

float map(vec3 p){
    for( int i = 0; i<8; ++i){
        float t = u_time*0.1;
        p.xz =rotate(p.xz,t);
        p.xy =rotate(p.xy,t*1.89);
        p.yz = rotate(p.yz, t*1.89);
        p.xz = abs(p.xz);
        p.xz-=.5;
	}
	return dot(sign(p),p)/4.;
}

vec4 rm (vec3 ro, vec3 rd){
    float t = 0.;
    vec3 col = vec3(0.);
    float d;
    for(float i =0.; i<64.; i++){
		vec3 p = ro + rd*t;
        d = map(p)*.5;
        if(d<0.02){
            break;
        }
        if(d>100.){
        	break;
        }
        //col+=vec3(0.6,0.8,0.8)/(400.*(d));
        col+=palette(length(p)*.1)/(400.*(d));
        t+=d;
    }
    return vec4(col,1./(d*100.));
}
void main()
{
    vec2 uv = (fragCoord-(u_resolution.xy/2.))/u_resolution.x;
	vec3 ro = vec3(0.,0.,-80.);
    ro.xz = rotate(ro.xz,u_time*2);
    vec3 cf = normalize(-ro);
    vec3 cs = normalize(cross(cf,vec3(0.0196, 0.6392, 1.0)));
    vec3 cu = normalize(cross(cf,cs));
    
    vec3 uuv = ro+cf*5. + uv.x*cs + uv.y*cu;
    
    vec3 rd = normalize(uuv-ro);
    
    vec4 col = rm(ro,rd);
    
    
    theColor = col;
}
