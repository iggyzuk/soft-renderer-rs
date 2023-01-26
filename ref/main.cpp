#include <SFML\Graphics.hpp>
#include <SFML\Window.hpp>
#include <iostream>
#include <fstream>
#include <sstream>
#include <assert.h>
#include <cstring>
#include <time.h>
#include <stdlib.h>
#include <math.h>
#include <vector>
#include <map>
#include <utility>

// #define PI 3.14159265

// typedef unsigned char byte;
// typedef unsigned int  uint;

// template <typename T>
// void LOG(const T& value) {
//     std::cout << value << std::endl;
// }

// template <typename U, typename... T>
// void LOG(const U& head, const T&... tail) {
//     std::cout << head << "; ";
//     LOG(tail...);
// }

// inline float random() {
//     return static_cast <float> (rand()) / static_cast <float> (RAND_MAX);
// }
// inline float clamp(float value, float lower, float upper) {
//     return value <= lower ? lower : value >= upper ? upper : value;
// }

// float lerp(float a, float b, float factor) {
//     return a * (1.0f-factor) + b * factor;
// }

// class Vector4;
// class Quaternion {
// public:
//     Quaternion(float x, float y, float z, float w) {
//         this->x = x;
//         this->y = y;
//         this->z = z;
//         this->w = w;
//     }
//     Quaternion(float angle, const Vector4& axis);

//     float length() const {
//         return (float)sqrt(x*x + y*y + z*z + w*w);
//     }
//     Quaternion normalize() {
//         float len = length();
//         if(len != 0) {
//             x /= len;
//             y /= len;
//             z /= len;
//             w /= len;
//         }
//         return *this;
//     }
//     Quaternion conjugate() const {
//         return Quaternion(-x, -y, -z, w);
//     }
//     Quaternion operator*(const Quaternion& q) const {
//         float ww = w * q.w - x * q.x - y * q.y - z * q.z;
//         float xx = x * q.w + w * q.x + y * q.z - z * q.y;
//         float yy = y * q.w + w * q.y + z * q.x - x * q.z;
//         float zz = z * q.w + w * q.z + x * q.y - y * q.x;
//         return Quaternion(xx, yy, zz, ww);
//     }
//     Quaternion operator*(const Vector4& v) const;

//     Quaternion operator+(const Quaternion& q) const {
//         return Quaternion(x + q.x, y + q.y, z + q.z, w + q.w);
//     }
//     Quaternion operator-(const Quaternion& q) const {
//         return Quaternion(x - q.x, y - q.y, z - q.z, w - q.w);
//     }
//     float dot(const Quaternion& q) const {
//         return x * q.x + y * q.y + z * q.z + w * q.w;
//     }
//     float x {0.0f};
//     float y {0.0f};
//     float z {0.0f};
//     float w {0.0f};
// };

// class Vector4 {
// public:
//     Vector4(float x = 0.0f, float y = 0.0f, float z = 0.0f, float w = 1.0f) {
//         this->x = x;
//         this->y = y;
//         this->z = z;
//         this->w = w;
//     }
//     float length() const {
//         return (float)sqrt(x*x + y*y + z*z + w*w);
//     }
//     Vector4 normalize() {
//         float len = length();
//         if(len != 0) {
//             x /= len;
//             y /= len;
//             z /= len;
//             w /= len;
//         }
//         return *this;
//     }
//     float dot(const Vector4& v) const {
//         return x * v.x + y * v.y + z * v.z + w * v.w;
//     }
//     Vector4 cross(const Vector4& v) {
//         float xx = y * v.z - z * v.y;
//         float yy = z * v.x - x * v.z;
//         float zz = x * v.y - y * v.x;
//         return Vector4(xx, yy, zz, 0);
//     }
//     Vector4 lerp(const Vector4& dest, float factor) {
//         return (*this) * (1.0f-factor) + dest * factor;
//     }
//     Vector4 operator+(const Vector4& v) const {
//         return Vector4(x + v.x, y + v.y, z + v.z, w + v.w);
//     }
//     Vector4 operator-(const Vector4& v) const {
//         return Vector4(x - v.x, y - v.y, z - v.z, w - v.w);
//     }
//     Vector4 operator*(const float factor) const {
//         return Vector4(x * factor, y * factor, z * factor, w * factor);
//     }
//     Vector4 operator/(const float factor) const {
//         assert(factor != 0);
//         return Vector4(x / factor, y / factor, z / factor, w / factor);
//     }
//     Vector4 rotate(Quaternion& rotation) {
//         Quaternion w = rotation * (*this) * rotation.conjugate();
//         return Vector4(w.x, w.y, w.z);
//     }
//     Vector4 rotate(Vector4 axis, float angle) {
//         float sinAngle = (float)sinf(-angle);
//         float cosAngle = (float)cosf(-angle);

//         return (*this).cross(axis * sinAngle) +           // Rotation on local X
//                (*this) * cosAngle +                       // Rotation on local Z
//                axis * (*this).dot(axis * (1 - cosAngle)); // Rotation on local Y
//     }
//     static Vector4 polar(float radius, float inclination, float azimuth) {
//         return Vector4(radius * sinf(inclination) * cosf(azimuth), radius * sinf(inclination) * sinf(azimuth), radius * cosf(inclination));
//     }
//     static Vector4 polarDegrees(float radius, float inclination, float azimuth) {
//         return polar(radius, inclination * PI / 180.0f, azimuth * PI / 180.0f);
//     }
//     float x {0.0f};
//     float y {0.0f};
//     float z {0.0f};
//     float w {0.0f};
// };
// std::ostream& operator<<(std::ostream& out, const Vector4& rhs) {
//     out << "(" << rhs.x << ", " << rhs.y << ", " << rhs.z << ")";
//     return out;
// }

// // Quaternion definitions of forward declared methods
// Quaternion::Quaternion(float angle, const Vector4& axis) {
//     float sinHalfAngle = sinf(angle / 2.0f);
//     float cosHalfAngle = cosf(angle / 2.0f);

//     this->x = axis.x * sinHalfAngle;
//     this->y = axis.y * sinHalfAngle;
//     this->z = axis.z * sinHalfAngle;
//     this->w = cosHalfAngle;
// }
// Quaternion Quaternion::operator*(const Vector4& v) const {
//     float ww = -x * v.x - y * v.y - z * v.z;
//     float xx =  w * v.x + y * v.z - z * v.y;
//     float yy =  w * v.y + z * v.x - x * v.z;
//     float zz =  w * v.z + x * v.y - y * v.x;
//     return Quaternion(xx, yy, zz, ww);
// }

