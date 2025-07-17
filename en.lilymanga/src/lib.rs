#![no_std]
use aidoku::{prelude::*, Source};
use mangabox::{Impl, MangaBox, Params};

const BASE_URL: &str = "https://lilymanga.net";

struct LilyManga;

impl Impl for LilyManga {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			..Default::default()
		}
	}
}

register_source!(
	MangaBox<LilyManga>,
	ListingProvider,
	Home,
	ImageRequestProvider,
	DeepLinkHandler
);
