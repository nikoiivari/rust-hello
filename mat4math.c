#include <math.h>
#include <strings.h>
#include "mat4math.h"


void pmOrtho(float pm[16], float left,   float right, 
                           float bottom, float top,
                           float near,   float far)
{
    bzero(pm, sizeof(float)*16);
    pm[0]=2/(right-left);
    pm[5]=2/(top-bottom);
    pm[10]=(-2)/(far-near);
    pm[15]=1.;
}

void pmFrustum(float pm[16], float left,   float right, 
                             float bottom, float top,
                             float near,   float far)
{
    bzero(pm, sizeof(float)*16);
    pm[0]=(2*near)/(right-left);
    pm[5]=(2*near)/(top-bottom);
    pm[8]=(right+left)/(right-left);
    pm[9]=(top+bottom)/(top-bottom);
    pm[10]=-(far+near)/(far-near);
    pm[11]=(-1.);
    pm[14]=-(2*far*near)/(far-near);
    pm[15]=1.;
}

void pmPerspective(float pm[16],
                   float fovVertical, float aspectRatio, 
                   float zNear, float zFar)
{
    float halfW = tanf(0.5 * (fovVertical/180 * M_PI));
    float halfH = halfW / aspectRatio;
    pmFrustum(pm, -halfW, halfW, -halfH, halfH, zNear, zFar);
}

void tmZ(float tm[16], float translationZ)
{
    bzero(tm, sizeof(float)*16);
    tm[0]=1.;
    tm[5]=1.;
    tm[10]=1.;
    tm[14]=translationZ;
    tm[15]=1.;
}

void rmX(float rm[16], float rotationX)
{
    float rx = rotationX/180. * M_PI;
    rm[0]=1.; rm[1]=0.;      rm[2]=0.;       rm[3]=0.;
    rm[4]=0.; rm[5]=cosf(rx);rm[6]=-sinf(rx);rm[7]=0.;
    rm[8]=0.; rm[9]=sinf(rx);rm[10]=cosf(rx);rm[11]=0.;
    rm[12]=0.;rm[13]=0.;     rm[14]=0.;      rm[15]=1.;
}

void rmY(float rm[16], float rotationY)
{
    float ry = rotationY/180. * M_PI;
    rm[0]=cosf(ry); rm[1]=0.;      rm[2]=sinf(ry);  rm[3]=0.;
    rm[4]=0.;       rm[5]=1.;      rm[6]=0.;        rm[7]=0.;
    rm[8]=-sinf(ry);rm[9]=0.;      rm[10]=cosf(ry); rm[11]=0.;
    rm[12]=0.;      rm[13]=0.;     rm[14]=0.;       rm[15]=1.;
}

void rmZ(float rm[16], float rotationZ)
{
    float rz = rotationZ/180. * M_PI;
    rm[0]=cosf(rz); rm[1]=-sinf(rz);    rm[2]=0.;       rm[3]=0.;
    rm[4]=sinf(rz); rm[5]=cosf(rz);     rm[6]=0.;       rm[7]=0.;
    rm[8]=0.;       rm[9]=0.;           rm[10]=1.;      rm[11]=0.;
    rm[12]=0.;      rm[13]=0.;          rm[14]=0.;      rm[15]=1.;
}

void rmXY(float rm[16], float rotationX, float rotationY)
{
    bzero(rm, sizeof(float)*16);
    float my[16];
    bzero(my, sizeof(float)*16);
    float mx[16];
    bzero(mx, sizeof(float)*16);
    
    rmX(mx, rotationX);
    rmY(my, rotationY);

    MN(mx,my,rm);
}

void rmXZ(float rm[16], float rotationX, float rotationZ)
{
    bzero(rm, sizeof(float)*16);
    float mz[16];
    bzero(mz, sizeof(float)*16);
    float mx[16];
    bzero(mx, sizeof(float)*16);
    
    rmX(mx, rotationX);
    rmZ(mz, rotationZ);

    MN(mx,mz,rm);
}