// Vector4 g_lightDirection;

// class Matrix4 {
// public:
//     Matrix4() {
//         identity();
//     }
//     void identity() {
//         for (uint i = 0; i < 4; ++i) {
//             for (uint j = 0; j < 4; ++j) {
//                 matrix[j][i] = (i == j ? 1.0f : 0.0f);
//             }
//         }
//     }
//     void viewport(unsigned short width, unsigned short height) {
//         identity();

//         float halfWidth = (float)width / 2.0f;
//         float halfHeight = (float)height / 2.0f;

//         matrix[0][0] = halfWidth;
//         matrix[1][1] = -halfHeight;
//         matrix[3][0] = halfWidth - 0.5f;
//         matrix[3][1] = halfHeight - 0.5f;
//     }
//     void perspective(float fov, float aspectRatio, float zNear, float zFar) {
//         identity();

//         float tanHalfFOV = (float)tan((fov / 2) * (PI / 180));
//         float zRange = zNear - zFar;

//         matrix[0][0] = 1.0f / (tanHalfFOV * aspectRatio);
//         matrix[1][1] = 1.0f / tanHalfFOV;
//         matrix[2][2] = (-zNear -zFar)/zRange;
//         matrix[3][2] = 2.0f * zFar * zNear / zRange;
//         matrix[2][3] = 1.0f;
//     }
//     void lookAt(const Vector4& eye, const Vector4& target, const Vector4& upAxis) {
//         Vector4 forward, right, up;
//         Matrix4 m;

//         forward = target - eye;
//         up = upAxis;

//         forward.normalize();

//         right = up.cross(forward);
//         right.normalize();

//         up = forward.cross(right);
//         up.normalize();

//         m[0][0] = right.x;
//         m[1][0] = right.y;
//         m[2][0] = right.z;

//         m[0][1] = up.x;
//         m[1][1] = up.y;
//         m[2][1] = up.z;

//         m[0][2] = forward.x;
//         m[1][2] = forward.y;
//         m[2][2] = forward.z;

//         m.translate(-eye.x, -eye.y, -eye.z);

//         *this = *this * m;
//     }
//     Matrix4 operator*(const Matrix4& mat) {
//         Matrix4 m;
//         for (uint i = 0; i < 4; ++i) {
//             for (uint j = 0; j < 4; ++j) {
//                 m[i][j] = matrix[0][j] * mat.matrix[i][0] +
//                           matrix[1][j] * mat.matrix[i][1] +
//                           matrix[2][j] * mat.matrix[i][2] +
//                           matrix[3][j] * mat.matrix[i][3];
//             }
//         }
//         return m;
//     }
//     Vector4 operator*(const Vector4& vec) const {
//         float a[4];
//         for (uint i = 0; i < 4; ++i) {
//             a[i] = matrix[0][i] * vec.x +
//                    matrix[1][i] * vec.y +
//                    matrix[2][i] * vec.z +
//                    matrix[3][i] * vec.w;
//         }
//         return Vector4(a[0], a[1], a[2], a[3]);
//     }
//     float* operator[](int a) {
//         return matrix[a];
//     }
//     void translate(float x, float y, float z) {
//         Matrix4 m;
//         m[3][0] = x;
//         m[3][1] = y;
//         m[3][2] = z;
//         *this = *this * m;
//     }
//     void rotateX(float angle) {
//         Matrix4 m;
//         float radian = (angle * ((float)PI / 180.0f));
//         float sinus = sinf(radian);
//         float cosinus = cosf(radian);
//         m[1][1] = cosinus;
//         m[2][2] = cosinus;
//         m[1][2] = sinus;
//         m[2][1] = -sinus;
//         *this = *this * m;
//     }

//     void rotateY(float angle) {
//         Matrix4 m;
//         float radian = (angle * ((float)PI / 180.0f));
//         float sinus = sinf(radian);
//         float cosinus = cosf(radian);
//         m[0][0] = cosinus;
//         m[2][2] = cosinus;
//         m[0][2] = -sinus;
//         m[2][0] = sinus;
//         *this = *this * m;
//     }

//     void rotateZ(float angle) {
//         Matrix4 m;
//         float radian = (angle * ((float)PI / 180.0f));
//         float sinus = sinf(radian);
//         float cosinus = cosf(radian);
//         m[0][0] = cosinus;
//         m[1][1] = cosinus;
//         m[0][1] = sinus;
//         m[1][0] = -sinus;
//         *this = *this * m;
//     }
//     void scale(float x, float y, float z) {
//         Matrix4 m;
//         m[0][0] = x;
//         m[1][1] = y;
//         m[2][2] = z;
//         *this = *this * m;
//     }
//     bool invert() {
//         float inv[4][4];

//         inv[0][0] =     matrix[1][1]  * matrix[2][2] * matrix[3][3] -
//                         matrix[1][1]  * matrix[3][2] * matrix[2][3] -
//                         matrix[1][2]  * matrix[2][1]  * matrix[3][3] +
//                         matrix[1][2]  * matrix[3][1]  * matrix[2][3] +
//                         matrix[1][3] * matrix[2][1]  * matrix[3][2] -
//                         matrix[1][3] * matrix[3][1]  * matrix[2][2];

//         inv[0][1] =     -matrix[0][1]  * matrix[2][2] * matrix[3][3] +
//                         matrix[0][1]  * matrix[3][2] * matrix[2][3] +
//                         matrix[0][2]  * matrix[2][1]  * matrix[3][3] -
//                         matrix[0][2]  * matrix[3][1]  * matrix[2][3] -
//                         matrix[0][3] * matrix[2][1]  * matrix[3][2] +
//                         matrix[0][3] * matrix[3][1]  * matrix[2][2];

//         inv[0][2] =     matrix[0][1]  * matrix[1][2] * matrix[3][3] -
//                         matrix[0][1]  * matrix[3][2] * matrix[1][3] -
//                         matrix[0][2]  * matrix[1][1] * matrix[3][3] +
//                         matrix[0][2]  * matrix[3][1] * matrix[1][3] +
//                         matrix[0][3] * matrix[1][1] * matrix[3][2] -
//                         matrix[0][3] * matrix[3][1] * matrix[1][2];

//         inv[0][3] =     -matrix[0][1]  * matrix[1][2] * matrix[2][3] +
//                         matrix[0][1]  * matrix[2][2] * matrix[1][3] +
//                         matrix[0][2]  * matrix[1][1] * matrix[2][3] -
//                         matrix[0][2]  * matrix[2][1] * matrix[1][3] -
//                         matrix[0][3] * matrix[1][1] * matrix[2][2] +
//                         matrix[0][3] * matrix[2][1] * matrix[1][2];

