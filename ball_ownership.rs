struct Ball{
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    r: i32,
}

struct Arena{
    w: i32,
    h: i32,
}

/*
    The ball is moved one step, bouncing at borders.
*/
fn move_ball(mut b: Ball, a: &Arena) -> Ball{
    let mut dist_x = 0;
    let mut dist_y = 0;

    if b.x + b.r + b.dx < 0 || b.x + b.r + b.dx > a.w {
        b.dx = -b.dx;
        dist_x = a.w - (b.x + b.r);
    }
    if b.y + b.r + b.dy < 0 || b.y + b.r + b.dy > a.h {
        b.dy = -b.dy;
        dist_y = a.h - (b.y + b.r);
    }

    b.x += b.dx + 2 * dist_x;
    b.y += b.dy + 2 * dist_y;
    b
}

fn main(){
    let arena = Arena{
        w: 300,
        h: 200,
    };

    let mut ball = Ball{
        x: 278,
        y: 180,
        dx: 5,
        dy: 5,
        r: 20,
    };

    println!("Ball (before move): x = {}, y = {}, dx = {}, dy = {}", ball.x, ball.y, ball.dx, ball.dy);

    ball = move_ball(ball, &arena);

    println!("Ball (after move): x = {}, y = {}, dx = {}, dy = {}", ball.x, ball.y, ball.dx, ball.dy);
}