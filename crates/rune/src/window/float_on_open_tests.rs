use super::{Release, ReleaseEffect, ReleaseEvent};

#[test]
fn a_paint_before_the_first_frame_keeps_the_fixed_size() {
    assert_eq!(Release::Fixed.step(ReleaseEvent::Paint), (Release::Fixed, ReleaseEffect::None));
}

#[test]
fn the_first_frame_asks_for_the_paint_that_releases_the_limits() {
    let (owed, effect) = Release::Fixed.step(ReleaseEvent::FirstFrame);
    assert_eq!((owed, effect), (Release::Owed, ReleaseEffect::AskPaint));
    assert_eq!(owed.step(ReleaseEvent::FirstFrame), (Release::Owed, ReleaseEffect::None), "asked once");
    assert_eq!(owed.step(ReleaseEvent::Paint), (Release::Done, ReleaseEffect::Release));
}

#[test]
fn the_limits_are_released_once() {
    for event in [ReleaseEvent::FirstFrame, ReleaseEvent::Paint] {
        assert_eq!(Release::Done.step(event), (Release::Done, ReleaseEffect::None));
    }
}