//         inv[1][0] =     -matrix[1][0]  * matrix[2][2] * matrix[3][3] +
//                         matrix[1][0]  * matrix[3][2] * matrix[2][3] +
//                         matrix[1][2]  * matrix[2][0] * matrix[3][3] -
//                         matrix[1][2]  * matrix[3][0] * matrix[2][3] -
//                         matrix[1][3] * matrix[2][0] * matrix[3][2] +
//                         matrix[1][3] * matrix[3][0] * matrix[2][2];

//         inv[1][1] =     matrix[0][0]  * matrix[2][2] * matrix[3][3] -
//                         matrix[0][0]  * matrix[3][2] * matrix[2][3] -
//                         matrix[0][2]  * matrix[2][0] * matrix[3][3] +
//                         matrix[0][2]  * matrix[3][0] * matrix[2][3] +
//                         matrix[0][3] * matrix[2][0] * matrix[3][2] -
//                         matrix[0][3] * matrix[3][0] * matrix[2][2];

//         inv[1][2] =     -matrix[0][0]  * matrix[1][2] * matrix[3][3] +
//                         matrix[0][0]  * matrix[3][2] * matrix[1][3] +
//                         matrix[0][2]  * matrix[1][0] * matrix[3][3] -
//                         matrix[0][2]  * matrix[3][0] * matrix[1][3] -
//                         matrix[0][3] * matrix[1][0] * matrix[3][2] +
//                         matrix[0][3] * matrix[3][0] * matrix[1][2];

//         inv[1][3] =     matrix[0][0]  * matrix[1][2] * matrix[2][3] -
//                         matrix[0][0]  * matrix[2][2] * matrix[1][3] -
//                         matrix[0][2]  * matrix[1][0] * matrix[2][3] +
//                         matrix[0][2]  * matrix[2][0] * matrix[1][3] +
//                         matrix[0][3] * matrix[1][0] * matrix[2][2] -
//                         matrix[0][3] * matrix[2][0] * matrix[1][2];

//         inv[2][0] =     matrix[1][0]  * matrix[2][1] * matrix[3][3] -
//                         matrix[1][0]  * matrix[3][1] * matrix[2][3] -
//                         matrix[1][1]  * matrix[2][0] * matrix[3][3] +
//                         matrix[1][1]  * matrix[3][0] * matrix[2][3] +
//                         matrix[1][3] * matrix[2][0] * matrix[3][1] -
//                         matrix[1][3] * matrix[3][0] * matrix[2][1];

//         inv[2][1] =     -matrix[0][0]  * matrix[2][1] * matrix[3][3] +
//                         matrix[0][0]  * matrix[3][1] * matrix[2][3] +
//                         matrix[0][1]  * matrix[2][0] * matrix[3][3] -
//                         matrix[0][1]  * matrix[3][0] * matrix[2][3] -
//                         matrix[0][3] * matrix[2][0] * matrix[3][1] +
//                         matrix[0][3] * matrix[3][0] * matrix[2][1];

//         inv[2][2] =     matrix[0][0]  * matrix[1][1] * matrix[3][3] -
//                         matrix[0][0]  * matrix[3][1] * matrix[1][3] -
//                         matrix[0][1]  * matrix[1][0] * matrix[3][3] +
//                         matrix[0][1]  * matrix[3][0] * matrix[1][3] +
//                         matrix[0][3] * matrix[1][0] * matrix[3][1] -
//                         matrix[0][3] * matrix[3][0] * matrix[1][1];

//         inv[2][3] =     -matrix[0][0]  * matrix[1][1] * matrix[2][3] +
//                         matrix[0][0]  * matrix[2][1] * matrix[1][3] +
//                         matrix[0][1]  * matrix[1][0] * matrix[2][3] -
//                         matrix[0][1]  * matrix[2][0] * matrix[1][3] -
//                         matrix[0][3] * matrix[1][0] * matrix[2][1] +
//                         matrix[0][3] * matrix[2][0] * matrix[1][1];

//         inv[3][0] =     -matrix[1][0] * matrix[2][1] * matrix[3][2] +
//                         matrix[1][0] * matrix[3][1] * matrix[2][2] +
//                         matrix[1][1] * matrix[2][0] * matrix[3][2] -
//                         matrix[1][1] * matrix[3][0] * matrix[2][2] -
//                         matrix[1][2] * matrix[2][0] * matrix[3][1] +
//                         matrix[1][2] * matrix[3][0] * matrix[2][1];

//         inv[3][1] =     matrix[0][0] * matrix[2][1] * matrix[3][2] -
//                         matrix[0][0] * matrix[3][1] * matrix[2][2] -
//                         matrix[0][1] * matrix[2][0] * matrix[3][2] +
//                         matrix[0][1] * matrix[3][0] * matrix[2][2] +
//                         matrix[0][2] * matrix[2][0] * matrix[3][1] -
//                         matrix[0][2] * matrix[3][0] * matrix[2][1];

//         inv[3][2] =     -matrix[0][0] * matrix[1][1] * matrix[3][2] +
//                         matrix[0][0] * matrix[3][1] * matrix[1][2] +
//                         matrix[0][1] * matrix[1][0] * matrix[3][2] -
//                         matrix[0][1] * matrix[3][0] * matrix[1][2] -
//                         matrix[0][2] * matrix[1][0] * matrix[3][1] +
//                         matrix[0][2] * matrix[3][0] * matrix[1][1];

//         inv[3][3] =     matrix[0][0] * matrix[1][1] * matrix[2][2] -
//                         matrix[0][0] * matrix[2][1] * matrix[1][2] -
//                         matrix[0][1] * matrix[1][0] * matrix[2][2] +
//                         matrix[0][1] * matrix[2][0] * matrix[1][2] +
//                         matrix[0][2] * matrix[1][0] * matrix[2][1] -
//                         matrix[0][2] * matrix[2][0] * matrix[1][1];

//         // Find determinant and check if it's zero meaning matrix is not invertable
//         float det =     matrix[0][0] * inv[0][0] +
//                         matrix[1][0] * inv[0][1] +
//                         matrix[2][0] * inv[0][2] +
//                         matrix[3][0] * inv[0][3];

//         if (det == 0) return false;

//         // Fill the matrix with inverted values
//         det = 1.0 / det;
//         for(int j = 0; j < 4; j++){
//             for(int i = 0; i < 4; i++){
//                 matrix[j][i] = inv[j][i] * det;
//             }
//         }
//         return true;
//     }
//     inline Vector4 translation() const {
//         return Vector4(matrix[3][0], matrix[3][1], matrix[3][2]);
//     }
// private:
//     float matrix[4][4];
// };


