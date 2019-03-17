use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH};
use amethyst::{
  core::transform::Transform,
  ecs::prelude::{Join, ReadExpect, System, Write, WriteStorage},
  ui::UiText,
};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
  type SystemData = (
    WriteStorage<'s, Ball>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, UiText>,
    Write<'s, ScoreBoard>,
    ReadExpect<'s, ScoreText>,
  );

  fn run(
    &mut self,
    (mut balls, mut locals, mut ui_text, mut scores, score_text): Self::SystemData,
  ) {
    for (ball, transform) in (&mut balls, &mut locals).join() {
      let ball_x = transform.translation().x;

      let did_hit = if ball_x <= ball.radius {
        // Right player scored on the left side.
        scores.score_right = (scores.score_right + 1).min(999);
        if let Some(text) = ui_text.get_mut(score_text.p2_score) {
          text.text = scores.score_right.to_string();
        }

        true
      } else if ball_x >= ARENA_WIDTH - ball.radius {
        // Left player scored on the right side.
        scores.score_left = (scores.score_left + 1).min(999);
        if let Some(text) = ui_text.get_mut(score_text.p1_score) {
          text.text = scores.score_left.to_string();
        }

        true
      } else {
        false
      };

      if did_hit {
        ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
        transform.set_x(ARENA_WIDTH / 2.0); // Reset Position
        if let Some(text) = ui_text.get_mut(score_text.p2_score) {
          text.text = scores.score_right.to_string();
        }
        // Reset stroke counter
        scores.stroke_left = 0;
        scores.stroke_right = 0;
        if let Some(text) = ui_text.get_mut(score_text.p1_stroke) {
          text.text = scores.stroke_left.to_string();
        }
        if let Some(text) = ui_text.get_mut(score_text.p2_stroke) {
          text.text = scores.stroke_right.to_string();
        }
      }
    }
  }
}
