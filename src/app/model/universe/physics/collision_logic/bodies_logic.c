#include <inttypes.h>

typedef struct {
    float x;
    float y;
} Vec2;

typedef struct {
    Vec2 position;
    float radius;
    float mass;
    Vec2 velocity;
} Body;

typedef struct {
    Vec2 delta1;
    Vec2 delta2;
} DeltaVelocities;

// return 0 if no collision
// return 1 if collision
uint8_t check_collision(Body body1, Body body2) {
    if (body2.position.x - body1.position.x < body1.radius) {
        return 1;
    }

    return 0;
}

// https://ru.wikipedia.org/wiki/Удар#Реальный_удар
// При коллизии.
// Нужно рассчитать насколько изменятся векторы скорости у тел.
// Предположительно, ты получишь новые скорости после вычислений, но
// от них нужно вычесть старые скорости и вернуть из функции.
// Если считать как в википедии, то пусть коэффициент восстановления
// будет как у стали: 0,55
DeltaVelocities calc_collision(Body body1, Body body2) {
    (void)body1;
    (void)body2;

    Vec2 delta1 = { .x = 0, .y = 0 };
    Vec2 delta2 = { .x = 100, .y = 100 };
    DeltaVelocities result = { .delta1 = delta1, .delta2 = delta2 };
    return result;
}