class Color {
public:
    Color(byte r, byte g, byte b, byte a) {
        this->r = r;
        this->g = g;
        this->b = b;
        this->a = a;
    }
    Color(uint hex) {
        this->r = (hex >> 24) & 0xFF;
        this->g = (hex >> 16) & 0xFF;
        this->b = (hex >> 8 ) & 0xFF;
        this->a = (hex      ) & 0xFF;
    }
    static Color Random() {
        return Color(random()*0xFF, random()*0xFF, random()*0xFF, 0xFF);
    }
    static Color White() {
        return Color(0xFFFFFFFF);
    }
    static Color Grey() {
        return Color(0x151515FF);
    }
    static Color Red() {
        return Color(0xFF0000FF);
    }
    static Color Green() {
        return Color(0x00FF00FF);
    }
    static Color Blue() {
        return Color(0x0000FFFF);
    }
    Vector4 asVector4() const {
        return Vector4((float)r,(float)g,(float)b,(float)a);
    }

    byte r;
    byte g;
    byte b;
    byte a;
};

class Vertex {
public:
    Vertex(Vector4 position, Vector4 texcoords, Vector4 normal) :
        position(position),
        texcoords(texcoords),
        normal(normal) {
    }
    static Vertex Random() {
        Vector4 v((random() - 0.5f) * 2.0f, (random() - 0.5f) * 2.0f, 1.0f, 1.0f);
        return Vertex(v, Color::Random().asVector4(), Color::Random().asVector4());
    }
    Vertex transform(Matrix4& transformMat, Matrix4& normalMat) {
        return Vertex(transformMat * position, texcoords, normalMat * normal);
    }
    Vertex perspectiveDivide() const {
        return Vertex( Vector4( position.x/position.w,
                                position.y/position.w,
                                position.z/position.w,
                                position.w ),
                       texcoords,
                       normal );
    }
    float triangleAreaTimesTwo(Vertex b, Vertex c) {
        float x1 = b.position.x - position.x;
        float y1 = b.position.y - position.y;

        float x2 = c.position.x - position.x;
        float y2 = c.position.y - position.y;

        return (x1 * y2 - x2 * y1);
    }
    Vertex lerp(const Vertex& other, float lerpAmt) {
        return Vertex( position.lerp(other.position, lerpAmt),
                       texcoords.lerp(other.texcoords, lerpAmt),
                       normal.lerp(other.normal, lerpAmt) );
    }
    bool isInsideViewFrustum() {
        return fabs(position.x) <= fabs(position.w) &&
               fabs(position.y) <= fabs(position.w) &&
               fabs(position.z) <= fabs(position.w);
    }
    float get(int index) {
        if(index == 0) return position.x;
        if(index == 1) return position.y;
        if(index == 2) return position.z;
        if(index == 3) return position.w;
        return 0;
    }

    Vector4 position;
    Vector4 texcoords;
    Vector4 normal;
};

std::ostream& operator<<(std::ostream& out, const Vertex& rhs) {
    out << "(" << (int)rhs.position.x << ", " << (int)rhs.position.y << ", " << (int)rhs.position.z << ")";
    return out;
}

class Gradients {
public:
    Gradients(Vertex minYVert, Vertex midYVert, Vertex maxYVert) {

        depth[0] = minYVert.position.z;
        depth[1] = midYVert.position.z;
        depth[2] = maxYVert.position.z;

        lightAmt[0] = clamp(minYVert.normal.dot(g_lightDirection), 0.0f, 1.0f) * 0.75f + 0.25f;
        lightAmt[1] = clamp(midYVert.normal.dot(g_lightDirection), 0.0f, 1.0f) * 0.75f + 0.25f;
        lightAmt[2] = clamp(maxYVert.normal.dot(g_lightDirection), 0.0f, 1.0f) * 0.75f + 0.25f;

        oneOverZ[0] = 1.0f / minYVert.position.w;
        oneOverZ[1] = 1.0f / midYVert.position.w;
        oneOverZ[2] = 1.0f / maxYVert.position.w;

        texcoords[0] = minYVert.texcoords * oneOverZ[0];
        texcoords[1] = midYVert.texcoords * oneOverZ[1];
        texcoords[2] = maxYVert.texcoords * oneOverZ[2];

        float oneOverdX =
            1.0f /
            (((midYVert.position.x - maxYVert.position.x) *
            (minYVert.position.y - maxYVert.position.y)) -
            ((minYVert.position.x - maxYVert.position.x) *
            (midYVert.position.y - maxYVert.position.y)));
        float oneOverdY = -oneOverdX;

        texcoordsXStep = calcStepX(texcoords, minYVert, midYVert, maxYVert, oneOverdX);
        texcoordsYStep = calcStepY(texcoords, minYVert, midYVert, maxYVert, oneOverdY);

        oneOverZXStep = calcStepX(oneOverZ, minYVert, midYVert, maxYVert, oneOverdX);
        oneOverZYStep = calcStepY(oneOverZ, minYVert, midYVert, maxYVert, oneOverdY);

        depthXStep = calcStepX(depth, minYVert, midYVert, maxYVert, oneOverdX);
        depthYStep = calcStepY(depth, minYVert, midYVert, maxYVert, oneOverdY);

        lightAmtXStep = calcStepX(lightAmt, minYVert, midYVert, maxYVert, oneOverdX);
        lightAmtYStep = calcStepY(lightAmt, minYVert, midYVert, maxYVert, oneOverdY);
    }
    template<typename T>
    inline T calcStepX(T values[], Vertex minYVert, Vertex midYVert, Vertex maxYVert, float oneOverdX) {
        return ((values[1] - values[2]) *
               (minYVert.position.y - maxYVert.position.y) -
               (values[0] - values[2]) *
               (midYVert.position.y - maxYVert.position.y)) * oneOverdX;
    }
    template<typename T>
    inline T calcStepY(T values[], Vertex minYVert, Vertex midYVert, Vertex maxYVert, float oneOverdY) {
        return ((values[1] - values[2]) *
               (minYVert.position.x - maxYVert.position.x) -
               (values[0] - values[2]) *
               (midYVert.position.x - maxYVert.position.x)) * oneOverdY;
    }

    Vector4 texcoords[3];
    Vector4 texcoordsXStep;
    Vector4 texcoordsYStep;

    float oneOverZ[3];
    float oneOverZXStep;
    float oneOverZYStep;