#if 0
void rmQold(float rm[16], float Q[4])
{
    bzero(rm, sizeof(float)*16);
    float M[16];
    //bzero(M, sizeof(float)*16);
    float N[16];
    //bzero(N, sizeof(float)*16);
    /* Q[3] == w */
    M[0] =  Q[3]; M[4] = -Q[2];  M[8] =  Q[1]; M[12] =  Q[0];
    M[1] =  Q[2]; M[5] =  Q[3];  M[9] = -Q[0]; M[13] =  Q[1];
    M[2] = -Q[1]; M[6] =  Q[0]; M[10] =  Q[3]; M[14] =  Q[2];
    M[3] = -Q[0]; M[7] = -Q[1]; M[11] = -Q[2]; M[15] =  Q[3];

    N[0] =  Q[3]; N[4] = -Q[2];  N[8] =  Q[1]; N[12] = -Q[0];
    N[1] =  Q[2]; N[5] =  Q[3];  N[9] = -Q[0]; N[13] = -Q[1];
    N[2] = -Q[1]; N[6] =  Q[0]; N[10] =  Q[3]; N[14] = -Q[2];
    N[3] =  Q[0]; N[7] =  Q[1]; N[11] =  Q[2]; N[15] =  Q[3];

    MN(M,N,rm);
}
#endif



void tmV(float tm[16], float V[3])
{
    tm[0] = 1.0; tm[4] = 0.0;  tm[8] = 0.0; tm[12] = V[0];
    tm[1] = 0.0; tm[5] = 1.0;  tm[9] = 0.0; tm[13] = V[1];
    tm[2] = 0.0; tm[6] = 0.0; tm[10] = 1.0; tm[14] = V[2];
    tm[3] = 0.0; tm[7] = 0.0; tm[11] = 0.0; tm[15] = 1.0;
}

void im(float im[16])
{
    im[0] = 1.0; im[4] = 0.0;  im[8] = 0.0; im[12] = 0.0;
    im[1] = 0.0; im[5] = 1.0;  im[9] = 0.0; im[13] = 0.0;
    im[2] = 0.0; im[6] = 0.0; im[10] = 1.0; im[14] = 0.0;
    im[3] = 0.0; im[7] = 0.0; im[11] = 0.0; im[15] = 1.0;
}

void invertM(float M[16])
{
    float Mrot[16];
    float Mtrn[16];
    
    /* Rotation part from M, transposed. Other parts set to identity. */
    im(Mrot);
    Mrot[0] = M[0]; Mrot[4] = M[1];  Mrot[8] = M[2];
    Mrot[1] = M[4]; Mrot[5] = M[5];  Mrot[9] = M[6];
    Mrot[2] = M[8]; Mrot[6] = M[9]; Mrot[10] = M[10];

    /* Translation part from M, signs inverted. Other parts set to identity. */
    im(Mtrn);
    Mtrn[12] = -(M[12]);
    Mtrn[13] = -(M[13]);
    Mtrn[14] = -(M[14]);

    //MN(Mrot, Mtrn, M); //XXX: bork! bork! bork!
    MN(Mtrn, Mrot, M);
}

void MN(float M[16], float N[16], float result[16])
{
    result[0] = M[0]*N[0] + M[4]*N[1] + M[8]*N[2] + M[12]*N[3];
    result[1] = M[1]*N[0] + M[5]*N[1] + M[9]*N[2] + M[13]*N[3];
    result[2] = M[2]*N[0] + M[6]*N[1] + M[10]*N[2] + M[14]*N[3];
    result[3] = M[3]*N[0] + M[7]*N[1] + M[11]*N[2] + M[15]*N[3];

    result[4] = M[0]*N[4] + M[4]*N[5] + M[8]*N[6] + M[12]*N[7];
    result[5] = M[1]*N[4] + M[5]*N[5] + M[9]*N[6] + M[13]*N[7];
    result[6] = M[2]*N[4] + M[6]*N[5] + M[10]*N[6] + M[14]*N[7];
    result[7] = M[3]*N[4] + M[7]*N[5] + M[11]*N[6] + M[15]*N[7];

    result[8] = M[0]*N[8] + M[4]*N[9] + M[8]*N[10] + M[12]*N[11];
    result[9] = M[1]*N[8] + M[5]*N[9] + M[9]*N[10] + M[13]*N[11];
    result[10] = M[2]*N[8] + M[6]*N[9] + M[10]*N[10] + M[14]*N[11];
    result[11] = M[3]*N[8] + M[7]*N[9] + M[11]*N[10] + M[15]*N[11];

    result[12] = M[0]*N[12] + M[4]*N[13] + M[8]*N[14] + M[12]*N[15];
    result[13] = M[1]*N[12] + M[5]*N[13] + M[9]*N[14] + M[13]*N[15];
    result[14] = M[2]*N[12] + M[6]*N[13] + M[10]*N[14] + M[14]*N[15];
    result[15] = M[3]*N[12] + M[7]*N[13] + M[11]*N[14] + M[15]*N[15];
}

