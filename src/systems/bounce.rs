use amethyst::{
  core::transform::Transform,
  ecs::prelude::{Join, ReadExpect, ReadStorage, System, Write, WriteStorage},
  ui::UiText,
};

use crate::pong::{Ball, Paddle, ScoreBoard, ScoreText, Side, ARENA_HEIGHT};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
  type SystemData = (
    WriteStorage<'s, Ball>,
    ReadStorage<'s, Paddle>,
    WriteStorage<'s, UiText>,
    ReadStorage<'s, Transform>,
    Write<'s, ScoreBoard>,
    ReadExpect<'s, ScoreText>,
  );

  fn run(
    &mut self,
    (mut balls, paddles, mut ui_text, transforms, mut scores, score_text): Self::SystemData,
  ) {
    // Check whether a ball collided, and bounce off accordingly.
    //
    // We also check for the velocity of the ball every time, to prevent multiple collisions
    // from occurring.
    for (ball, transform) in (&mut balls, &transforms).join() {
      let ball_x = transform.translation().x;
      let ball_y = transform.translation().y;

      // Bounce at the top or the bottom of the arena.
      if ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0 {
        ball.velocity[1] = -ball.velocity[1];
      } else if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
        ball.velocity[1] = -ball.velocity[1];
      }

      // Bounce at the paddles.
      for (paddle, paddle_transform) in (&paddles, &transforms).join() {
        let paddle_x = paddle_transform.translation().x - paddle.width * 0.5;
        let paddle_y = paddle_transform.translation().y - paddle.height * 0.5;

        // To determine whether the ball has collided with a paddle, we create a larger
        // rectangle around the current one, by subtracting the ball radius from the
        // lowest coordinates, and adding the ball radius to the highest ones. The ball
        // is then within the paddle if its centre is within the larger wrapper
        // rectangle.
        if point_in_rect(
          ball_x,
          ball_y,
          paddle_x - ball.radius,
          paddle_y - ball.radius,
          paddle_x + paddle.width + ball.radius,
          paddle_y + paddle.height + ball.radius,
        ) {
          if paddle.side == Side::Left && ball.velocity[0] < 0.0 {
            ball.velocity[0] = -ball.velocity[0];

            // Count strokes per serve
            scores.stroke_left = (scores.stroke_left + 1).min(999);
            if let Some(text) = ui_text.get_mut(score_text.p1_stroke) {
              text.text = scores.stroke_left.to_string();
            }

            // Count max strokes
            if scores.stroke_left >= scores.stroke_max_left {
              scores.stroke_max_left = scores.stroke_left;
              if let Some(text) = ui_text.get_mut(score_text.p1_max_stroke) {
                text.text = scores.stroke_max_left.to_string();
              }
            }
          } else if paddle.side == Side::Right && ball.velocity[0] > 0.0 {
            ball.velocity[0] = -ball.velocity[0];

            // Count strokes per serve
            scores.stroke_right = (scores.stroke_right + 1).min(999);
            if let Some(text) = ui_text.get_mut(score_text.p2_stroke) {
              text.text = scores.stroke_right.to_string();
            }

            // Count max strokes
            if scores.stroke_right >= scores.stroke_max_right {
              scores.stroke_max_right = scores.stroke_right;
              if let Some(text) = ui_text.get_mut(score_text.p2_max_stroke) {
                text.text = scores.stroke_max_right.to_string();
              }
            }
          }
        }
      }
    }
  }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
  x >= left && x <= right && y >= bottom && y <= top
}