    float depth[3];
    float depthXStep;
    float depthYStep;

    float lightAmt[3];
    float lightAmtXStep;
    float lightAmtYStep;
};

class Edge {
public:
    Edge(Gradients gradients, Vertex start, Vertex end, int startIndex) {
        yStart = (int)ceilf(start.position.y);
        yEnd   = (int)ceilf(end.position.y);

        float yDist = end.position.y - start.position.y;
        float xDist = end.position.x - start.position.x;

        float yPrestep = yStart - start.position.y;
        xStep = xDist / yDist;
        x = start.position.x + yPrestep * xStep;
        float xPrestep = x - start.position.x;

        texcoords = gradients.texcoords[startIndex] + (gradients.texcoordsXStep * xPrestep) + (gradients.texcoordsYStep * yPrestep);
        texcoordsStep = gradients.texcoordsYStep + gradients.texcoordsXStep * xStep;

        oneOverZ = gradients.oneOverZ[startIndex] + gradients.oneOverZXStep * xPrestep + gradients.oneOverZYStep * yPrestep;
        oneOverZStep = gradients.oneOverZYStep + gradients.oneOverZXStep * xStep;

        depth = gradients.depth[startIndex] + gradients.depthXStep * xPrestep + gradients.depthYStep * yPrestep;
        depthStep = gradients.depthYStep + gradients.depthXStep * xStep;

        lightAmt = gradients.lightAmt[startIndex] + gradients.lightAmtXStep * xPrestep + gradients.lightAmtYStep * yPrestep;
        lightAmtStep = gradients.lightAmtYStep + gradients.lightAmtXStep * xStep;
    }

    void Step() {
        x         = x + xStep;
        texcoords = texcoords + texcoordsStep;
        oneOverZ  = oneOverZ + oneOverZStep;
        depth     = depth + depthStep;
        lightAmt  = lightAmt + lightAmtStep;
    }

    float x;
    float xStep;
    int yStart;
    int yEnd;

    Vector4 texcoords;
    Vector4 texcoordsStep;

    float oneOverZ;
    float oneOverZStep;

    float depth;
    float depthStep;

    float lightAmt;
    float lightAmtStep;
};

class Bitmap {
public:
    Bitmap(unsigned short width, unsigned short height) {
        this->width = width;
        this->height = height;
        this->pixels = new byte[width * height * 4];
    }
    ~Bitmap() {
        delete[] pixels;
    }
    void clear(Color color) {
        for (uint x = 0; x < width; ++x) {
            for (uint y = 0; y < height; ++y) {
                setPixel(x, y, color);
            }
        }
    }
    void setPixel(uint x, uint y, Color color) {
        int index = (x + y * width) * 4;
        if(index < 0 || index >= width * height * 4) return;

        pixels[index]     = color.r; // R
        pixels[index + 1] = color.g; // G
        pixels[index + 2] = color.b; // B
        pixels[index + 3] = color.a; // A
    }
    void copyPixel(uint destX, uint destY, uint srcX, uint srcY, const Bitmap& src, float lightAmt) {
        int destIndex = (destX + destY * width) * 4;
        int srcIndex = (srcX + srcY * src.width) * 4;

        if(destX < 0 || destX >= width || srcX < 0 || srcX >= width) return;
        if(destIndex < 0 || destIndex >= width * height * 4) return;
        if(srcIndex < 0 || srcIndex >= src.width * src.height * 4) return;

        pixels[destIndex]     = (byte)(src.pixels[srcIndex    ] * lightAmt); // R
        pixels[destIndex + 1] = (byte)(src.pixels[srcIndex + 1] * lightAmt); // G
        pixels[destIndex + 2] = (byte)(src.pixels[srcIndex + 2] * lightAmt); // B
        pixels[destIndex + 3] = (byte)(src.pixels[srcIndex + 3]);            // A
    }
    static Bitmap LoadFromFile(const std::string& filename) {
        sf::Texture texture;
        texture.loadFromFile(filename);
        Bitmap bitmap(texture.getSize().x, texture.getSize().y);
        memcpy(bitmap.pixels, texture.copyToImage().getPixelsPtr(), bitmap.width * bitmap.height * 4);
        return bitmap;
    }
    unsigned short width;
    unsigned short height;
    byte* pixels;
};

class Display {
public:
    Display(Bitmap& bitmap, float scale) : bitmap(bitmap) {
        window.create(sf::VideoMode(bitmap.width * scale, bitmap.height * scale, 32), "Software Renderer");

        texture.create(bitmap.width, bitmap.height);
        sprite.setTexture(texture);
        sprite.scale(scale, scale);
    }
    void draw() {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed)
                window.close();
        }

        texture.update(bitmap.pixels);
        window.draw(sprite);
        window.display();
    }
    bool isOpen() {
        return window.isOpen();
    }
private:
    sf::RenderWindow window;
    sf::Texture      texture;
    sf::Sprite       sprite;
    Bitmap&          bitmap;
};

class StarsField {

    class Star {
    public:
        Star(uint id, float x, float y, float z, Color color) : color(color) {
            this->id = id;
            this->x  = x;
            this->y  = y;
            this->z  = z;
        }
        uint id;
        float x,y,z;
        Color color;
    };

public:
    StarsField(int numStars, float spread, float speed) {
        this->spread = spread;
        this->speed = speed;

        stars.resize(numStars);
        for(int i = 0; i < numStars; ++i) {
            initStar(i);
        }
    }
    ~StarsField() {
        for(uint i = 0; i < stars.size(); ++i) {
            delete stars[i];
        }
        stars.empty();
    }
    void initStar(int index) {
        float x = 2 * (random() - 0.5f) * spread;
        float y = 2 * (random() - 0.5f) * spread;
        float z = (random() + 0.001f) * spread;
        stars[index] = new Star(index, x, y, z, Color::Random());
    }
    void render(Bitmap& target, const float dt) {

        float halfFOV = tan((130.0f / 2.0f) * (PI / 180.0f));

        uint halfWidth = target.width / 2;
        uint halfHeight = target.height / 2;

        for(auto& star : stars) {
            star->z -= speed * dt;
            if(star->z <= 0.0f) initStar(star->id);

            int x = (star->x / (star->z * halfFOV)) * halfWidth + halfWidth;
            int y = (star->y / (star->z * halfFOV)) * halfHeight + halfHeight;

            if(x <= 0 || x > target.width || y <= 0 || y > target.height) {
                initStar(star->id);
            } else {
                target.setPixel(x, y, star->color);
            }


        }
    }
private:
    float speed;
    float spread;

