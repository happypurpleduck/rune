use super::{Release, ReleaseEffect, ReleaseEvent};
use crate::machine::Effects;

#[test]
fn a_paint_before_the_first_frame_keeps_the_fixed_size() {
    let mut release = Release::Fixed;
    assert_eq!(release.step(ReleaseEvent::Paint), Effects::none());
    assert_eq!(release, Release::Fixed);
}

#[test]
fn the_first_frame_asks_for_the_paint_that_releases_the_limits() {
    let mut release = Release::Fixed;
    assert_eq!(release.step(ReleaseEvent::FirstFrame), Effects::one(ReleaseEffect::AskPaint));
    assert_eq!(release.step(ReleaseEvent::FirstFrame), Effects::none(), "asked once");
    assert_eq!(release.step(ReleaseEvent::Paint), Effects::one(ReleaseEffect::Release));
    assert_eq!(release, Release::Done);
}

#[test]
fn the_limits_are_released_once() {
    let mut release = Release::Done;
    for event in [ReleaseEvent::FirstFrame, ReleaseEvent::Paint] {
        assert_eq!(release.step(event), Effects::none());
    }
    assert_eq!(release, Release::Done);
}
