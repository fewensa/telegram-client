
use rtdlib::types::*;
use crate::types::*;
use crate::api::TDFB;


#[doc(hidden)] pub struct _TGInputMessageAnimationBuilder { inner: TGInputMessageAnimation }

impl _TGInputMessageAnimationBuilder {

  pub fn build(&self) -> TGInputMessageAnimation { self.inner.clone() }

  ///  Duration of the animation, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Width of the animation; may be replaced by the server. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Height of the animation; may be replaced by the server. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  

  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_animation(animation);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageAnimation {
  pub fn builder() -> _TGInputMessageAnimationBuilder {
    _TGInputMessageAnimationBuilder { inner: Self::new(InputMessageAnimation::_new()) }
  }
}

impl TDFB for TGInputMessageAnimation {}

impl AsRef<TGInputMessageAnimation> for TGInputMessageAnimation {
  fn as_ref(&self) -> &TGInputMessageAnimation { self }
}

impl AsRef<TGInputMessageAnimation> for _TGInputMessageAnimationBuilder {
  fn as_ref(&self) -> &TGInputMessageAnimation { &self.inner }
}