    std::vector<Star*> stars;
};

template<typename T>
T StringToNumber(std::string string) {
    std::istringstream buffer(string);
    T value; buffer >> value;
    return value;
}

class OBJLoader {
public:

    struct OBJIndex {
        uint vertexIndex;
        uint texCoordIndex;
        uint normalIndex;
    };

    struct OBJModel {
        std::vector<Vector4>  vertices;
        std::vector<Vector4>  texCoords;
        std::vector<Vector4>  normals;
        std::vector<OBJIndex> indices;
    };

    struct IndexedModel {
        std::vector<Vector4>      vertices;
        std::vector<Vector4>      texCoords;
        std::vector<Vector4>      normals;
        std::vector<Vector4>      tangents;
        std::vector<uint> indices;
    };

    static OBJModel Load(std::string filename) {

        OBJModel model;

        std::ifstream file(filename, std::ios::binary | std::ios::in);
        if(file.is_open()){
            for(std::string line; std::getline(file,line);){
                std::vector<std::string> tokens = split(line, ' ');
                assert(tokens.size() > 0);

                if(tokens[0] == "#") continue;
                if(tokens[0] == "v") {
                    model.vertices.push_back( Vector4(StringToNumber<float>(tokens[1]),
                                                      StringToNumber<float>(tokens[2]),
                                                      StringToNumber<float>(tokens[3])) );
                }
                else if(tokens[0] == "vt") {
                    model.texCoords.push_back( Vector4(StringToNumber<float>(tokens[1]),
                                                       1.0f - StringToNumber<float>(tokens[2]),
                                                       0.0f,
                                                       1.0f) );
                }
                else if(tokens[0] == "vn") {
                    model.normals.push_back( Vector4(StringToNumber<float>(tokens[1]),
                                                     StringToNumber<float>(tokens[2]),
                                                     StringToNumber<float>(tokens[3]),
                                                     0.0f) );
                }
                else if(tokens[0] == "f") {
                    for(uint i = 1; i < tokens.size(); ++i) {
                        std::vector<std::string> indexTokens = split(tokens[i], '/');

                        OBJIndex index;
                        index.vertexIndex   = StringToNumber<uint>(indexTokens[0]) - 1;
                        index.texCoordIndex = StringToNumber<uint>(indexTokens[1]) - 1;
                        index.normalIndex   = StringToNumber<uint>(indexTokens[2]) - 1;

                        model.indices.push_back(index);
                    }
                }
            }
        }
        file.close();
        return model;
    }

    static IndexedModel ToIndexedModel(const OBJModel& obj) {

        IndexedModel model;
        std::map<uint, uint> indexMap; // OBJModel.indices -> IndexModel.indices
        uint currentVertexIndex = 0;

        for(uint i = 0; i < obj.indices.size(); ++i) {
            OBJIndex currentIndex = obj.indices[i];

            Vector4 currentPosition = obj.vertices[currentIndex.vertexIndex];
            Vector4 currentTexCoord = obj.texCoords[currentIndex.texCoordIndex];
            Vector4 currentNormal   = obj.normals[currentIndex.normalIndex];

            // Check for duplicates O(n^2)
            int previousVertexIndex = -1;
            for(uint j = 0; j < i; ++j) {
                OBJIndex oldIndex = obj.indices[j];

                if(currentIndex.vertexIndex == oldIndex.vertexIndex &&
                   currentIndex.texCoordIndex == oldIndex.texCoordIndex &&
                   currentIndex.normalIndex == oldIndex.normalIndex) {
                    previousVertexIndex = j;
                    break;
                }
            }

            if(previousVertexIndex == -1) {
                indexMap[i] = currentVertexIndex;

                model.vertices.push_back(currentPosition);
                model.texCoords.push_back(currentTexCoord);
                model.normals.push_back(currentNormal);
                model.indices.push_back(currentVertexIndex);
                currentVertexIndex++;
            }
            else {
                model.indices.push_back(indexMap[(uint)previousVertexIndex]);
            }
        }
        return model;
    }

private:

    static std::vector<std::string>& split(const std::string& s, char delim, std::vector<std::string>& elems) {
        std::stringstream ss(s);
        std::string item;
        while (std::getline(ss, item, delim)) {
            elems.push_back(item);
        }
        return elems;
    }
    static std::vector<std::string> split(const std::string& s, char delim) {
        std::vector<std::string> elems;
        split(s, delim, elems);
        return elems;
    }
};

class Mesh {
public:
    Mesh(std::string filename) {
        OBJLoader::IndexedModel model = OBJLoader::ToIndexedModel(OBJLoader::Load(filename));

        for(uint i = 0; i < model.vertices.size(); ++i) {
            vertices.push_back(Vertex( model.vertices[i],
                                       model.texCoords[i],
                                       model.normals[i] ));
        }

        indices.resize(model.indices.size());
        for(uint j = 0; j < model.indices.size(); ++j) {
            indices[j] = model.indices[j];
        }
    }
    std::vector<Vertex> vertices;
    std::vector<uint> indices;
};