void Mv(float M[16], float v[3], float result[3])
{
    result[0] = M[0]*v[0] + M[4]*v[1] + M[8]*v[2] + M[12]*1.0;
    result[1] = M[1]*v[0] + M[5]*v[1] + M[9]*v[2] + M[13]*1.0;
    result[2] = M[2]*v[0] + M[6]*v[1] + M[10]*v[2]+ M[14]*1.0;
}

/* Quaternion math */
/* Q[3] is scalar part. Q[0], Q[1], Q[2] are vector part. */

void rmQ(float rm[16], float Q[4])
{
    
    rm[0] = 1 - (2 * (Q[1]*Q[1])) - (2 * (Q[2]*Q[2]));
    rm[1] = (2 * Q[0] * Q[1]) + (2 * Q[3] * Q[2]);
    rm[2] = (2 * Q[0] * Q[2]) - (2 * Q[3] * Q[1]);

    rm[4] = (2 * Q[0] * Q[1]) - (2 * Q[3] * Q[2]);
    rm[5] = 1 - (2 * (Q[0]*Q[0])) - (2 * (Q[2]*Q[2]));
    rm[6] = (2 * Q[0] * Q[2]) + (2 * Q[3] * Q[0]);

    rm[8] = (2 * Q[0] * Q[2]) + (2 * Q[3] * Q[1]);
    rm[9] = (2 * Q[1] * Q[2]) - (2 * Q[3] * Q[0]);
    rm[10]= 1 - (2 * (Q[0]*Q[0])) - (2 * (Q[1]*Q[1]));

                                            rm[12] = 0.0;
                                            rm[13] = 0.0;
                                            rm[14] = 0.0;
    rm[3] = 0.0; rm[7] = 0.0; rm[11] = 0.0; rm[15] = 1.0;
}

void iQ(float Q[4])
{
    Q[0] = 0.0; Q[1] = 0.0; Q[2] = 0.0; Q[3] = 1.0;
}

void normalizeQ(float Q[4])
{
                         /*<----------------- magnitude ------------------>*/
    float foo = ( 1.0f / sqrtf(Q[0]*Q[0] + Q[1]*Q[1] + Q[2]*Q[2] + Q[3]*Q[3]) );
    Q[0] = Q[0] * foo;
    Q[1] = Q[1] * foo;
    Q[2] = Q[2] * foo;
    Q[3] = Q[3] * foo;
}

void conjQ(float Q[4], float result[4])
{
    result[0] = -Q[0];
    result[1] = -Q[1];
    result[2] = -Q[2];
    result[3] = Q[3];
}

/* Quaternion from unit vector and angle */
void Qvangle(float Q[4], float v[3], float angle)
{
    /* radians */
    float ang = angle/180. * M_PI;
    float sine = sinf(ang/2);
    Q[0] = v[0] * sine;
    Q[1] = v[1] * sine;
    Q[2] = v[2] * sine;
    Q[3] = cosf(ang/2);
}

void QR(float Q[4], float R[4], float result[4])
{
    result[0] =  Q[0] * R[3] + Q[1] * R[2] - Q[2] * R[1] + Q[3] * R[0];
    result[1] = -Q[0] * R[2] + Q[1] * R[3] + Q[2] * R[0] + Q[3] * R[1];
    result[2] =  Q[0] * R[1] - Q[1] * R[0] + Q[2] * R[3] + Q[3] * R[2];
    result[3] = -Q[0] * R[0] - Q[1] * R[1] - Q[2] * R[2] + Q[3] * R[3];
}

void Qv(float Q[4], float v[3], float result[3])
{
    float conj[4];
    float tmpres[4];
    float tmpres2[4];
    float tmpv[4];
    tmpv[0] = v[0];
    tmpv[1] = v[1];
    tmpv[2] = v[2];
    tmpv[3] = 0.0f;
    
    conjQ(Q, conj);
    // Try other order...
    QR(Q, tmpv, tmpres);
    QR(tmpres, conj, tmpres2);

    result[0] = tmpres2[0]; result[1] = tmpres2[1]; result[2] = tmpres2[2];
}
