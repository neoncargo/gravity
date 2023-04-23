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
    
    const float cor = 0.55; /* coefficient of restitution (COR), COR of steel = 0.55 */
	
    Vec2 delta1 = { .x = 0, .y = 0 };
    Vec2 delta2 = { .x = 0, .y = 0 };
    Vec2 velocity1_after_collision = { .x = 0, .y = 0 };
    Vec2 velocity2_after_collision= { .x = 0, .y = 0 };
	
    DeltaVelocities result = { .delta1 = 0, .delta2 = 0 };
	
    velocity1_after_collision.x = body1.velocity.x - (1 + cor) * ( body2.mass / (body1.mass + body2.mass) ) * (body1.velocity.x - body2.velocity.x);
    velocity1_after_collision.y = body1.velocity.y - (1 + cor) * ( body2.mass / (body1.mass + body2.mass) ) * (body1.velocity.y - body2.velocity.y);
	
    velocity2_after_collision.x = body2.velocity.x + (1 + cor) * ( body1.mass / (body1.mass + body2.mass) ) * (body1.velocity.x - body2.velocity.x);
    velocity2_after_collision.y = body2.velocity.y + (1 + cor) * ( body1.mass / (body1.mass + body2.mass) ) * (body1.velocity.y - body2.velocity.y);
	
    result.delta1.x = velocity1_after_collision.x - body1.velocity.x;
    result.delta1.y = velocity1_after_collision.y - body1.velocity.y;
	
    result.delta2.x = velocity2_after_collision.x - body2.velocity.x;
    result.delta2.y = velocity2_after_collision.y - body2.velocity.y;
	
    return result;
}