class RenderContext : public Bitmap {
public:
    RenderContext(unsigned short width, unsigned short height) :
        Bitmap(width, height) {
        zBuffer = new float[width * height];
        clearDepthBuffer();
    }
    ~RenderContext() {
        delete[] zBuffer;
    }
    void clearDepthBuffer() {
        uint size = width * height;
        for(uint i = 0; i < size; ++i) {
            zBuffer[i] = 1.0f;
        }
    }
    void drawMesh(Mesh mesh, Matrix4 viewProjection, Matrix4 transform, const Bitmap& texture) {

        Matrix4 mvp = viewProjection * transform;

        for(uint i = 0; i < mesh.indices.size(); i += 3) {
            drawTriangle( mesh.vertices[mesh.indices[i    ]].transform(mvp, transform),
                          mesh.vertices[mesh.indices[i + 1]].transform(mvp, transform),
                          mesh.vertices[mesh.indices[i + 2]].transform(mvp, transform),
                          texture );
        }
    }
    void drawTriangle(Vertex v1, Vertex v2, Vertex v3, const Bitmap& texture) {

        bool v1Inside = v1.isInsideViewFrustum();
        bool v2Inside = v2.isInsideViewFrustum();
        bool v3Inside = v3.isInsideViewFrustum();

        if(v1Inside && v2Inside && v3Inside) {
            fillTriangle(v1, v2, v3, texture);
            return;
        }

        //if(!v1Inside && !v2Inside && !v3Inside) return;

        std::vector<Vertex> vertices;
        std::vector<Vertex> auxiliaryList;

        vertices.push_back(v1);
        vertices.push_back(v2);
        vertices.push_back(v3);

        if(clipPolygonAxis(vertices, auxiliaryList, 0) &&
           clipPolygonAxis(vertices, auxiliaryList, 1) &&
           clipPolygonAxis(vertices, auxiliaryList, 2)) {

            Vertex initialVertex = vertices[0];
            for(uint i = 1; i < vertices.size() - 1; ++i) {
                fillTriangle(initialVertex, vertices[i], vertices[i + 1], texture);
            }
        }
    }
private:
    bool clipPolygonAxis(std::vector<Vertex>& vertices, std::vector<Vertex>& auxiliaryList, int componentIndex) {
        clipPolygonComponent(vertices, componentIndex, 1.0f, auxiliaryList);
        vertices.clear();

        if(auxiliaryList.empty()) return false;

        clipPolygonComponent(auxiliaryList, componentIndex, -1.0f, vertices);
        auxiliaryList.clear();

        return !vertices.empty();

    }
    void clipPolygonComponent(std::vector<Vertex>& vertices, int componentIndex, float componentFactor, std::vector<Vertex>& result) {
        Vertex previousVertex = vertices[vertices.size() - 1];
        float previousComponent = previousVertex.get(componentIndex) * componentFactor;
        bool previousInside = previousComponent <= previousVertex.position.w;

        for(auto& currentVertex : vertices) {
            float currentComponent = currentVertex.get(componentIndex) * componentFactor;
            bool currentInside = currentComponent <= currentVertex.position.w;

            if(currentInside ^ previousInside) {
                float lerpAmt = (previousVertex.position.w - previousComponent) /
                                ((previousVertex.position.w - previousComponent) -
                                (currentVertex.position.w - currentComponent));

                result.push_back(previousVertex.lerp(currentVertex, lerpAmt));
            }

            if(currentInside) {
                result.push_back(currentVertex);
            }

            previousVertex = currentVertex;
            previousComponent = currentComponent;
            previousInside = currentInside;
        }
    }
    void fillTriangle(Vertex v1, Vertex v2, Vertex v3, const Bitmap& texture) {
        Matrix4 screenspace;
        screenspace.viewport(width, height);

        Matrix4 indentity;

        Vertex minYVert = v1.transform(screenspace, indentity).perspectiveDivide();
        Vertex midYVert = v2.transform(screenspace, indentity).perspectiveDivide();
        Vertex maxYVert = v3.transform(screenspace, indentity).perspectiveDivide();

        if(minYVert.triangleAreaTimesTwo(maxYVert, midYVert) >= 0)
            return;

        if(maxYVert.position.y < midYVert.position.y) {
            Vertex temp = maxYVert;
            maxYVert = midYVert;
            midYVert = temp;
        }

        if(midYVert.position.y < minYVert.position.y) {
            Vertex temp = midYVert;
            midYVert = minYVert;
            minYVert = temp;
        }

        if(maxYVert.position.y < midYVert.position.y) {
            Vertex temp = maxYVert;
            maxYVert = midYVert;
            midYVert = temp;
        }

        scanTriangle(minYVert, midYVert, maxYVert, minYVert.triangleAreaTimesTwo(maxYVert, midYVert) >= 0, texture);
    }
    void scanTriangle(Vertex minYVert, Vertex midYVert, Vertex maxYVert, bool handedness, const Bitmap& texture) {
        Gradients gradients (minYVert, midYVert, maxYVert);
        Edge topToBottom    (gradients, minYVert, maxYVert, 0);
        Edge topToMiddle    (gradients, minYVert, midYVert, 0);
        Edge middleToBottom (gradients, midYVert, maxYVert, 1);

        scanEdges(gradients, topToBottom, topToMiddle, handedness, texture);
        scanEdges(gradients, topToBottom, middleToBottom, handedness, texture);
    }
    void scanEdges(const Gradients& gradients, Edge& a, Edge& b, bool handedness, const Bitmap& texture) {
        Edge* left = &a;
        Edge* right = &b;

        if(handedness) {
            Edge* temp = left;
            left = right;
            right = temp;
        }

        uint yStart = b.yStart;
        uint yEnd = b.yEnd;

        for (uint j = yStart; j < yEnd; ++j) {
            drawScanLine(gradients, *left, *right, j, texture);
            left->Step();
            right->Step();
        }
    }
    void drawScanLine(const Gradients& gradients, const Edge& left, const Edge& right, uint j, const Bitmap& texture) {
        int xMin = (int)ceilf(left.x);
        int xMax = (int)ceilf(right.x);
        float xPrestep = xMin - left.x;

        float texCoordXXStep = gradients.texcoordsXStep.x;
        float texCoordYXStep = gradients.texcoordsXStep.y;
        float oneOverZXStep  = gradients.oneOverZXStep;
        float depthXStep     = gradients.depthXStep;
        float lightAmtXStep  = gradients.lightAmtXStep;

        float texCoordX = left.texcoords.x + texCoordXXStep * xPrestep;
        float texCoordY = left.texcoords.y + texCoordYXStep * xPrestep;
        float oneOverZ  = left.oneOverZ + oneOverZXStep * xPrestep;
        float depth     = left.depth + depthXStep * xPrestep;
        float lightAmt  = left.lightAmt + lightAmtXStep * xPrestep;

        for(int i = xMin; i < xMax; ++i) {

            int index = i + j * width;

            if(depth < zBuffer[index]) {
                zBuffer[index] = depth;

                float z = 1.0f / oneOverZ;
                int srcX = (int)((texCoordX * z) * (float)(texture.width - 1) + 0.5f);
                int srcY = (int)((texCoordY * z) * (float)(texture.height - 1) + 0.5f);

                copyPixel(i, j, srcX, srcY, texture, lightAmt);
            }

            texCoordX += texCoordXXStep;
            texCoordY += texCoordYXStep;
            oneOverZ  += oneOverZXStep;
            depth     += depthXStep;
            lightAmt  += lightAmtXStep;
        }
    }

private:
    float* zBuffer;
};

class Animation {
public:
    Animation(int totalFrames, float frameTimeInSeconds) {
        this->currentFrame = 0;
        this->totalFrames = totalFrames;
        this->frameTimeInSeconds = frameTimeInSeconds;
        this->timer = 0.0f;
    }
    void addFrame(Mesh&& mesh) {
        frames.push_back(mesh);
    }
    Mesh& frame() {
        return frames[currentFrame];
    }
    void animate(const float& dt) {
        timer += dt;
        while(timer > frameTimeInSeconds) {
            currentFrame++;
            if(currentFrame >= totalFrames) {
                currentFrame = 0;
            }
            timer -= frameTimeInSeconds;
        }
    }

private:
    std::vector<Mesh> frames;
    uint currentFrame;
    uint totalFrames;
    float frameTimeInSeconds;
    float timer;
};

class Instance {
public:
    Instance(const Mesh& mesh, const Bitmap& texture) :
        mesh(mesh),
        texture(texture) {
    }
    void render(RenderContext& target, Matrix4 vp) {
        target.drawMesh(mesh, vp, transform, texture);
    }

    const Mesh&   mesh;
    const Bitmap& texture;
    Matrix4       transform;
};

class Camera {
public:
    Camera() {
        this->position = Vector4(0.0f, 5.0f, 10.0f, 1.0f);
        this->direction = Vector4(0.0f, 0.0f, -1.0f, 0.0f);
    }
    void update(const float& dt) {

        // Movement
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) turbo = true;
        else turbo = false;
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) speed += turbo ? 4.0f : 1.0f;
        else if(sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) speed -= turbo ? 4.0f : 1.0f;

        // Orientation
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::A)) hAngle -= 2.0f * dt;
        else if(sf::Keyboard::isKeyPressed(sf::Keyboard::D)) hAngle += 2.0f * dt;
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::S)) vAngle -= 2.0f * dt;
        else if(sf::Keyboard::isKeyPressed(sf::Keyboard::W)) vAngle += 2.0f * dt;

        hAnglef = lerp(hAnglef, hAngle, dt * 10.0f);
        vAnglef = lerp(vAnglef, vAngle, dt * 10.0f);

        // Quaternions
        Quaternion horizontalQuat (hAnglef, Vector4(0.0f, 1.0f, 0.0f));
        Quaternion verticalQuat   (vAnglef, Vector4(1.0f, 0.0f, 0.0f));

        Quaternion viewQuat = horizontalQuat * verticalQuat;

        direction = Vector4(0.0f, 0.0f, -1.0f, 0.0f);
        direction = direction.rotate(viewQuat);

        position = position + direction * speed * dt;
        speed *= friction;
    }

    Matrix4& matrix() {
        transform.identity();
        transform.lookAt(position, position + direction, Vector4(0.0f, 1.0f, 0.0f));
        return transform;
    }

private:
    Vector4 position;
    Vector4 direction;

    bool  turbo = false;
    float speed {0.0f};
    float friction {0.8f};

    float hAngle {0.0f};
    float vAngle {0.0f};
    float hAnglef {0.0f};
    float vAnglef {0.0f};

    Matrix4 transform;
};

int main() {

    srand(time(nullptr));

    float scale = 2.0f;

    RenderContext context(1080 / scale, 720 / scale);
    Display display(context, scale);
    StarsField starfield(4096, 64.0f, 4.0f);

    sf::Clock clock;
    float counter = 0.0f;

    Matrix4 projection;
    projection.perspective(90.0f, 800.0f/600.0f, 0.1f, 100.0f);

    Bitmap texture(16, 16);
    for(int j = 0; j < texture.height; ++j) {
        for(int i = 0; i < texture.width; ++i) {
            bool isLight = (i + j) % 2 == 0;
            if(isLight) {
                texture.setPixel(i, j, Color(0xFEDB00FF));
            } else {
                texture.setPixel(i, j, Color(0xFF9536FF));
            }
        }
    }

    Bitmap marioTex = Bitmap::LoadFromFile("assets/mario.png");
    Bitmap turtleTex = Bitmap::LoadFromFile("assets/turtle.png");

    Mesh model1("assets/mario.obj");
    Mesh model2("assets/turtle.obj");
    Mesh model3("assets/box.obj");
    Mesh model4("assets/plane.obj");

    Instance mario  (model1, marioTex);
    Instance turtle (model2, turtleTex);
    Instance box    (model3, texture);
    Instance plane  (model4, texture);

    Animation anim(8, 0.08f);
    anim.addFrame(Mesh("assets/animation/turtle1.obj"));
    anim.addFrame(Mesh("assets/animation/turtle2.obj"));
    anim.addFrame(Mesh("assets/animation/turtle3.obj"));
    anim.addFrame(Mesh("assets/animation/turtle4.obj"));
    anim.addFrame(Mesh("assets/animation/turtle5.obj"));
    anim.addFrame(Mesh("assets/animation/turtle6.obj"));
    anim.addFrame(Mesh("assets/animation/turtle7.obj"));
    anim.addFrame(Mesh("assets/animation/turtle8.obj"));

    Camera camera;

    while(display.isOpen()) {

        float dt = clock.restart().asSeconds();
        counter += dt;

        camera.update(dt);

        // Clear screen & depth buffer
        context.clear(Color::Grey());
        context.clearDepthBuffer();

        starfield.render(context, dt);

        // Matrix transformations
        g_lightDirection = Vector4(cosf(counter * 0.25f) * 5.0f, sinf(counter) + 2.0f, sinf(counter * 0.25f));
        g_lightDirection.normalize();

        // Mario
        mario.transform.identity();
        mario.transform.lookAt(mario.transform.translation(), turtle.transform.translation() - mario.transform.translation(), Vector4(0.0f, 1.0f, 0.0f));
        mario.transform.invert();
        mario.transform.translate(5.0f, -1.5f, 0.0f);
        mario.transform.scale(3.0f, 3.0f, 3.0f);

        // Turtle
        turtle.transform.identity();
        turtle.transform.translate(cosf(counter) * 25.0f, 0.0f, sinf(counter) * 25.0f);

        // Portal
        plane.transform.identity();
        plane.transform.translate(0.0f, 10.0f, 0.0f);
        plane.transform.rotateY(counter * 40.0f);
        plane.transform.translate(15.0f, 0.0f, 0.0f);
        plane.transform.rotateY(90.0f);
        plane.transform.rotateX(cosf(counter) * 25.0f);
        plane.transform.rotateZ(sinf(counter) * 5.0f);

        // Box
        box.transform.identity();
        box.transform.translate(0.0f, -2.5f, 0.0f);
        box.transform.scale(100.0f, 1.0f, 100.0f);


        Matrix4 viewProjection = projection * camera.matrix();

        anim.animate(dt);
        context.drawMesh(anim.frame(), viewProjection, Matrix4(), turtleTex);

        // Finally render everything
        turtle.render(context, viewProjection);
        mario.render(context, viewProjection);
        plane.render(context, viewProjection);
        box.render(context, viewProjection);

        display.draw();
    }

    return 0;
}
